use server_common::error::CustomError;
use tracing::instrument;

use crate::dao;
#[instrument]
pub async fn get(reqwest_client: reqwest::Client, query: String) -> Result<String, CustomError> {
    let result = dao::test::get(reqwest_client, query).await;
    return result;
}
