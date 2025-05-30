use chrono::Utc;
use serde_json::json;
use server_common::{
    constant::{ESAnalyzeSearchResult, ESDetail, ESHitsAnalyze, ESInsertOrUpdateResponse},
    error::CustomError,
    fetch::json_request_wrapper,
};
use tracing::instrument;

use crate::{
    dto::add_article_dto::AddArticleDTO,
    env::Environment,
    model::article_model::{ESAnalyzeArticleHighlight, ESArticleEntry},
};

#[instrument]
pub async fn article_dao_page(
    reqwest_client: reqwest::Client,
    sort: Option<&str>,
    from: u32,
    size: u32,
    analyze: Option<String>,
) -> Result<ESHitsAnalyze<ESArticleEntry, ESAnalyzeArticleHighlight>, CustomError> {
    let create_time_sort = sort.unwrap_or("desc");
    let analyze_keyword = analyze.unwrap_or("".to_string());
    let document;
    if !analyze_keyword.is_empty() {
        document = json!({
            "query": {
                "bool": {
                    "should": [
                        {
                            "match": {
                                "title": analyze_keyword
                            }
                        },
                        {
                            "match": {
                                "content": analyze_keyword
                            }
                        }
                    ],
                }
            },
            "highlight": {
                "fields": {
                    "title": {},
                    "content": {}
                }
            },
            "sort": [
                {
                    "create_time": {
                        "order": create_time_sort
                    }
                }
            ],
            "from": from,
            "size": size
        })
        .to_string();
    } else {
        document = json!({
            "query": {
                "match_all": {}
            },
            "sort": [
                {
                    "create_time": {
                        "order": create_time_sort
                    }
                }
            ],
            "from": from,
            "size":size
        })
        .to_string();
    }
    let json =
        json_request_wrapper::<ESAnalyzeSearchResult<ESArticleEntry, ESAnalyzeArticleHighlight>>(
            &reqwest_client,
            reqwest::Method::GET,
            &format!(
                "{}/{}/_search",
                Environment::get_elasticsearch_api(),
                Environment::get_article_table_name()
            ),
            Some(server_common::fetch::content_type_json_header()),
            Some(document.clone()),
        )
        .await?;
    return Ok(json.hits);
}

#[instrument]
pub async fn article_dao_add(
    reqwest_client: reqwest::Client,
    data: AddArticleDTO,
) -> Result<String, CustomError> {
    let current_timestamp_millis = Utc::now().timestamp_millis();
    let add_item = ESArticleEntry {
        title: data.title,
        content: data.content,
        create_time: current_timestamp_millis,
        account: data.account,
        update_time: current_timestamp_millis,
    };

    let document = json!(add_item).to_string();
    let json = json_request_wrapper::<ESInsertOrUpdateResponse>(
        &reqwest_client,
        reqwest::Method::POST,
        &format!(
            "{}/{}/_doc?refresh=wait_for",
            Environment::get_elasticsearch_api(),
            Environment::get_article_table_name()
        ),
        Some(server_common::fetch::content_type_json_header()),
        Some(document.clone()),
    )
    .await?;
    return Ok(json._id);
}

#[instrument]
pub async fn article_dao_get(
    reqwest_client: reqwest::Client,
    id: &str,
) -> Result<ESDetail<ESArticleEntry>, CustomError> {
    let json = json_request_wrapper::<ESDetail<ESArticleEntry>>(
        &reqwest_client,
        reqwest::Method::GET,
        &format!(
            "{}/{}/_doc/{}",
            Environment::get_elasticsearch_api(),
            Environment::get_article_table_name(),
            id
        ),
        Some(server_common::fetch::content_type_json_header()),
        None,
    )
    .await?;
    return Ok(json);
}
