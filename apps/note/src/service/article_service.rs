use opentelemetry::trace::{Span, SpanKind, Tracer};
use server_common::{constant::{ESDetail, ESHitsAnalyze}, error::CustomError};

use crate::{
    dao,
    dto::add_article_dto::AddArticleDTO,
    middleware::log::get_tracer,
    model::article_model::{ESAnalyzeArticleHighlight, ESArticleEntry},
};

pub async fn page(
    reqwest_client: reqwest::Client,
    sort: Option<&str>,
    from: u32,
    size: u32,
    analyze: Option<String>,
) -> Result<ESHitsAnalyze<ESArticleEntry, ESAnalyzeArticleHighlight>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("article page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("article page analyze", vec![]);
    let es_response_result = dao::article_dao::page(reqwest_client, sort, from, size, analyze).await;
    span.add_event("article page analyze finish", vec![]);
    return es_response_result;
}

pub async fn add(reqwest_client: reqwest::Client, data: AddArticleDTO) -> Result<String, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("article page service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("article page analyze", vec![]);
    let es_response_result = dao::article_dao::add(reqwest_client, data).await;
    span.add_event("article page analyze finish", vec![]);
    return es_response_result;
}

pub async fn get(reqwest_client: reqwest::Client, id: &str) -> Result<ESDetail<ESArticleEntry>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("get article service")
        .with_kind(SpanKind::Internal)
        .start(tracer);

    span.add_event("get article service", vec![]);
    let es_response_result = dao::article_dao::get(reqwest_client, id).await;
    span.add_event("get article service finish", vec![]);
    return es_response_result;
}


