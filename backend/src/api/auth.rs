use crate::api::error::ApiError;
use crate::db::models as db_models;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::{Cookie, Expiration};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{web, FromRequest};
use anyhow::{anyhow, Context, Result};
use chrono::Duration;
use ed25519_dalek::{Keypair, KEYPAIR_LENGTH};
use jwt_compact::alg::Ed25519;
use jwt_compact::{AlgorithmExt, TimeOptions, Token, UntrustedToken};
use serde::{Deserialize, Serialize};
use tracing::warn;

// pub type Authority = actix_jwt_auth_middleware::Authority<UserClaims>;

#[derive(Debug)]
pub enum AuthError {
    NoCookie,
    UnparsableToken(jwt_compact::ParseError),
    InvalidToken(jwt_compact::ValidationError),
}

impl ApiError for AuthError {
    fn to_http(&self) -> (StatusCode, String) {
        match self {
            AuthError::NoCookie => (
                StatusCode::UNAUTHORIZED,
                "No session cookie found".to_string(),
            ),
            AuthError::UnparsableToken(_) => (
                StatusCode::BAD_REQUEST,
                "Could not parse session token".to_string(),
            ),
            AuthError::InvalidToken(_) => (
                StatusCode::UNAUTHORIZED,
                "Your session token does not pass validation, probably you should relogin"
                    .to_string(),
            ),
        }
    }
}

pub struct Authority {
    pub cookie_name: &'static str,
    key_pair: Keypair,
    header: jwt_compact::Header,
    time_options: TimeOptions,
    duration: Duration,
}

impl Authority {
    pub fn new(cookie_name: &'static str, key_pair: Keypair) -> Self {
        Self {
            cookie_name,
            key_pair,
            header: jwt_compact::Header::default(),
            time_options: TimeOptions::default(),
            duration: Duration::hours(3),
        }
    }

    pub fn create_signed_cookie(&self, claims: UserClaims) -> Result<Cookie> {
        let claims = jwt_compact::Claims::new(claims)
            .set_duration_and_issuance(&self.time_options, self.duration);
        let compact_token = Ed25519
            .token(self.header.clone(), &claims, &self.key_pair)
            .context("Could not create the token")?;
        Ok(Cookie::build(self.cookie_name, compact_token)
            .secure(true)
            .http_only(true)
            .expires(Expiration::DateTime(OffsetDateTime::from_unix_timestamp(
                claims.expiration.unwrap().timestamp(),
            )?))
            .finish())
    }

    fn extract_from_cookie(&self, cookie: Option<Cookie>) -> Result<Token<UserClaims>, AuthError> {
        let cookie = cookie.ok_or(AuthError::NoCookie)?;
        let untrusted_token =
            UntrustedToken::new(cookie.value()).map_err(AuthError::UnparsableToken)?;

        let token = Ed25519
            .validate_integrity(&untrusted_token, &self.key_pair.public)
            .map_err(AuthError::InvalidToken)?;

        Ok(token)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: db_models::UserId,
    pub username: String,
    pub name: String,
}

impl FromRequest for UserClaims {
    // works
    type Error = crate::api::error::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(match req.app_data::<web::Data<Authority>>() {
            Some(authority) => authority
                .extract_from_cookie(req.cookie(authority.cookie_name))
                .map(|token| token.claims().clone().custom) // TODO: we may want to provide a way to get standard claims like exp or iat
                .map_err(|e| {
                    warn!("Could not extract user claims from cookie: {:?}", e);
                    e.into()
                }),
            None => Err(anyhow!("Authority is not registered??").into()),
        })
    }
}

impl From<db_models::User> for UserClaims {
    fn from(u: db_models::User) -> Self {
        Self {
            user_id: u.id,
            username: u.username,
            name: u.name,
        }
    }
}

pub struct AuthKeys(Keypair);
impl Clone for AuthKeys {
    fn clone(&self) -> Self {
        AuthKeys(Keypair::from_bytes(&self.0.to_bytes()).unwrap())
    }
}

impl AuthKeys {
    pub const SIZE: usize = KEYPAIR_LENGTH;
    pub fn new(bytes: [u8; Self::SIZE]) -> Result<Self> {
        Ok(Self(Keypair::from_bytes(&bytes)?))
    }

    pub fn try_new(bytes: &[u8]) -> Result<Self> {
        Ok(Self(Keypair::from_bytes(bytes)?))
    }
}

pub fn configure(keys: AuthKeys) -> Result<impl Fn(&mut ServiceConfig) + Clone> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(Authority::new("session", keys.clone().0)));
    })
}
