use actix_web::body::BoxBody;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::web::{BufMut, BytesMut};
use actix_web::{HttpResponse, ResponseError as ActixResponseError};
use std::fmt::{Display, Formatter};
use std::io::Write;

/// Necessary because of this issue: https://github.com/actix/actix-web/issues/1711
#[derive(Debug)]
pub enum Error {
    Anyhow(anyhow::Error),
}
pub type ResponseResult<T> = Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ActixResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        let mut buf = BytesMut::new();
        let _ = write!((&mut buf).writer(), "{}", self);

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        res.set_body(BoxBody::new(buf))
    }
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Error::Anyhow(t.into())
    }
}
