mod models;
mod sso;

use crate::api::models::LoginRequest;
use crate::db::models as db_models;
use crate::db::{DbData, GetOrCreateUser, GetSessions};
use crate::error::ResponseResult;
use actix_web::http::StatusCode;
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

#[post("/login")]
async fn login(db: DbData, body: web::Json<LoginRequest>) -> ResponseResult<HttpResponse> {
    let body = body.into_inner();

    let user: db_models::User = db
        .send(GetOrCreateUser {
            username: body.username,
            name: body.name,
        })
        .await??;

    Ok(HttpResponse::build(StatusCode::OK)
        // .cookie(
        //     Cookie::build("user_id", user.id.0.to_string())
        //         .path("/")
        //         .http_only(true)
        //         .finish(),
        // )
        .body(format!("{:#?}", user)))
}

pub fn configure() -> Result<impl Fn(&mut ServiceConfig) + Clone> {
    Ok(|cfg: &mut ServiceConfig| {
        cfg.service(hello)
            .service(ping)
            .service(echo)
            .service(get_sessions)
            .service(error)
            .service(login);
    })
}
