use crate::db::models as db_models;
use crate::db::models::SessionId;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl From<db_models::Session> for Session {
    fn from(db_session: db_models::Session) -> Self {
        Self {
            id: db_session.id,
            title: db_session.title,
            start_time: Utc.from_utc_datetime(&db_session.start_time),
            end_time: db_session.end_time.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewSession {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSession {
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSession {
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize)]
pub struct SessionWithMarks {
    pub id: SessionId,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub attendance_marks: Vec<AttendanceMark>,
}

impl From<db_models::SessionWithMarks> for SessionWithMarks {
    fn from((session, marks, users): db_models::SessionWithMarks) -> Self {
        Self {
            id: session.id,
            title: session.title,
            start_time: Utc.from_utc_datetime(&session.start_time),
            end_time: session.end_time.map(|dt| Utc.from_utc_datetime(&dt)),
            attendance_marks: marks
                .into_iter()
                .map(|(_, mark)| AttendanceMark {
                    username: users.get(&mark.user_id).unwrap().username.clone(),
                    mark_time: Utc.from_utc_datetime(&mark.mark_time),
                    is_manual: mark.is_manual,
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AttendanceMark {
    pub username: String,
    pub mark_time: DateTime<Utc>,
    pub is_manual: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AttendanceMarkRef {
    pub session_id: SessionId,
    pub username: String,
}

/// This is a login request used only for testing
/// It should not be available in production
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
}
