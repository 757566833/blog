use opentelemetry::trace::{Span, SpanKind, Tracer};
use server_common::{constant::ESHitsAnalyze, error::CustomError};

use crate::{dao, middleware::log::get_tracer, model::note_model::{ESAnalyzeNoteHighlight, ESNoteEntry}};



pub async fn page(
    reqwest_client: reqwest::Client,
    sort: Option<&str>,
    from: u32,
    size: u32,
    analyze:Option<String>,
) -> Result<ESHitsAnalyze<ESNoteEntry,ESAnalyzeNoteHighlight>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("note page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("note page analyze", vec![]);
    let es_response_result = dao::note_dao::page(reqwest_client, sort, from, size,analyze).await;
    span.add_event("note page analyze finish", vec![]);
    return es_response_result;
}
