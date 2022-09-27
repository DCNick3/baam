use diesel::backend::Backend;
use diesel::connection::{
    AnsiTransactionManager, Connection, ConnectionGatWorkaround, DefaultLoadingMode,
    LoadConnection, LoadRowIter, SimpleConnection, TransactionManager,
};
use diesel::deserialize::Queryable;
use diesel::expression::QueryMetadata;
use diesel::pg::{Pg, PgConnection, TransactionBuilder};
use diesel::query_builder::{Query, QueryFragment, QueryId};
use diesel::result::{ConnectionError, ConnectionResult, QueryResult};
use diesel::RunQueryDsl;
use diesel::{select, sql_function, sql_types};
use tracing::{debug, field, instrument};

// https://www.postgresql.org/docs/12/functions-info.html
sql_function! {
    // db.name
    fn current_database() -> sql_types::Text;
}
sql_function! {
    // net.peer.ip
    fn inet_server_addr() -> sql_types::Inet;
}
sql_function! {
    // net.peer.port
    fn inet_server_port() -> sql_types::Integer;
}
sql_function! {
    // db.version
    fn version() -> sql_types::Text;
}

#[derive(Queryable, Clone, Debug, PartialEq)]
struct PgConnectionInfo {
    current_database: String,
    inet_server_addr: ipnet::IpNet,
    inet_server_port: i32,
    version: String,
}

pub struct InstrumentedPgConnection {
    inner: PgConnection,
    info: PgConnectionInfo,
}

impl SimpleConnection for InstrumentedPgConnection {
    #[instrument(
        fields(
            db.name=%self.info.current_database,
            db.system="postgresql",
            db.version=%self.info.version,
            otel.kind="client",
            net.peer.ip=%self.info.inet_server_addr,
            net.peer.port=%self.info.inet_server_port,
            db.statement=query,
        ),
        skip(self, query),
        err,
    )]
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        // TODO: we could track transaction start/savepoint/commit/rollback here and represent them as spans
        debug!("executing batch query");
        self.inner.batch_execute(query)?;

        Ok(())
    }
}

impl<'conn, 'query> ConnectionGatWorkaround<'conn, 'query, Pg, DefaultLoadingMode>
    for InstrumentedPgConnection
{
    type Cursor =
        <PgConnection as ConnectionGatWorkaround<'conn, 'query, Pg, DefaultLoadingMode>>::Cursor;
    type Row =
        <PgConnection as ConnectionGatWorkaround<'conn, 'query, Pg, DefaultLoadingMode>>::Row;
}

impl Connection for InstrumentedPgConnection {
    type Backend = Pg;
    type TransactionManager = AnsiTransactionManager;

    #[instrument(
        fields(
            db.name=field::Empty,
            db.system="postgresql",
            db.version=field::Empty,
            otel.kind="client",
            net.peer.ip=field::Empty,
            net.peer.port=field::Empty,
        ),
        skip(database_url),
        err,
    )]
    fn establish(database_url: &str) -> ConnectionResult<InstrumentedPgConnection> {
        debug!("establishing postgresql connection");
        let mut conn = PgConnection::establish(database_url)?;

        debug!("querying postgresql connection information");
        let info: PgConnectionInfo = select((
            current_database(),
            inet_server_addr(),
            inet_server_port(),
            version(),
        ))
        .get_result(&mut conn)
        .map_err(ConnectionError::CouldntSetupConfiguration)?;

        let span = tracing::Span::current();
        span.record("db.name", &info.current_database.as_str());
        span.record("db.version", &info.version.as_str());
        span.record(
            "net.peer.ip",
            &format!("{}", info.inet_server_addr).as_str(),
        );
        span.record("net.peer.port", &info.inet_server_port);

        Ok(InstrumentedPgConnection { inner: conn, info })
    }

    #[doc(hidden)]
    #[instrument(
        fields(
            db.name=%self.info.current_database,
            db.system="postgresql",
            db.version=%self.info.version,
            otel.kind="client",
            net.peer.ip=%self.info.inet_server_addr,
            net.peer.port=%self.info.inet_server_port,
            db.statement=field::Empty,
            db.operation=field::Empty,
        ),
        skip(self, source),
        err,
    )]
    fn execute_returning_count<T>(&mut self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Pg> + QueryId,
    {
        debug!("executing returning count");

        instrument_with_db_statement(&source);

        self.inner.execute_returning_count(source)
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as TransactionManager<Self>>::TransactionStateData {
        self.inner.transaction_state()
    }
}

impl InstrumentedPgConnection {
    #[instrument(
        fields(
            db.name=%self.info.current_database,
            db.system="postgresql",
            db.version=%self.info.version,
            otel.kind="client",
            net.peer.ip=%self.info.inet_server_addr,
            net.peer.port=%self.info.inet_server_port,
        ),
        skip(self),
    )]
    pub fn build_transaction(&mut self) -> TransactionBuilder<PgConnection> {
        debug!("starting transaction builder");
        self.inner.build_transaction()
    }
}

#[cfg(feature = "r2d2")]
impl diesel::r2d2::R2D2Connection for InstrumentedPgConnection {
    fn ping(&mut self) -> QueryResult<()> {
        self.inner.ping()
    }

    fn is_broken(&mut self) -> bool {
        self.inner.is_broken()
    }
}

impl diesel::migration::MigrationConnection for InstrumentedPgConnection {
    fn setup(&mut self) -> QueryResult<usize> {
        self.inner.setup()
    }
}

impl LoadConnection for InstrumentedPgConnection {
    #[instrument(
        fields(
            db.name=%self.info.current_database,
            db.system="postgresql",
            db.version=%self.info.version,
            otel.kind="client",
            net.peer.ip=%self.info.inet_server_addr,
            net.peer.port=%self.info.inet_server_port,
            db.statement=field::Empty,
            db.operation=field::Empty,
        ),
        skip(self, source),
        err
    )]
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<LoadRowIter<'conn, 'query, Self, Self::Backend, DefaultLoadingMode>>
    where
        T: Query + QueryFragment<Self::Backend> + QueryId + 'query,
        Self::Backend: QueryMetadata<T::SqlType>,
    {
        instrument_with_db_statement(&source);

        <PgConnection as LoadConnection<DefaultLoadingMode>>::load(&mut self.inner, source)
    }
}

fn instrument_with_db_statement<Q: QueryFragment<Pg>>(query: &Q) {
    let (statement, operation) = dump_query(query);
    let span = tracing::Span::current();
    span.record("db.statement", statement);
    span.record("db.operation", operation);
}

fn dump_query<Q: QueryFragment<Pg>>(query: &Q) -> (String, String) {
    use crate::diesel::query_builder::QueryBuilder;

    let mut query_builder = <Pg as Backend>::QueryBuilder::default();
    let backend = Pg::default();
    if let Err(e) = QueryFragment::<Pg>::to_sql(query, &mut query_builder, &backend) {
        // TODO: maybe there is a more graceful way to handle it?
        panic!("Failed to convert query to SQL: {}", e)
    }

    let query_str: String = query_builder.finish();

    let operation = query_str
        .as_str()
        .split_once(' ')
        .map(|(op, _)| op)
        .unwrap_or(&query_str)
        .to_string();

    (query_str, operation)
}

// impl<Changes, Output> UpdateAndFetchResults<Changes, Output> for InstrumentedPgConnection
// where
//     Changes: Copy + AsChangeset<Target = <Changes as HasTable>::Table> + IntoUpdateTarget,
//     Update<Changes, Changes>: LoadQuery<PgConnection, Output>,
// {
//     fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output> {
//         debug!("updating and fetching changeset");
//         self.inner.update_and_fetch(changeset)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_info_on_establish() {
        InstrumentedPgConnection::establish(
            &std::env::var("DATABASE_URL").expect("no postgresql env var specified"),
        )
        .expect("failed to establish connection or collect info");
    }
}
