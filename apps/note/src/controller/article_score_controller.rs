use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::Deserialize;
use server_common::{fetch::content_type_json_header, response::axum_response};
use typeshare::typeshare;

use crate::{
    dto::add_article_score_dto::AddArticleScoreDTO,
    middleware::log::get_tracer,
    route::{NoteAppExtension, NoteAppState},
    service,
};

#[typeshare]
#[derive(Deserialize)]
pub struct ArticleScorePageRequest {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub article_id: String,
}

pub async fn article_score_page(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Query(params): axum::extract::Query<ArticleScorePageRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get article score page controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let from = (page - 1) * page_size;
    let article_id = params.article_id;
    let response =
        service::article_score_service::page(&state.postgres_db_pool, from, page_size, &article_id)
            .await;

    return axum_response(response, content_type_json_header());
}

#[typeshare]
#[derive(Deserialize)]
pub struct AddArticleScoreRequest {
    pub article_id: String,
    // 有意为之，应该是u32，为了制造bug
    pub score: i32,
    pub comment: String,
}
pub async fn add_article_score(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(ext): Extension<NoteAppExtension>,
    axum::extract::Json(params): axum::extract::Json<AddArticleScoreRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("add article score controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let dto = AddArticleScoreDTO {
        account: ext.account.clone(),
        article_id: params.article_id,
        score: params.score,
        comment: params.comment,
    };
    let response = service::article_score_service::add(&state.postgres_db_pool, dto).await;

    return axum_response(response, content_type_json_header());
}

pub async fn get_article_score_average(
    axum::extract::State(state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get article score average controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    let response =
        service::article_score_service::get_average_score_by_article_id(&state.postgres_db_pool, &id).await;

    return axum_response(response, content_type_json_header());
}
