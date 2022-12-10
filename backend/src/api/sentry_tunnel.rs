use crate::api::error::ApiResult;
use actix_web::{post, web, web::ServiceConfig, HttpResponse};
use anyhow::anyhow;
use awc::http::Uri;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    sentry_hostname: String,
    sentry_project_id: String,
}

struct Data {
    awc_client: awc::Client,
}

#[derive(Deserialize, Debug)]
struct SentryHeader {
    dsn: String,
}

// get post data and redirect to sentry
#[post("/tunnel")]
async fn tunnel(
    data: web::Data<Data>,
    config: web::Data<Option<Config>>,
    body: String,
) -> ApiResult<HttpResponse> {
    if let Some(config) = config.get_ref() {
        let client = &data.awc_client;

        let header = body
            .split('\n')
            .next()
            .ok_or_else(|| anyhow!("empty body?"))?;
        let header: SentryHeader =
            serde_json::from_str(header).map_err(|e| anyhow!("invalid header: {}", e))?;
        let dsn = Uri::from_str(&header.dsn).map_err(|e| anyhow!("invalid dsn: {}", e))?;

        let hostname = dsn
            .host()
            .ok_or_else(|| anyhow!("Missing hostname in DSN"))?;
        if hostname != config.sentry_hostname {
            return Err(anyhow!("Invalid hostname in DSN").into());
        }
        let project_id = dsn.path().trim_start_matches('/');
        if project_id != config.sentry_project_id {
            return Err(anyhow!("Invalid project ID in DSN").into());
        }

        let url = format!(
            "https://{}/api/{}/envelope/",
            config.sentry_hostname, config.sentry_project_id
        );

        let res = client
            .post(url)
            .send_body(body)
            .await
            .map_err(|e| anyhow!("Failed to send request to Sentry: {}", e))?;

        if res.status().is_success() {
            Ok(HttpResponse::Ok().finish())
        } else {
            Err(anyhow!("Sentry returned error: {}", res.status()).into())
        }
    } else {
        Err(anyhow!("Sentry tunnel is not configured").into())
    }
}

pub fn configure(config: Option<Config>) -> impl Fn(&mut ServiceConfig) + Clone {
    move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(Data {
            awc_client: awc::Client::default(),
        }))
        .app_data(web::Data::new(config.clone()))
        .service(tunnel);
    }
}
