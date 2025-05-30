use server_common::{
    constant::{ESDetail, ESHitsAnalyze},
    error::CustomError,
};
use tracing::instrument;

use crate::{
    dao,
    dto::add_article_dto::AddArticleDTO,
    model::article_model::{ESAnalyzeArticleHighlight, ESArticleEntry},
};

#[instrument]
pub async fn article_service_page(
    reqwest_client: reqwest::Client,
    sort: Option<&str>,
    from: u32,
    size: u32,
    analyze: Option<String>,
) -> Result<ESHitsAnalyze<ESArticleEntry, ESAnalyzeArticleHighlight>, CustomError> {
    let es_response_result =
        dao::article_dao::article_dao_page(reqwest_client, sort, from, size, analyze).await;
    return es_response_result;
}
#[instrument]
pub async fn article_service_add(
    reqwest_client: reqwest::Client,
    data: AddArticleDTO,
) -> Result<String, CustomError> {
    let es_response_result = dao::article_dao::article_dao_add(reqwest_client, data).await;
    return es_response_result;
}
#[instrument]
pub async fn article_service_get(
    reqwest_client: reqwest::Client,
    id: &str,
) -> Result<ESDetail<ESArticleEntry>, CustomError> {
    let es_response_result = dao::article_dao::article_dao_get(reqwest_client, id).await;
    return es_response_result;
}
