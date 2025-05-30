use server_common::{constant::DBPageResponse, error::CustomError};
use tracing::instrument;

use crate::{
    dao, dto::add_article_score_dto::AddArticleScoreDTO, model::article_score_entry::ArticleScoreEntry,
};
#[instrument]
pub async fn article_score_service_page(
    pool: &sqlx::Pool<sqlx::Postgres>,
    from: u32,
    size: u32,
    article_id: &str,
) -> Result<DBPageResponse<ArticleScoreEntry>, CustomError> {
    let response_result = dao::article_score_dao::article_score_dao_page(pool, article_id, from, size).await?;
    let count_response_result =
        dao::article_score_dao::article_score_dao_get_count_by_account(pool, article_id).await?;
    let response = DBPageResponse {
        items: response_result,
        total: count_response_result as u32,
    };
    return Ok(response);
}
#[instrument]
pub async fn article_score_service_add(
    pool: &sqlx::Pool<sqlx::Postgres>,
    data: AddArticleScoreDTO,
) -> Result<u64, CustomError> {
    let response_result = dao::article_score_dao::article_score_dao_add_article_score(pool, data).await;
    return response_result;
}

#[instrument]
pub async fn article_score_service_get_average_score_by_article_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    article_id: &str,
) -> Result<f64, CustomError> {
    let response_result =
        dao::article_score_dao::article_score_dao_get_average_score_by_article_id(pool, article_id).await;
    return response_result;
}
