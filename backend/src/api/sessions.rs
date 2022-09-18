use crate::api::auth::UserClaims;
use crate::api::error::{ApiError, ApiResult};
use crate::api::models;
use crate::db;
use crate::db::DbData;
use actix_http::StatusCode;
use actix_web::{delete, get, post, web};
use chrono::Utc;
use tracing::Span;

#[derive(Debug)]
pub struct SessionNotFoundError;

impl ApiError for SessionNotFoundError {
    fn to_http(&self) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, "Session not found".to_string())
    }
}

#[get("/sessions")]
async fn get_sessions(user: UserClaims, db: DbData) -> ApiResult<web::Json<Vec<models::Session>>> {
    let sessions = db
        .send(db::GetSessions {
            span: Span::current(),
            user_id: user.user_id,
        })
        .await??;

    Ok(web::Json(sessions.into_iter().map(|v| v.into()).collect()))
}

#[post("/sessions")]
async fn create_session(
    user: UserClaims,
    db: DbData,
    req: web::Json<models::NewSession>,
) -> ApiResult<web::Json<models::Session>> {
    let req = req.into_inner();
    let session = db
        .send(db::CreateSession {
            span: Span::current(),
            owner_id: user.user_id,
            title: req.title,
            start_time: Utc::now().naive_utc(),
        })
        .await??;

    Ok(web::Json(session.into()))
}

#[delete("/sessions/{session_id}")]
async fn delete_session(
    user: UserClaims,
    db: DbData,
    req: web::Path<models::DeleteSession>,
) -> ApiResult<web::Json<models::Session>> {
    let req = req.into_inner();
    let session = db
        .send(db::DeleteSession {
            span: Span::current(),
            owner_id: user.user_id,
            session_id: req.session_id,
        })
        .await??;

    if let Some(session) = session {
        Ok(web::Json(session.into()))
    } else {
        ApiResult::Err(SessionNotFoundError.into())
    }
}
