use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct AttendanceMark {
    pub id: i32,
    pub user_id: i32,
    pub session_id: i32,
    pub mark_time: NaiveDateTime,
    pub is_manual: bool,
}

#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub owner_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}
