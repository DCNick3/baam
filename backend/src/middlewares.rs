use crate::diagnostics::RequestIds;
use actix_http::header::HeaderName;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::str::FromStr;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AddDiagnosticIds;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AddDiagnosticIds
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AddRequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddRequestIdMiddleware { service }))
    }
}

pub struct AddRequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AddRequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ids = RequestIds::from_request(req.request());

        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res: ServiceResponse<B> = fut.await?;

            res.headers_mut().insert(
                HeaderName::from_str("X-Request-Id").unwrap(),
                format!("{}", ids.request_id).parse().unwrap(),
            );
            res.headers_mut().insert(
                HeaderName::from_str("X-Span-Id").unwrap(),
                format!("{}", ids.span_id).parse().unwrap(),
            );
            res.headers_mut().insert(
                HeaderName::from_str("X-Trace-Id").unwrap(),
                format!("{}", ids.trace_id).parse().unwrap(),
            );

            Ok(res)
        })
    }
}
