use actix_http::HttpMessage;
use actix_web::HttpRequest;
use opentelemetry::trace::{SpanId, TraceContextExt, TraceId};
use tracing_actix_web::{RequestId, RootSpan};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct RequestIds {
    pub request_id: RequestId,
    pub span_id: SpanId,
    pub trace_id: TraceId,
}

impl RequestIds {
    pub fn from_request(req: &HttpRequest) -> RequestIds {
        let ext = req.extensions();
        let root_span = ext.get::<RootSpan>().expect("Expected root span to be set");
        let request_id = *ext
            .get::<RequestId>()
            .expect("Expected request id to be set");

        let otel_context = root_span.context();
        let otel_span = otel_context.span();
        let span_context = otel_span.span_context();
        let span_id = span_context.span_id();
        let trace_id = span_context.trace_id();

        RequestIds {
            request_id,
            span_id,
            trace_id,
        }
    }
}
