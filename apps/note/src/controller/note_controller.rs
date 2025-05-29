use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::Deserialize;
use server_common::{fetch::content_type_json_header, response::axum_response};
use typeshare::typeshare;

use crate::{
    middleware::log::get_tracer,
    route::{NoteAppExtension, NoteAppState},
    service,
};

#[typeshare]
#[derive(Deserialize)]
pub struct NotePageRequest {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub analyze: Option<String>,
}

pub async fn note_page(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Query(params): axum::extract::Query<NotePageRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get note controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let from = (page - 1) * page_size;
    let analyze = params.analyze;
    let response =
        service::note_service::page(state.reqwest_client, None, from, page_size, analyze).await;

    return axum_response(response, content_type_json_header());
}
