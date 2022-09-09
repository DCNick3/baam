mod api;
mod config;

use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};
use std::io::ErrorKind;

async fn main_impl() -> Result<()> {
    let environment = std::env::var("ENVIRONMENT").context(
        "Please set ENVIRONMENT env var (probably you want to use either 'prod' or 'dev')",
    )?;

    let config = config::Config::load(&environment).context("Loading config")?;

    let api = api::configure().context("Configuring api")?;
    let frontend = baam_frontend::configure(config.frontend).context("Configuring frontend")?;

    println!("Starting server on http://{}/", config.server.endpoint);

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/").configure(api.clone()))
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
