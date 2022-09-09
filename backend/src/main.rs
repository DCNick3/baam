mod api;
mod config;

use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};
use url::Url;

async fn main_impl() -> Result<()> {
    let config = config::Config {
        frontend: baam_frontend::Config {
            upstream: None, //Some(Url::parse("http://localhost:5173").unwrap()),
        },
    };

    let api = api::configure().context("Configuring api")?;
    let frontend = baam_frontend::configure(config.frontend).context("Configuring frontend")?;

    println!("Starting server on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/").configure(api.clone()))
            .configure(frontend.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}

#[actix_web::main]
async fn main() {
    main_impl().await.unwrap();
}
