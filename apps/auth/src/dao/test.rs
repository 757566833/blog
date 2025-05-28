use chrono::Utc;
use opentelemetry::trace::{Span, SpanKind, Tracer};
use server_common::{error::CustomError};

use crate::{
    middleware::log::get_tracer,
};
pub async fn get(_reqwest_client: reqwest::Client, _query: String) -> Result<String, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("get test repository")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    span.add_event("get utc time stamp", vec![]);
    let current_timestamp_millis = Utc::now().timestamp_millis();
    span.add_event("get utc time stamp end", vec![]);
    return Ok(current_timestamp_millis.to_string());
}
