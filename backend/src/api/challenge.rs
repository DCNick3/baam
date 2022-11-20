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
use integer_encoding::{VarInt, VarIntReader};
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
    let data = base64::decode(data)?;
    parse_challenge(&data)
}

fn calculate_hmac(seed: &[u8], index: u32) -> Result<[u8; HMAC_SIZE]> {
    let mut hmac = hmac::SimpleHmac::<Sha1>::new_from_slice(seed)?;
    hmac.update(&index.encode_var_vec());
    let result = hmac.finalize().into_bytes();
    Ok(result[..HMAC_SIZE].try_into()?)
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

    let difference_1 = submission_time - expected_start_time;
    let difference_2 = expected_end_time - submission_time;
    let difference = difference_1.min(difference_2).max(chrono::Duration::zero());

    if difference > chrono::Duration::from_std(config.jitter_window)? {
        bail!(
            "Challenge out of time window ({} ms)",
            difference.num_milliseconds()
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
    use crate::api::challenge::{parse_challenge, validate_challenge};
    use std::time::Duration;

    #[test]
    fn test_parse_challenge() {
        let data = base64::decode("AAAAAAEC").unwrap();
        let parsed = super::parse_challenge(&data).unwrap();
        assert_eq!(parsed.hmac, [0, 0, 0, 0]);
        assert_eq!(parsed.session_id.0, 1);
        assert_eq!(parsed.challenge_index, 2);
    }

    #[test]
    fn test_parse_encoded_challenge() {
        let data = "PQRETQwE";
        let parsed = super::parse_encoded_challenge(data).unwrap();
        assert_eq!(parsed.hmac, [0x3d, 0x04, 0x44, 0x4d]);
        assert_eq!(parsed.session_id.0, 12);
        assert_eq!(parsed.challenge_index, 4);
    }

    #[test]
    fn test_validate_challenge() {
        let seed = base64::decode("YNxExINfvxmC0q6g").unwrap();
        let parsed_challenge = parse_challenge(&base64::decode("PQRETQwE").unwrap()).unwrap();
        validate_challenge(
            parsed_challenge,
            chrono::Utc::now(),
            super::ChallengeParams {
                start_time: chrono::Utc::now(),
                seed,
            },
            &super::Config {
                qr_interval: Duration::from_secs(1),
                jitter_window: Duration::from_millis(500),
            },
        )
        .unwrap();
    }
}
