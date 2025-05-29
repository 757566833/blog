use opentelemetry::trace::{Span, SpanKind, Tracer};
use server_common::{constant::DBPageResponse, error::CustomError};

use crate::{
    dao, dto::add_article_score_dto::AddArticleScoreDTO, middleware::log::get_tracer,
    model::article_score_entry::ArticleScoreEntry,
};

pub async fn page(
    pool: &sqlx::Pool<sqlx::Postgres>,
    from: u32,
    size: u32,
    article_id: &str,
) -> Result<DBPageResponse<ArticleScoreEntry>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("article score page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("article score page", vec![]);
    let response_result = dao::article_score_dao::page(pool, article_id, from, size).await?;

    span.add_event("article score page finish", vec![]);
    span.add_event("article score page", vec![]);
    let count_response_result =
        dao::article_score_dao::get_count_by_account(pool, article_id).await?;

    span.add_event("article score page finish", vec![]);
    let response = DBPageResponse {
        items: response_result,
        total: count_response_result as u32,
    };
    return Ok(response);
}

pub async fn add(
    pool: &sqlx::Pool<sqlx::Postgres>,
    data: AddArticleScoreDTO,
) -> Result<u64, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("article score page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("article score page", vec![]);
    let response_result = dao::article_score_dao::add_article_score(pool, data).await;
    span.add_event("article score page finish", vec![]);
    return response_result;
}

// get average by article_id
pub async fn get_average_score_by_article_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    article_id: &str,
) -> Result<f64, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("get article score average by article_id service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("get article score average by article_id ", vec![]);
    let response_result =
        dao::article_score_dao::get_average_score_by_article_id(pool, article_id).await;
    span.add_event("get article score average by article_id finish", vec![]);
    return response_result;
}
