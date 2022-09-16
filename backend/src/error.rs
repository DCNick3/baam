use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError as ActixResponseError};
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub struct AnyhowError(anyhow::Error);

impl Serialize for AnyhowError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}", self.0).serialize(serializer)
    }
}

impl Debug for AnyhowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Necessary because of this issue: https://github.com/actix/actix-web/issues/1711
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum Error {
    Anyhow(AnyhowError),
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
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(HashMap::from([("error", self)]))
    }
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Error::Anyhow(AnyhowError(t.into()))
    }
}
