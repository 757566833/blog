use opentelemetry::trace::{Span, SpanKind, Tracer};
use serde_json::json;
use server_common::{constant::{ESAnalyzeSearchResult, ESHitsAnalyze}, error::CustomError, fetch::json_request_wrapper};

use crate::{
    env::Environment, middleware::log::get_tracer, model::note_model::{ESAnalyzeNoteHighlight, ESNoteEntry}
};

pub async fn page(
    reqwest_client: reqwest::Client,
    sort: Option<&str>,
    from: u32,
    size: u32,
    analyze: Option<String>,
) -> Result<ESHitsAnalyze<ESNoteEntry, ESAnalyzeNoteHighlight>, CustomError> {
    let tracer = get_tracer();
    let mut span = tracer
        .span_builder(" note page repository")
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
    span.add_event("search note page es", vec![]);
    let json =
        json_request_wrapper::<ESAnalyzeSearchResult<ESNoteEntry, ESAnalyzeNoteHighlight>>(
            &reqwest_client,
            reqwest::Method::GET,
            tracer,
            &format!(
                "{}/{}/_search",
                Environment::get_elasticsearch_api(),
                Environment::get_note_table_name()
            ),
            Some(server_common::fetch::content_type_json_header()),
            Some(document.clone()),
        )
        .await?;
    span.add_event("search note page es finish", vec![]);

    // let response = reqwest_client
    //     .get(format!(
    //         "{}/{}/_search",
    //         Environment::get_elasticsearch_api(),
    //         Environment::get_note_table_name()
    //     ))
    //     .body(document)
    //     .headers(server_common::fetch::content_type_json_header())
    //     .send()
    //     .await
    //     .map_err(|error| {
    //         log_error(CustomError::HTTP(format!(
    //             "http error: {},{}",
    //             error.to_string(),
    //             "get note list  interface error"
    //         )))
    //     })?;
    // span.set_attribute(KeyValue::new(
    //     "response.status",
    //     response.status().to_string(),
    // ));
    // let text = response.text().await.map_err(|error| {
    //     log_error(CustomError::HTTP(format!(
    //         "http error: {},{}",
    //         error.to_string(),
    //         "search note page response to string error"
    //     )))
    // })?;
    // span.set_attribute(KeyValue::new("response.body", text.clone()));
    // let json = serde_json::from_str::<
    //     ESAnalyzeSearchResult<ESnote, ESAnalyzenoteHighlight>,
    // >(&text)
    // .map_err(|error| {
    //     log_error(CustomError::JSON(format!(
    //         "json: {},{}",
    //         error.to_string(),
    //         "get note page response json deserialization error"
    //     )))
    // })?;
    return Ok(json.hits);
}
