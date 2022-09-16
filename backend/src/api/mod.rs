mod models;

use crate::db::{DbData, GetSessions};
use crate::error::ResponseResult;
use actix_web::{get, post, web, web::ServiceConfig, HttpResponse, Responder};
use anyhow::{anyhow, Result};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! (this is a stub 6)")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/sessions")]
async fn get_sessions(db: DbData) -> ResponseResult<web::Json<Vec<models::Session>>> {
    let sessions = db.send(GetSessions).await??;

    Ok(web::Json(sessions.into_iter().map(|v| v.into()).collect()))
}

#[get("/error")]
async fn error() -> ResponseResult<web::Bytes> {
    Err(anyhow!("Example error").into())
}

pub fn configure() -> Result<impl Fn(&mut ServiceConfig) + Clone> {
    Ok(|cfg: &mut ServiceConfig| {
        cfg.service(hello)
            .service(ping)
            .service(echo)
            .service(get_sessions)
            .service(error);
    })
}
