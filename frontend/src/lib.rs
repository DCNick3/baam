use actix_web::error::ErrorBadGateway;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder, Result};
use actix_web_static_files::ResourceFiles;
use awc::http::uri::Uri;
use awc::Connector;
use rustls::client::{ServerCertVerified, ServerCertVerifier};
use rustls::ServerName;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use url::Url;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub upstream: Option<Url>,
}

struct StaticData {
    client: awc::Client,
    upstream_url: Url,
}

async fn index(data: web::Data<StaticData>, req: HttpRequest) -> Result<HttpResponse> {
    const IGNORE_HEADERS: &[&str] = &["1"];

    let path = req.match_info().query("filename");
    let url = data.upstream_url.join(path).unwrap();
    let url = Uri::from_str(url.as_ref()).unwrap();

    let mut client_resp = data.client.get(url).send().await.map_err(ErrorBadGateway)?;

    let mut resp = HttpResponseBuilder::new(client_resp.status());

    for (name, value) in client_resp.headers() {
        if !IGNORE_HEADERS.iter().any(|v| v == name) {
            resp.append_header((name, value));
        }
    }

    Ok(resp.body(client_resp.body().await.map_err(ErrorBadGateway)?))
}

struct DummyVerifier;
impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}

pub fn configure(config: Config) -> anyhow::Result<impl Fn(&mut ServiceConfig) + Clone> {
    Ok(move |cfg: &mut ServiceConfig| {
        if let Some(upstream) = &config.upstream {
            // if we have an upstream frontend server configured - proxy connections to it
            info!("Will serve frontend from {}", upstream);
            let client = awc::Client::builder()
                .connector(
                    // disable https verification; this is only used for devserver anyways
                    Connector::new().rustls(Arc::new(
                        rustls::ClientConfig::builder()
                            .with_safe_defaults()
                            .with_custom_certificate_verifier(Arc::new(DummyVerifier))
                            .with_no_client_auth(),
                    )),
                )
                .finish();
            cfg.app_data(web::Data::new(StaticData {
                client,
                upstream_url: upstream.clone(),
            }))
            .route("/{filename:.*}", web::get().to(index));
        } else {
            // serve the built-in files otherwise
            info!("Will serve frontend from the built-in files");
            let generated = generate();
            cfg.service(ResourceFiles::new("/", generated));
        }
    })
}
