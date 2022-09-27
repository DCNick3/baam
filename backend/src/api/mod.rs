pub mod auth;
pub mod error;
pub mod models;
mod sessions;
mod sso;

use crate::api::models::LoginRequest;
use crate::db::models as db_models;
use crate::db::DbData;
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, web::ServiceConfig, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use error::ApiResult;
use std::collections::HashMap;
use tracing::Span;

use crate::api::auth::UserClaims;
use crate::db;
pub use auth::AuthKeys;

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

#[get("/error")]
async fn make_error() -> ApiResult<web::Bytes> {
    Err(anyhow!("Example error").into())
}

#[post("/login")]
async fn login(
    db: DbData,
    authority: web::Data<auth::Authority>,
    body: web::Json<LoginRequest>,
) -> ApiResult<HttpResponse> {
    let body = body.into_inner();

    let user: db_models::User = db
        .send(db::GetOrCreateUser {
            span: Span::current(),
            username: body.username,
            name: body.name,
        })
        .await??;

    let mut cookie: Cookie = authority
        .create_signed_cookie(user.into())
        .map_err(|e| anyhow!("Cookie creation failed: {:?}", e))?;

    cookie.set_http_only(true);
    cookie.set_same_site(actix_web::cookie::SameSite::Strict);

    Ok(HttpResponse::build(StatusCode::OK)
        .cookie(cookie)
        .json(HashMap::<(), ()>::new()))
}

#[get("/me")]
async fn me(user: UserClaims) -> ApiResult<web::Json<models::User>> {
    Ok(web::Json(models::User {
        username: user.username,
        name: user.name,
    }))
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Api route handler not found")
}

pub fn configure(keys: AuthKeys) -> Result<impl Fn(&mut ServiceConfig) + Clone> {
    let auth = auth::configure(keys)?;

    Ok(move |cfg: &mut ServiceConfig| {
        cfg
            // testing
            .service(hello)
            .service(ping)
            .service(echo)
            .service(make_error)
            // sessions
            .service(sessions::get_sessions)
            .service(sessions::create_session)
            .service(sessions::get_session)
            .service(sessions::delete_session)
            .service(sessions::add_mark)
            .service(sessions::delete_mark)
            // auth
            .service(login)
            .service(me)
            .configure(auth.clone())
            .default_service(web::route().to(not_found));
    })
}
