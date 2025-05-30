use chrono::Utc;
use server_common::error::CustomError;
use tracing::instrument;
#[instrument]
pub async fn get(_reqwest_client: reqwest::Client, _query: String) -> Result<String, CustomError> {
    let current_timestamp_millis = Utc::now().timestamp_millis();

    return Ok(current_timestamp_millis.to_string());
}
