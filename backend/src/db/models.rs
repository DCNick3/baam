use crate::db::schema;
use chrono::NaiveDateTime;
use derive_more::{From, Into};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, From, Into, Serialize, Deserialize)]
pub struct UserId(pub i32);
#[derive(Debug, Clone, Copy, From, Into, Serialize, Deserialize)]
pub struct SessionId(pub i32);
#[derive(Debug, Clone, Copy, From, Into, Serialize, Deserialize)]
pub struct AttendanceMarkId(pub i32);

#[derive(Debug, Clone, Queryable)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: UserId,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Queryable)]
pub struct AttendanceMark {
    #[diesel(deserialize_as = i32)]
    pub id: AttendanceMarkId,
    #[diesel(deserialize_as = i32)]
    pub user_id: UserId,
    #[diesel(deserialize_as = i32)]
    pub session_id: SessionId,
    pub mark_time: NaiveDateTime,
    pub is_manual: bool,
}

#[derive(Debug, Clone, Queryable)]
pub struct Session {
    #[diesel(deserialize_as = i32)]
    pub id: SessionId,
    pub title: String,
    #[diesel(deserialize_as = i32)]
    pub owner_id: UserId,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}
