use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::Deserialize;
use server_common::{fetch::content_type_json_header, response::axum_response};
use typeshare::typeshare;

use crate::{
    dto::add_note::AddNoteDTO,
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
        .span_builder("get note page controller")
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

#[typeshare]
#[derive(Deserialize)]
pub struct AddNoteRequest {
    pub title: String,
    pub content: String,
}
pub async fn add_note(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(ext): Extension<NoteAppExtension>,
    axum::extract::Json(params): axum::extract::Json<AddNoteRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("add note controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let dto = AddNoteDTO {
        account: ext.account.clone(),
        title: params.title,
        content: params.content,
    };
    let response = service::note_service::add(state.reqwest_client, dto).await;

    return axum_response(response, content_type_json_header());
}

pub async fn get_note(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get note controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let response = service::note_service::get(state.reqwest_client, &id).await;

    return axum_response(response, content_type_json_header());
}
