pub mod models;
#[rustfmt::skip]
mod schema;

use crate::db::models::{NewUser, SessionId, UserId};
use actix::prelude::*;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;
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

#[derive(Debug)]
pub struct CreateSession {
    pub span: Span,
    pub owner_id: UserId,
    pub title: String,
    pub start_time: NaiveDateTime,
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

#[derive(Debug)]
pub struct DeleteSession {
    pub span: Span,
    pub session_id: SessionId,
    pub owner_id: UserId,
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

#[derive(Debug)]
pub struct GetOrCreateUser {
    pub span: Span,
    pub username: String,
    pub name: String,
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
                username: &user.username,
                name: &user.name,
            })
            .on_conflict(username)
            .do_update()
            .set(name.eq(&user.name))
            .get_result::<models::User>(&mut self.get_conn()?)
            .context("Failed to insert user")?;

        Ok(user)
    }
}
