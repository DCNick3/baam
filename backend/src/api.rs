use actix_web::{get, post, web::ServiceConfig, HttpResponse, Responder};
use anyhow::Result;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! (this is a stub 6)")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub fn configure() -> Result<impl Fn(&mut ServiceConfig) + Clone> {
    Ok(|cfg: &mut ServiceConfig| {
        cfg.service(hello).service(echo);
    })
}
