// linking order workaround
extern crate openssl;
#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

mod api;
mod config;
mod db;
mod diagnostics;
mod middlewares;

use crate::api::AuthKeys;
use crate::middlewares::AddDiagnosticIds;
use actix::SyncArbiter;
use actix_web::middleware::ErrorHandlers;
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

    let auth_keys = AuthKeys::new([
        // TODO: replace this hard-coded key with something more secure
        0x5c, 0x6a, 0xc5, 0xf2, 0xb8, 0x12, 0xf1, 0x9d, 0x7e, 0x70, 0xd1, 0xe4, 0x9a, 0x28, 0x20,
        0xa6, 0x5b, 0xba, 0xb8, 0x9a, 0xa3, 0x76, 0x0d, 0xb0, 0x80, 0x53, 0xe4, 0x3d, 0x7a, 0x5d,
        0x27, 0x08, 0x3a, 0xb6, 0xf8, 0x28, 0xf2, 0x69, 0x04, 0x61, 0xd7, 0x05, 0xdb, 0x89, 0x1d,
        0x0d, 0xef, 0x94, 0x6e, 0xdd, 0xc2, 0x44, 0xf2, 0x92, 0xa3, 0x67, 0x71, 0x80, 0x31, 0xe5,
        0xb2, 0xcb, 0x8f, 0xc0,
    ])
    .context("Generating auth keys")?;

    let database = db::DbExecutor::new(&database_url).context("Connecting to the database")?;
    let database = SyncArbiter::start(3, move || database.clone());
    let api = api::configure(auth_keys).context("Configuring api")?;
    let frontend = baam_frontend::configure(config.frontend).context("Configuring frontend")?;

    println!("Starting server on http://{}/", config.server.endpoint);

    HttpServer::new(move || {
        App::new()
            .wrap(AddDiagnosticIds)
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/api")
                    .configure(api.clone())
                    .wrap(ErrorHandlers::new().default_handler(api::error::api_error_handler)),
            )
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
