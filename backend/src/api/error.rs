use crate::api::auth::AuthError;
use crate::api::sessions::SessionNotFoundError;
use crate::diagnostics::RequestIds;
use actix_web::body::{BoxBody, EitherBody, MessageBody};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, HttpResponse, ResponseError as ActixResponseError};
use enum_dispatch::enum_dispatch;
use futures::FutureExt;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub struct AnyhowApiError(anyhow::Error);

#[enum_dispatch]
pub trait ApiError: Debug {
    fn to_http(&self) -> (StatusCode, String);
}

impl Debug for AnyhowApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ApiError for AnyhowApiError {
    fn to_http(&self) -> (StatusCode, String) {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0))
    }
}

/// Necessary because of this issue: https://github.com/actix/actix-web/issues/1711
#[allow(clippy::enum_variant_names)]
#[enum_dispatch(ApiError)]
#[derive(Debug)]
pub enum Error {
    AnyhowApiError,
    AuthError,
    SessionNotFoundError,
}
pub type ApiResult<T> = Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ActixResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let (status, message) = self.to_http();

        HttpResponse::build(status).body(message)
    }
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Error::AnyhowApiError(AnyhowApiError(t.into()))
    }
}

async fn api_error_handler_impl<B: MessageBody + 'static>(
    res: dev::ServiceResponse<B>,
) -> Result<dev::ServiceResponse<EitherBody<B>>, actix_web::error::Error> {
    let (req, res) = res.into_parts();
    let (res, body) = res.into_parts();

    let body = actix_web::body::to_bytes(body)
        .await
        .map_err(|e| e.into())?;
    let body = std::str::from_utf8(&body).expect("Expected error body to be a utf-8 string");

    let req_ids = RequestIds::from_request(&req);

    let body = HashMap::from([
        ("error", body.to_string()),
        ("request_id", req_ids.request_id.to_string()),
        ("trace_id", req_ids.trace_id.to_string()),
        ("span_id", req_ids.span_id.to_string()),
    ]);

    let body = serde_json::to_string(&body).expect("Expected error body to be serializable");
    let body = EitherBody::right(body.boxed());

    let mut res = res.set_body(body);

    res.headers_mut().insert(
        actix_web::http::header::CONTENT_TYPE,
        actix_web::http::header::HeaderValue::from_static("application/json"),
    );

    Ok(dev::ServiceResponse::new(req, res))
}

pub fn api_error_handler(
    res: dev::ServiceResponse<BoxBody>,
) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
    Ok(ErrorHandlerResponse::Future(
        api_error_handler_impl(res).boxed_local(),
    ))
}
