use crate::db::schema;
use chrono::NaiveDateTime;
use derive_more::{From, Into};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Serialize, Deserialize)]
pub struct UserId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Serialize, Deserialize)]
pub struct SessionId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, Serialize, Deserialize)]
pub struct AttendanceMarkId(pub i32);

#[derive(Debug, Clone, Queryable)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: UserId,
    pub username: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub name: Option<&'a str>,
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

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::marks)]
pub struct NewAttendanceMark {
    #[diesel(serialize_as = i32)]
    pub user_id: UserId,
    #[diesel(serialize_as = i32)]
    pub session_id: SessionId,
    pub mark_time: NaiveDateTime,
    pub is_manual: bool,
}

#[derive(Debug, Clone, Queryable)]
pub struct Session {
    #[diesel(deserialize_as = i32)]
    pub id: SessionId,
    pub title: Option<String>,
    #[diesel(deserialize_as = i32)]
    pub owner_id: UserId,
    pub active: bool,
    pub start_time: NaiveDateTime,
    pub seed: String,
}

pub type SessionWithMarks = (
    Session,
    HashMap<AttendanceMarkId, AttendanceMark>,
    HashMap<UserId, User>,
);
