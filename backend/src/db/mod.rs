pub mod models;
#[rustfmt::skip]
mod schema;

use actix::prelude::*;
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DbData = actix_web::web::Data<Addr<DbExecutor>>;

#[derive(Clone)]
pub struct DbExecutor(Pool<ConnectionManager<PgConnection>>);

impl DbExecutor {
    pub fn new(database_url: &str) -> Result<Self> {
        let manager =
            Pool::new(ConnectionManager::new(database_url)).context("Failed to create pool")?;

        Ok(Self(manager))
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
    type Result = Result<Vec<models::Session>>;

    fn handle(&mut self, _msg: GetSessions, _ctx: &mut Self::Context) -> Self::Result {
        use schema::sessions::dsl::*;

        let mut conn = self.0.get().context("Failed to get connection from pool")?;

        let results = sessions
            .load::<models::Session>(&mut conn)
            .context("Failed to load sessions")?;

        Ok(results)
    }
}
