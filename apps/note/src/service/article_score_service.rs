use opentelemetry::trace::{Span, SpanKind, Tracer};
use server_common::error::CustomError;

use crate::{
    dao, dto::add_article_score::AddArticleScoreDTO, middleware::log::get_tracer,
    model::article_score_entry::ArticleScoreEntry,
};

pub async fn page(
    pool: &sqlx::Pool<sqlx::Postgres>,
    from: u32,
    size: u32,
    article_id: &str,
) -> Result<Vec<ArticleScoreEntry>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("article score page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("article score page", vec![]);
    let es_response_result = dao::article_score_dao::page(pool, article_id, from, size).await;
    span.add_event("article score page finish", vec![]);
    return es_response_result;
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
    let es_response_result = dao::article_score_dao::add_article_score(pool, data).await;
    span.add_event("article score page finish", vec![]);
    return es_response_result;
}

// get sum by article_id
pub async fn get_sum_by_article_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    article_id: &str,
) -> Result<i32, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("get article score sum by article_id service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("get article score sum by article_id ", vec![]);
    let es_response_result =
        dao::article_score_dao::get_sum_score_by_article_id(pool, article_id).await;
    span.add_event("get article score sum by article_id finish", vec![]);
    return es_response_result;
}
