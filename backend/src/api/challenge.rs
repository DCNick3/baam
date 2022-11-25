use crate::api::models::{Challenge, ChallengeResult};
use crate::api::ApiResult;
use crate::api::UserClaims;
use crate::db;
use crate::db::models::SessionId;
use crate::db::DbData;
use actix_web::web::{Buf, ServiceConfig};
use actix_web::{post, web};
use anyhow::bail;
use anyhow::Result;
use chrono::TimeZone;
use chrono::{DateTime, Utc};
use hmac::Mac;
use integer_encoding::VarIntReader;
use serde::Deserialize;
use sha1::Sha1;
use std::io::Read;
use std::time::Duration;
use tracing::warn;

const HMAC_SIZE: usize = 4;

#[derive(Debug, Copy, Clone)]
struct ParsedChallenge {
    hmac: [u8; HMAC_SIZE],
    session_id: SessionId,
    challenge_index: u32,
}

#[derive(Debug)]
struct ChallengeParams {
    seed: Vec<u8>,
    start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(with = "humantime_serde")]
    pub qr_interval: Duration,
    #[serde(with = "humantime_serde")]
    pub jitter_window: Duration,
}

fn parse_challenge(data: &[u8]) -> Result<ParsedChallenge> {
    let mut cursor = std::io::Cursor::new(data);
    let mut hmac = [0u8; HMAC_SIZE];
    cursor.read_exact(&mut hmac)?;
    let session_id = cursor.read_varint::<u32>()?;
    let challenge_index = cursor.read_varint::<u32>()?;

    if cursor.remaining() != 0 {
        bail!("Extra data after challenge");
    }

    Ok(ParsedChallenge {
        hmac,
        session_id: SessionId(session_id as i32),
        challenge_index,
    })
}

fn parse_encoded_challenge(data: &str) -> Result<ParsedChallenge> {
    if data.len() > 64 {
        bail!("Challenge too long");
    }
    let data = base64::decode(data.replace('-', "+").replace('_', "/"))?;
    parse_challenge(&data)
}

fn calculate_hmac(seed: &[u8], index: u32) -> Result<[u8; HMAC_SIZE]> {
    let mut hmac = hmac::SimpleHmac::<Sha1>::new_from_slice(seed)?;
    let data = index.to_le_bytes();
    hmac.update(&data);
    let result = hmac.finalize().into_bytes();
    let result = result.as_slice();
    Ok(result[..HMAC_SIZE].try_into()?)
}

/// Time subtraction with specified bounds
fn saturating_sub(
    a: DateTime<Utc>,
    b: DateTime<Utc>,
    lower_bound: Option<chrono::Duration>,
    upper_bound: Option<chrono::Duration>,
) -> chrono::Duration {
    let mut diff = a - b;
    if let Some(max) = upper_bound {
        diff = diff.min(max);
    }
    if let Some(min) = lower_bound {
        diff = diff.max(min);
    }
    diff
}

fn validate_challenge(
    challenge: ParsedChallenge,
    submission_time: DateTime<Utc>,
    params: ChallengeParams,
    config: &Config,
) -> Result<()> {
    let valid_hmac = calculate_hmac(&params.seed, challenge.challenge_index)?;
    if valid_hmac != challenge.hmac {
        bail!("Invalid hmac");
    }

    let expected_start_time = params.start_time
        + chrono::Duration::from_std(config.qr_interval * challenge.challenge_index)?;
    let expected_end_time = expected_start_time + chrono::Duration::from_std(config.qr_interval)?;

    let difference_1 = saturating_sub(
        expected_start_time,
        submission_time,
        Some(chrono::Duration::zero()),
        None,
    );
    let difference_2 = saturating_sub(
        submission_time,
        expected_end_time,
        Some(chrono::Duration::zero()),
        None,
    );
    let difference = difference_1.max(difference_2);

    if difference > chrono::Duration::from_std(config.jitter_window)? {
        bail!(
            "Challenge out of time window ({} ms > jitter window, {} ms)",
            difference.num_milliseconds(),
            config.jitter_window.as_millis(),
        );
    }

    Ok(())
}

#[post("/challenge")]
async fn submit_challenge(
    user: UserClaims,
    db: DbData,
    challenge: web::Json<Challenge>,
    config: web::Data<Config>,
) -> ApiResult<web::Json<ChallengeResult>> {
    let submission_time = Utc::now();

    let challenge = match parse_encoded_challenge(&challenge.challenge) {
        Ok(c) => c,
        Err(e) => {
            warn!(
                "User {:?} submitted unparsable challenge {:?}: {:?}",
                user.user_id, challenge.challenge, e
            );
            return Ok(web::Json(ChallengeResult::Invalid));
        }
    };

    let session = db
        .send(db::LookupSession {
            span: tracing::Span::current(),
            session_id: challenge.session_id,
        })
        .await??;
    let session = match session {
        Some(s) => s,
        None => {
            warn!(
                "User {:?} tried to submit challenge for unknown session {:?}",
                user.user_id, challenge.session_id
            );
            return Ok(web::Json(ChallengeResult::Failed));
        }
    };

    match validate_challenge(
        challenge,
        submission_time,
        ChallengeParams {
            start_time: Utc.from_utc_datetime(&session.start_time),
            seed: base64::decode(session.seed)?,
        },
        config.get_ref(),
    ) {
        Ok(()) => {}
        Err(e) => {
            warn!(
                "User {:?} submitted invalid challenge {:?}: {:?}",
                user.user_id, challenge, e
            );
            return Ok(web::Json(ChallengeResult::Failed));
        }
    }

    let (_, other_students) = db
        .send(db::AddAutoAttendanceMark {
            span: tracing::Span::current(),
            session_id: session.id,
            student_id: user.user_id,
            mark_time: submission_time.naive_utc(),
        })
        .await??;

    Ok(web::Json(ChallengeResult::Success {
        other_students: other_students.into_iter().map(|s| s.into()).collect(),
    }))
}

pub fn configure(config: Config) -> impl Fn(&mut ServiceConfig) + Clone {
    move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(config.clone()))
            .service(submit_challenge);
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use crate::db::models::SessionId;
    use std::ops::{Add, Sub};
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_parse_challenge() {
        let data = base64::decode("AAAAAAEC").unwrap();
        let parsed = parse_challenge(&data).unwrap();
        assert_eq!(parsed.hmac, [0, 0, 0, 0]);
        assert_eq!(parsed.session_id.0, 1);
        assert_eq!(parsed.challenge_index, 2);
    }

    #[test]
    fn test_parse_encoded_challenge() {
        let data = "PQRETQwE";
        let parsed = parse_encoded_challenge(data).unwrap();
        assert_eq!(parsed.hmac, [0x3d, 0x04, 0x44, 0x4d]);
        assert_eq!(parsed.session_id.0, 12);
        assert_eq!(parsed.challenge_index, 4);
    }

    fn init() -> (Vec<u8>, ParsedChallenge, DateTime<Utc>, Config) {
        let seed = base64::decode("YNxExINfvxmC0q6g").unwrap();
        let parsed_challenge = ParsedChallenge {
            hmac: [48, 137, 117, 180],
            session_id: SessionId(12),
            challenge_index: 4,
        };
        let start_time = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(1645671600, 0),
            chrono::Utc,
        );
        let config = super::Config {
            qr_interval: Duration::from_secs(1),
            jitter_window: Duration::from_millis(300),
        };
        (seed, parsed_challenge, start_time, config)
    }

    #[test]
    fn test_validate_challenge_valid() {
        let (seed, parsed_challenge, start_time, config) = init();
        let jitter_window = chrono::Duration::from_std(config.jitter_window).unwrap();
        let cases = [
            // Window start
            start_time.add(chrono::Duration::seconds(4)),
            // Window center
            start_time.add(chrono::Duration::milliseconds(4500)),
            // Window end
            start_time.add(chrono::Duration::seconds(5)),
            // Window start and jitter window
            start_time.add(chrono::Duration::seconds(4).sub(jitter_window)),
            // Window end and jitter window
            start_time.add(chrono::Duration::seconds(5).add(jitter_window)),
        ];
        for submission_time in cases {
            validate_challenge(
                parsed_challenge,
                submission_time,
                super::ChallengeParams {
                    start_time,
                    seed: seed.clone(),
                },
                &config,
            )
            .expect(&format!(
                "Challenge at timestamp {} should be accepted",
                submission_time
            ));
        }
    }

    #[test]
    fn test_validate_challenge_invalid() {
        let (seed, parsed_challenge, start_time, config) = init();

        // Out-of-bounds submisison times

        // Slightly out of bounds to fail
        let jitter_window_plus_1 = chrono::Duration::from_std(config.jitter_window).unwrap()
            + chrono::Duration::milliseconds(1);
        let cases = [
            // Window start and jitter window
            start_time.add(chrono::Duration::seconds(4).sub(jitter_window_plus_1)),
            // Window end and jitter window
            start_time.add(chrono::Duration::seconds(5).add(jitter_window_plus_1)),
            // Overflow/underflow does not crash anything
            chrono::DateTime::<chrono::Utc>::MIN_UTC,
            chrono::DateTime::<chrono::Utc>::MAX_UTC,
        ];
        for submission_time in cases {
            validate_challenge(
                parsed_challenge,
                submission_time,
                super::ChallengeParams {
                    start_time,
                    seed: seed.clone(),
                },
                &config,
            )
            .expect_err(&format!(
                "Challenge at timestamp {} should not be accepted",
                submission_time
            ));
        }

        let submission_time = start_time.add(chrono::Duration::seconds(4));

        // Test if this passes to make sure errors later are caused by the changes made
        validate_challenge(
            parsed_challenge,
            submission_time,
            super::ChallengeParams {
                start_time,
                seed: seed.clone(),
            },
            &config,
        )
        .expect(&format!(
            "Challenge {:?} should be accepted",
            parsed_challenge
        ));

        // Wrong HMAC
        let incorrect_challenge = ParsedChallenge {
            hmac: [48, 137, 117, 181],
            ..parsed_challenge
        };
        validate_challenge(
            incorrect_challenge,
            submission_time,
            super::ChallengeParams {
                start_time,
                seed: seed.clone(),
            },
            &config,
        )
        .expect_err(&format!(
            "Challenge {:?} should not be accepted",
            incorrect_challenge
        ));

        // Wrong index (or HMAC lol)
        let incorrect_challenge = ParsedChallenge {
            challenge_index: 3,
            ..parsed_challenge
        };
        validate_challenge(
            incorrect_challenge,
            submission_time,
            super::ChallengeParams {
                start_time,
                seed: seed.clone(),
            },
            &config,
        )
        .expect_err(&format!(
            "Challenge {:?} should not be accepted",
            incorrect_challenge
        ));
    }
}
