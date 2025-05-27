use opentelemetry::{global, Context};
use opentelemetry::trace::{FutureExt, Span, SpanKind, TraceContextExt, Tracer};
use opentelemetry_http::HeaderExtractor;
use std::sync::OnceLock;


pub fn get_tracer() -> &'static global::BoxedTracer {
    static TRACER: OnceLock<global::BoxedTracer> = OnceLock::new();
    TRACER.get_or_init(|| global::tracer("note"))
}
pub async  fn with_log_tracer(req: axum::extract::Request, next: axum::middleware::Next)-> Result<axum::response::Response, axum::http::StatusCode> {
    let context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(req.headers()))
    });
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("note")
        .with_kind(SpanKind::Server)
        .start_with_context(tracer, &context);

    span.add_event("dispatching request", vec![]);
    let cx = Context::default().with_span(span);
    let response = next.run(req).with_context(cx).await;
    Ok(response)
}
