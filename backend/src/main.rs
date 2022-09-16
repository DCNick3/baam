// linking order workaround
extern crate openssl;
#[macro_use]
extern crate diesel;

mod api;
mod config;
mod db;
mod error;

use actix::SyncArbiter;
use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};
use opentelemetry::sdk::resource::{EnvResourceDetector, SdkProvidedResourceDetector};
use opentelemetry::sdk::{trace as sdktrace, Resource};
use opentelemetry_otlp::{HasExportConfig, WithExportConfig};
use std::io::ErrorKind;
use std::time::Duration;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

fn init_tracer() -> Result<sdktrace::Tracer> {
    let mut exporter = opentelemetry_otlp::new_exporter().tonic().with_env();

    println!(
        "Using opentelemetry endpoint {}",
        exporter.export_config().endpoint
    );

    // overwrite the service name because k8s service name does not always matches what we want
    std::env::set_var("OTEL_SERVICE_NAME", "baam-backend");

    let resource = Resource::from_detectors(
        Duration::from_secs(0),
        vec![
            Box::new(EnvResourceDetector::new()),
            Box::new(SdkProvidedResourceDetector),
        ],
    );

    println!("Using opentelemetry resources {:?}", resource);

    Ok(opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(sdktrace::config().with_resource(resource))
        .install_batch(opentelemetry::runtime::Tokio)?)
}

fn init_tracing() -> Result<()> {
    let tracer = init_tracer().context("Setting up the opentelemetry exporter")?;

    Registry::default()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .event_format(tracing_subscriber::fmt::format::Format::default().compact()),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    Ok(())
}

fn get_database_url() -> String {
    if let Ok(v) = std::env::var("DATABASE_URL") {
        v
    } else {
        let (host, port, name, user, password) = (|| -> Result<_> {
            use std::env::var;
            Ok((
                var("DATABASE_HOST")?,
                var("DATABASE_PORT")?,
                var("DATABASE_NAME")?,
                var("DATABASE_USER")?,
                var("DATABASE_PASSWORD")?,
            ))
        })().expect("Please set DATABASE_URL to a valid postgres URI (for example postgres://user:password@hostname:5432/database_name)\n\
Alternatively you can set DATABASE_HOST, DATABASE_PORT, DATABASE_NAME, DATABASE_USER, DATABASE_PASSWORD variables");

        format!("postgres://{user}:{password}@{host}:{port}/{name}")
    }
}

async fn main_impl() -> Result<()> {
    init_tracing().context("Initializing tracing")?;

    let environment = std::env::var("ENVIRONMENT").context(
        "Please set ENVIRONMENT env var (probably you want to use either 'prod' or 'dev')",
    )?;

    let database_url = get_database_url();

    let config = config::Config::load(&environment).context("Loading config")?;

    let database = db::DbExecutor::new(&database_url).context("Connecting to the database")?;
    let database = SyncArbiter::start(3, move || database.clone());
    let api = api::configure().context("Configuring api")?;
    let frontend = baam_frontend::configure(config.frontend).context("Configuring frontend")?;

    println!("Starting server on http://{}/", config.server.endpoint);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(database.clone()))
            .service(web::scope("/api").configure(api.clone()))
            .configure(frontend.clone())
    })
    .bind(config.server.endpoint)?
    .run()
    .await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_impl()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
}
