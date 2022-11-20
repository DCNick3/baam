pub mod models;
#[rustfmt::skip]
mod schema;

use crate::api::error::{ApiError, ApiResult};
use crate::db::models::{NewAttendanceMark, NewUser, SessionId, UserId};
use actix::prelude::*;
use actix_http::StatusCode;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::Connection as DieselConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel_tracing::pg::InstrumentedPgConnection;
use r2d2::PooledConnection;
use std::collections::HashMap;
use tracing::{info, instrument, Span};

#[derive(Debug)]
pub enum DbError {
    SessionNotFound,
    MarkNotFound,
}

impl ApiError for DbError {
    fn to_http(&self) -> (StatusCode, String) {
        match self {
            DbError::SessionNotFound => (StatusCode::NOT_FOUND, "Session not found".to_string()),
            DbError::MarkNotFound => (StatusCode::NOT_FOUND, "Mark not found".to_string()),
        }
    }
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbData = actix_web::web::Data<Addr<DbExecutor>>;

type Pool = r2d2::Pool<ConnectionManager<InstrumentedPgConnection>>;
type Connection = PooledConnection<ConnectionManager<InstrumentedPgConnection>>;

#[derive(Clone)]
pub struct DbExecutor(Pool);

impl DbExecutor {
    #[instrument(skip(database_url))]
    pub fn new(database_url: &str) -> Result<Self> {
        let pool =
            Pool::new(ConnectionManager::new(database_url)).context("Failed to create pool")?;

        let mut conn = pool.get().context("Failed to get connection from pool")?;
        if MigrationHarness::has_pending_migration(&mut conn, MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Failed to check for pending migrations: {}", e))?
        {
            info!("Applying migrations");
            MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS)
                .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
            info!("All pending locations applied");
        } else {
            info!("Database is up to date, no migrations needed");
        }

        Ok(Self(pool))
    }

    fn get_conn(&mut self) -> Result<Connection> {
        self.0.get().context("Failed to get connection from pool")
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub struct GetSessions {
    pub span: Span,
    pub user_id: UserId,
}
/// Get session, checking its owner
#[derive(Debug)]
pub struct GetSession {
    pub span: Span,
    pub owner_id: UserId,
    pub session_id: SessionId,
}
/// Just get session, without checking its owner
#[derive(Debug)]
pub struct LookupSession {
    pub span: Span,
    pub session_id: SessionId,
}
#[derive(Debug)]
pub struct CreateSession {
    pub span: Span,
    pub owner_id: UserId,
    pub title: Option<String>,
    pub start_time: NaiveDateTime,
    pub seed: String,
}
#[derive(Debug)]
pub struct DeleteSession {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
}
#[derive(Debug)]
pub struct AddManualAttendanceMark {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
    pub student_username: String,
    pub mark_time: NaiveDateTime,
}
#[derive(Debug)]
pub struct AddAutoAttendanceMark {
    pub span: Span,
    pub session_id: SessionId,
    pub student_id: UserId,
    pub mark_time: NaiveDateTime,
}
#[derive(Debug)]
pub struct DeleteAttendanceMark {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
    pub student_username: String,
}
#[derive(Debug)]
pub struct GetOrCreateUser {
    pub span: Span,
    pub username: String,
    pub name: String,
}

fn get_or_create_user(
    conn: &mut Connection,
    username_: &str,
    name_: Option<&str>,
) -> ApiResult<models::User> {
    use schema::users::dsl::*;
    let user: models::User = {
        users
            .filter(username.eq(username_))
            .first::<models::User>(conn)
            .optional()
            .context("Failed to load session")
    }
    .transpose()
    .unwrap_or_else(|| {
        diesel::insert_into(users)
            .values(&NewUser {
                username: username_,
                name: name_,
            })
            .on_conflict(username)
            .do_update()
            .set(name.eq(None as Option<&str>))
            .get_result::<models::User>(conn)
            .context("Failed to insert user")
    })?;

    Ok(user)
}

fn get_session(
    conn: &mut Connection,
    session_id_: SessionId,
    owner_id_: UserId,
) -> ApiResult<models::Session> {
    use schema::sessions::dsl::*;
    sessions
        .filter(id.eq(&session_id_.0))
        .filter(owner_id.eq(&owner_id_.0))
        .first(conn)
        .optional()
        .context("Failed to load session")?
        .ok_or_else(|| DbError::SessionNotFound.into())
}

impl Message for GetSessions {
    type Result = ApiResult<Vec<models::Session>>;
}
impl Handler<GetSessions> for DbExecutor {
    type Result = <GetSessions as Message>::Result;

    #[instrument(name = "GetSessions", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: GetSessions, _: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let results = sessions
            .filter(owner_id.eq(&msg.user_id.0))
            .load::<models::Session>(&mut self.get_conn()?)
            .context("Failed to load sessions")?;

        Ok(results)
    }
}

impl Message for GetSession {
    type Result = ApiResult<models::SessionWithMarks>;
}
impl Handler<GetSession> for DbExecutor {
    type Result = <GetSession as Message>::Result;

    #[instrument(name = "GetSession", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: GetSession, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> ApiResult<_> {
            let session = get_session(conn, msg.session_id, msg.owner_id)?;

            let marks: Vec<models::AttendanceMark> = {
                use schema::marks::dsl::*;
                marks
                    .filter(session_id.eq(&msg.session_id.0))
                    .load(conn)
                    .context("Failed to load marks")?
            };

            let mark_user_ids = marks.iter().map(|m| m.user_id.0).collect::<Vec<_>>();

            let users: Vec<models::User> = {
                use schema::users::dsl::*;
                users
                    .filter(id.eq_any(mark_user_ids))
                    .load(conn)
                    .context("Failed to load users")?
            };

            let marks = marks
                .into_iter()
                .map(|m| (m.id, m))
                .collect::<HashMap<_, _>>();

            let users = users
                .into_iter()
                .map(|u| (u.id, u))
                .collect::<HashMap<_, _>>();

            Ok((session, marks, users))
        })
    }
}

impl Message for LookupSession {
    type Result = ApiResult<Option<models::Session>>;
}
impl Handler<LookupSession> for DbExecutor {
    type Result = <LookupSession as Message>::Result;

    #[instrument(name = "LookupSession", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: LookupSession, _: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let result = sessions
            .filter(id.eq(&msg.session_id.0))
            .first::<models::Session>(&mut self.get_conn()?)
            .optional()
            .context("Failed to load session")?;

        Ok(result)
    }
}

impl Message for CreateSession {
    type Result = ApiResult<models::Session>;
}
impl Handler<CreateSession> for DbExecutor {
    type Result = <CreateSession as Message>::Result;

    #[instrument(name = "GetOrCreateUser", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: CreateSession, _: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let result = diesel::insert_into(sessions)
            .values((
                owner_id.eq(&msg.owner_id.0),
                title.eq(&msg.title),
                start_time.eq(&msg.start_time),
                seed.eq(&msg.seed),
            ))
            .get_result::<models::Session>(&mut self.get_conn()?)
            .context("Failed to create session")?;

        Ok(result)
    }
}

impl Message for DeleteSession {
    type Result = ApiResult<models::Session>;
}
impl Handler<DeleteSession> for DbExecutor {
    type Result = <DeleteSession as Message>::Result;

    #[instrument(name = "DeleteSession", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: DeleteSession, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> ApiResult<_> {
            {
                use schema::marks::dsl::*;
                diesel::delete(marks.filter(session_id.eq(&msg.session_id.0)))
                    .execute(conn)
                    .context("Failed to delete session marks")?;
            }
            {
                use schema::sessions::dsl::*;

                diesel::delete(
                    sessions.filter(id.eq(&msg.session_id.0).and(owner_id.eq(&msg.owner_id.0))),
                )
                .get_result::<models::Session>(conn)
                .optional()
                .context("Failed to delete session")?
            }
            .ok_or_else(|| DbError::SessionNotFound.into())
        })
    }
}

impl Message for AddManualAttendanceMark {
    type Result = ApiResult<models::AttendanceMark>;
}
impl Handler<AddManualAttendanceMark> for DbExecutor {
    type Result = <AddManualAttendanceMark as Message>::Result;

    #[instrument(name = "AddManualAttendanceMark", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: AddManualAttendanceMark, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> ApiResult<_> {
            let user = get_or_create_user(conn, &msg.student_username, None)?;

            // check that the session is owned by the supplied owner_id
            let _session = get_session(conn, msg.session_id, msg.owner_id)?;

            use schema::marks::dsl::*;
            Ok(diesel::insert_into(marks)
                .values(NewAttendanceMark {
                    session_id: msg.session_id,
                    user_id: user.id,
                    mark_time: msg.mark_time,
                    is_manual: true,
                })
                .on_conflict((session_id, user_id))
                .do_nothing()
                .get_result(conn)
                .optional()
                .context("Failed to insert mark")
                .transpose()
                .unwrap_or_else(|| {
                    marks
                        .filter(session_id.eq(&msg.session_id.0))
                        .filter(user_id.eq(&user.id.0))
                        .first(conn)
                        .context("Failed to load an already existing mark")
                })?)
        })
    }
}

impl Message for AddAutoAttendanceMark {
    type Result = ApiResult<(models::AttendanceMark, Vec<models::User>)>;
}
impl Handler<AddAutoAttendanceMark> for DbExecutor {
    type Result = <AddAutoAttendanceMark as Message>::Result;

    #[instrument(name = "AddAutoAttendanceMark", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: AddAutoAttendanceMark, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> ApiResult<_> {
            use schema::marks::dsl::*;
            let mark = diesel::insert_into(marks)
                .values(NewAttendanceMark {
                    session_id: msg.session_id,
                    user_id: msg.student_id,
                    mark_time: msg.mark_time,
                    is_manual: false,
                })
                .on_conflict((session_id, user_id))
                .do_nothing()
                .get_result(conn)
                .optional()
                .context("Failed to insert mark")
                .transpose()
                .unwrap_or_else(|| {
                    marks
                        .filter(session_id.eq(&msg.session_id.0))
                        .filter(user_id.eq(&msg.student_id.0))
                        .first(conn)
                        .context("Failed to load an already existing mark")
                })?;

            Ok((
                mark,
                vec![], // TODO: return the list of students who were marked before this one
            ))
        })
    }
}

impl Message for DeleteAttendanceMark {
    type Result = ApiResult<models::AttendanceMark>;
}
impl Handler<DeleteAttendanceMark> for DbExecutor {
    type Result = <DeleteAttendanceMark as Message>::Result;

    #[instrument(name = "DeleteAttendanceMark", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: DeleteAttendanceMark, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> ApiResult<_> {
            let user = get_or_create_user(conn, &msg.student_username, None)?;

            // check that the session is owned by the supplied owner_id
            let _session = get_session(conn, msg.session_id, msg.owner_id)?;

            use schema::marks::dsl::*;

            Ok(diesel::delete(
                marks
                    .filter(session_id.eq(&msg.session_id.0))
                    .filter(user_id.eq(&user.id.0)),
            )
            .get_result(conn)
            .optional()
            .context("Failed to insert mark")?
            .ok_or(DbError::MarkNotFound)?)
        })
    }
}

impl Message for GetOrCreateUser {
    type Result = ApiResult<models::User>;
}
impl Handler<GetOrCreateUser> for DbExecutor {
    type Result = <GetOrCreateUser as Message>::Result;

    #[instrument(name = "GetOrCreateUser", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: GetOrCreateUser, _: &mut Self::Context) -> Self::Result {
        let user = get_or_create_user(&mut self.get_conn()?, &msg.username, Some(&msg.name))?;

        Ok(user)
    }
}
