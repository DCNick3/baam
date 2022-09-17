pub mod models;
#[rustfmt::skip]
mod schema;

use actix::prelude::*;
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;
use tracing::info;

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

pub struct GetSessions;
impl Message for GetSessions {
    type Result = Result<Vec<models::Session>>;
}

impl Handler<GetSessions> for DbExecutor {
    type Result = <GetSessions as Message>::Result;

    fn handle(&mut self, _msg: GetSessions, _: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let mut conn = self.0.get().context("Failed to get connection from pool")?;

        let results = sessions
            .load::<models::Session>(&mut conn)
            .context("Failed to load sessions")?;

        Ok(results)
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct GetOrCreateUser {
    pub username: String,
    pub name: String,
}
impl Message for GetOrCreateUser {
    type Result = Result<models::User>;
}

impl Handler<GetOrCreateUser> for DbExecutor {
    type Result = <GetOrCreateUser as Message>::Result;

    fn handle(&mut self, user: GetOrCreateUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;

        let user = diesel::insert_into(users)
            .values(&user)
            .on_conflict(username)
            .do_update()
            .set(name.eq(&user.name))
            .get_result::<models::User>(&mut self.get_conn()?)
            .context("Failed to insert user")?;

        Ok(user)
    }
}
