pub mod models;
#[rustfmt::skip]
mod schema;

use crate::db::models::{NewAttendanceMark, NewUser, SessionId, UserId};
use actix::prelude::*;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection as DieselConnection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;
use std::collections::HashMap;
use tracing::{info, instrument, Span};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbData = actix_web::web::Data<Addr<DbExecutor>>;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DbExecutor(Pool);

impl DbExecutor {
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
#[derive(Debug)]
pub struct GetSession {
    pub span: Span,
    pub owner_id: UserId,
    pub session_id: SessionId,
}
#[derive(Debug)]
pub struct CreateSession {
    pub span: Span,
    pub owner_id: UserId,
    pub title: String,
    pub start_time: NaiveDateTime,
}
#[derive(Debug)]
pub struct DeleteSession {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
}
#[derive(Debug)]
pub struct AddAttendanceMark {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
    pub student_username: String,
    /// Is this a manual attendance mark or a check-in by a student?
    pub is_manual: bool,
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

impl Message for GetSessions {
    type Result = Result<Vec<models::Session>>;
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
    type Result = Result<Option<models::SessionWithMarks>>;
}
impl Handler<GetSession> for DbExecutor {
    type Result = <GetSession as Message>::Result;

    #[instrument(name = "GetSession", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: GetSession, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> Result<_> {
            let session: Option<models::Session> = {
                use schema::sessions::dsl::*;
                sessions
                    .filter(id.eq(&msg.session_id.0))
                    .filter(owner_id.eq(&msg.owner_id.0))
                    .first(conn)
                    .optional()
                    .context("Failed to load session")?
            };

            Ok(if let Some(session) = session {
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

                Some((session, marks, users))
            } else {
                None
            })
        })
    }
}

impl Message for CreateSession {
    type Result = Result<models::Session>;
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
            ))
            .get_result::<models::Session>(&mut self.get_conn()?)
            .context("Failed to create session")?;

        Ok(result)
    }
}

impl Message for DeleteSession {
    type Result = Result<Option<models::Session>>;
}
impl Handler<DeleteSession> for DbExecutor {
    type Result = <DeleteSession as Message>::Result;

    #[instrument(name = "DeleteSession", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: DeleteSession, _: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let result = diesel::delete(
            sessions.filter(id.eq(&msg.session_id.0).and(owner_id.eq(&msg.owner_id.0))),
        )
        .get_result::<models::Session>(&mut self.get_conn()?)
        .optional()
        .context("Failed to delete session")?;

        Ok(result)
    }
}

impl Message for AddAttendanceMark {
    type Result = Result<Option<models::AttendanceMark>>;
}
impl Handler<AddAttendanceMark> for DbExecutor {
    type Result = <AddAttendanceMark as Message>::Result;

    #[instrument(name = "AddAttendanceMark", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: AddAttendanceMark, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> Result<_> {
            let user: models::User = {
                use schema::users::dsl::*;
                users
                    .filter(username.eq(&msg.student_username))
                    .first::<models::User>(conn)
                    .optional()
                    .context("Failed to load session")
            }
            .transpose()
            .unwrap_or_else(|| {
                use schema::users::dsl::*;
                diesel::insert_into(users)
                    .values(&NewUser {
                        username: msg.student_username.as_str(),
                        name: None,
                    })
                    .on_conflict(username)
                    .do_update()
                    .set(name.eq(None as Option<&str>))
                    .get_result::<models::User>(&mut self.get_conn()?)
                    .context("Failed to insert user")
            })?;

            // check that the session is owned by the supplied owner_id
            if {
                use schema::sessions::dsl::*;
                sessions
                    .filter(id.eq(&msg.session_id.0))
                    .filter(owner_id.eq(&msg.owner_id.0))
                    .first::<models::Session>(conn)
                    .optional()
                    .context("Failed to load session")?
            }
            .is_none()
            {
                return Ok(None);
            }

            use schema::marks::dsl::*;
            Ok(Some(
                diesel::insert_into(marks)
                    .values(NewAttendanceMark {
                        session_id: msg.session_id,
                        user_id: user.id,
                        mark_time: msg.mark_time,
                        is_manual: msg.is_manual,
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
                    })?,
            ))
        })
    }
}

impl Message for DeleteAttendanceMark {
    type Result = Result<Option<models::AttendanceMark>>;
}
impl Handler<DeleteAttendanceMark> for DbExecutor {
    type Result = <DeleteAttendanceMark as Message>::Result;

    #[instrument(name = "DeleteAttendanceMark", parent = &msg.span, skip(self))]
    fn handle(&mut self, msg: DeleteAttendanceMark, _: &mut Self::Context) -> Self::Result {
        self.get_conn()?.transaction(|conn| -> Result<_> {
            // TODO: extract
            let user: models::User = {
                use schema::users::dsl::*;
                users
                    .filter(username.eq(&msg.student_username))
                    .first::<models::User>(conn)
                    .optional()
                    .context("Failed to load session")
            }
            .transpose()
            .unwrap_or_else(|| {
                use schema::users::dsl::*;
                diesel::insert_into(users)
                    .values(&NewUser {
                        username: msg.student_username.as_str(),
                        name: None,
                    })
                    .on_conflict(username)
                    .do_update()
                    .set(name.eq(None as Option<&str>))
                    .get_result::<models::User>(&mut self.get_conn()?)
                    .context("Failed to insert user")
            })?;

            // check that the session is owned by the supplied owner_id
            if {
                use schema::sessions::dsl::*;
                sessions
                    .filter(id.eq(&msg.session_id.0))
                    .filter(owner_id.eq(&msg.owner_id.0))
                    .first::<models::Session>(conn)
                    .optional()
                    .context("Failed to load session")?
            }
            .is_none()
            {
                return Ok(None);
            }

            use schema::marks::dsl::*;
            diesel::delete(marks)
                .filter(session_id.eq(&msg.session_id.0))
                .filter(user_id.eq(&user.id.0))
                .get_result(conn)
                .optional()
                .context("Failed to insert mark")
        })
    }
}

impl Message for GetOrCreateUser {
    type Result = Result<models::User>;
}
impl Handler<GetOrCreateUser> for DbExecutor {
    type Result = <GetOrCreateUser as Message>::Result;

    #[instrument(name = "GetOrCreateUser", parent = &user.span, skip(self))]
    fn handle(&mut self, user: GetOrCreateUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;

        let user = diesel::insert_into(users)
            .values(&NewUser {
                username: user.username.as_str(),
                name: Some(user.name.as_str()),
            })
            .on_conflict(username)
            .do_update()
            .set(name.eq(&user.name))
            .get_result::<models::User>(&mut self.get_conn()?)
            .context("Failed to insert user")?;

        Ok(user)
    }
}
