use crate::api::auth::UserClaims;
use crate::api::error::{ApiError, ApiResult};
use crate::api::models;
use crate::db;
use crate::db::DbData;
use actix_http::StatusCode;
use actix_web::{delete, get, post, put, web};
use chrono::TimeZone;
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

#[get("/sessions/{session_id}")]
async fn get_session(
    user: UserClaims,
    db: DbData,
    req: web::Path<models::GetSession>,
) -> ApiResult<web::Json<models::SessionWithMarks>> {
    let req = req.into_inner();
    let session = db
        .send(db::GetSession {
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

#[put("/sessions/{session_id}/marks/{username}")]
async fn add_mark(
    user: UserClaims,
    db: DbData,
    req: web::Path<models::AddAttendanceMark>,
) -> ApiResult<web::Json<models::AttendanceMark>> {
    let time = Utc::now();

    let req = req.into_inner();
    let mark: Option<db::models::AttendanceMark> = db
        .send(db::AddAttendanceMark {
            span: Span::current(),
            owner_id: user.user_id,
            session_id: req.session_id,
            student_username: req.username.clone(),
            mark_time: time.naive_utc(),
            is_manual: true,
        })
        .await??;

    if let Some(mark) = mark {
        Ok(web::Json(models::AttendanceMark {
            username: req.username,
            mark_time: Utc.from_utc_datetime(&mark.mark_time),
            is_manual: mark.is_manual,
        }))
    } else {
        ApiResult::Err(SessionNotFoundError.into())
    }
}
