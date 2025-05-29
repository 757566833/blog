use chrono::Utc;
use opentelemetry::trace::{Span, SpanKind, Tracer};
use serde_json::json;
use server_common::{constant::{ESAnalyzeSearchResult, ESDetail, ESHitsAnalyze, ESInsertOrUpdateResponse}, error::CustomError, fetch::json_request_wrapper};

use crate::{
    dto::add_article::AddArticleDTO, env::Environment, middleware::log::get_tracer, model::article_model::{ESAnalyzeArticleHighlight, ESArticleEntry}
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
        .span_builder(" article page repository")
        .with_kind(SpanKind::Internal)
        .start(tracer);
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
    span.add_event("search article page es", vec![]);
    let json =
        json_request_wrapper::<ESAnalyzeSearchResult<ESArticleEntry, ESAnalyzeArticleHighlight>>(
            &reqwest_client,
            reqwest::Method::GET,
            tracer,
            &format!(
                "{}/{}/_search",
                Environment::get_elasticsearch_api(),
                Environment::get_article_table_name()
            ),
            Some(server_common::fetch::content_type_json_header()),
            Some(document.clone()),
        )
        .await?;
    span.add_event("search article page es finish", vec![]);
    return Ok(json.hits);
}



pub async fn add(reqwest_client: reqwest::Client, data: AddArticleDTO) -> Result<String, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("add article repository")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    let current_timestamp_millis = Utc::now().timestamp_millis();
    let add_item = ESArticleEntry {
        title: data.title,
        content: data.content,
        create_time: current_timestamp_millis,
        account: data.account,
        update_time: current_timestamp_millis,
    };
    span.add_event("params to json string ", vec![]);
    let document = json!(add_item).to_string();
    span.add_event("params to json string end ", vec![]);
    span.add_event("params insert es ", vec![]);
    let json = json_request_wrapper::<ESInsertOrUpdateResponse>(
        &reqwest_client,
        reqwest::Method::POST,
        tracer,
        &format!(
            "{}/{}/_doc?refresh=wait_for",
            Environment::get_elasticsearch_api(),
            Environment::get_article_table_name()
        ),
        Some(server_common::fetch::content_type_json_header()),
        Some(document.clone()),
    )
    .await?;
    span.add_event("params insert es end", vec![]);
   
    return Ok(json._id);
}


// get by id
pub async fn get(
    reqwest_client: reqwest::Client,
    id: &str,
) -> Result<ESDetail<ESArticleEntry>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder("get article by id repository")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    span.add_event("get article by id", vec![]);
    let json = json_request_wrapper::<ESDetail<ESArticleEntry>>(
        &reqwest_client,
        reqwest::Method::GET,
        tracer,
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
    span.add_event("get article by id end", vec![]);
    return Ok(json);
}