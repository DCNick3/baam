use crate::db::models as db_models;
use crate::db::models::SessionId;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Session as seen in the listing
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub title: Option<String>,
    pub active: bool,
    pub start_time: DateTime<Utc>,
}

impl From<db_models::Session> for Session {
    fn from(db_session: db_models::Session) -> Self {
        Self {
            id: db_session.id,
            title: db_session.title,
            active: db_session.active,
            start_time: Utc.from_utc_datetime(&db_session.start_time),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewSession {
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSession {
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSession {
    pub session_id: SessionId,
}

/// Session as seen on attendance check page
#[derive(Serialize, Deserialize)]
pub struct SessionWithMarks {
    pub id: SessionId,
    pub title: Option<String>,
    pub active: bool,
    pub start_time: DateTime<Utc>,
    pub seed: String,
    pub attendance_marks: Vec<AttendanceMark>,
}

impl From<db_models::SessionWithMarks> for SessionWithMarks {
    fn from((session, marks, users): db_models::SessionWithMarks) -> Self {
        Self {
            id: session.id,
            title: session.title,
            active: session.active,
            start_time: Utc.from_utc_datetime(&session.start_time),
            seed: session.seed,
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
    pub name: Option<String>,
}

impl From<db_models::User> for User {
    fn from(db_user: db_models::User) -> Self {
        Self {
            username: db_user.username,
            name: db_user.name,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    /// Base-64 encoded challenge
    pub challenge: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum ChallengeResult {
    Success { other_students: Vec<User> },
    Invalid,
    Failed,
}
