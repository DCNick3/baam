use crate::api::models::{Challenge, ChallengeResult};
use crate::api::UserClaims;
use crate::db::DbData;
use actix_web::{post, web};

#[post("/challenge")]
pub async fn submit_challenge(
    user: UserClaims,
    db: DbData,
    challenge: web::Json<Challenge>,
) -> web::Json<ChallengeResult> {
    web::Json(ChallengeResult::Failure)
}
