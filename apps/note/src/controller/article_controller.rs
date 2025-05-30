use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::Deserialize;
use server_common::{fetch::content_type_json_header, response::axum_response};
use typeshare::typeshare;

use crate::{
    dto::add_article_dto::AddArticleDTO,
    middleware::log::get_tracer,
    route::{NoteAppExtension, NoteAppState},
    service,
};

#[typeshare]
#[derive(Deserialize)]
pub struct ArticlePageRequest {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub analyze: Option<String>,
}

pub async fn article_page(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Query(params): axum::extract::Query<ArticlePageRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get article page controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let from = (page - 1) * page_size;
    let analyze = params.analyze;
    let response =
        service::article_service::article_service_page(state.reqwest_client, None, from, page_size, analyze).await;

    return axum_response(response, content_type_json_header());
}

#[typeshare]
#[derive(Deserialize)]
pub struct AddArticleRequest {
    pub title: String,
    pub content: String,
}
pub async fn add_article(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(ext): Extension<NoteAppExtension>,
    axum::extract::Json(params): axum::extract::Json<AddArticleRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("add article controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let dto = AddArticleDTO {
        account: ext.account.clone(),
        title: params.title,
        content: params.content,
    };
    let response = service::article_service::article_service_add(state.reqwest_client, dto).await;

    return axum_response(response, content_type_json_header());
}

pub async fn get_article(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get article controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let response = service::article_service::article_service_get(state.reqwest_client, &id).await;

    return axum_response(response, content_type_json_header());
}
