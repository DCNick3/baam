use crate::db::models as db_models;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl From<db_models::Session> for Session {
    fn from(db_session: db_models::Session) -> Self {
        Self {
            id: db_session.id.0,
            title: db_session.title,
            start_time: db_session.start_time.and_local_timezone(Utc).unwrap(),
            end_time: db_session
                .end_time
                .map(|t| t.and_local_timezone(Utc).unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionWithMarks {
    pub id: i32,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub attendance_marks: Vec<AttendanceMark>,
}

#[derive(Serialize, Deserialize)]
pub struct AttendanceMark {
    pub user_id: i32,
    pub username: String,
    pub mark_time: DateTime<Utc>,
    pub is_manual: bool,
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
