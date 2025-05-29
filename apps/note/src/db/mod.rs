use serde_json::json;

use crate::env::Environment;

pub async fn init_db(reqwest_client: reqwest::Client) {
    let elasticsearch_api = Environment::get_elasticsearch_api();
    if elasticsearch_api.is_empty() {
        panic!("cant find elasticsearch by env")
    }
    let es_info_response = reqwest_client
        .get(&elasticsearch_api)
        .send()
        .await
        .expect("cant connect elastic search db");
    if es_info_response.status().as_u16() >= 300 {
        panic!("es not ready")
    }
    let note_table_name = Environment::get_note_table_name();
   

    if note_table_name.is_empty() {
        panic!("cant find table by env")
    }
    let task1 = init_note(reqwest_client.clone(), &elasticsearch_api, &note_table_name);

    tokio::join!(task1);
}
async fn init_note(reqwest_client: reqwest::Client, elasticsearch_api: &str, table_name: &str) {
    let check = reqwest_client
        .head(format!("{}/{}", &elasticsearch_api, table_name))
        .send()
        .await
        .expect(&format!("cant connect es in init {} ", table_name));
    if check.status() != axum::http::StatusCode::OK {
        let document = json!({
          "mappings": {
            "properties": {
              "account": { "type": "keyword" },
              "content": {
                "type": "text",
                "analyzer": "ik_max_word",
                "search_analyzer": "ik_smart"
              },
              "title": {
                "type": "text",
                "analyzer": "ik_max_word",
                "search_analyzer": "ik_smart"
              },
              "create_time": { "type": "date" },
              "update_time": { "type": "date" }
            }
          }
        })
        .to_string();
        let response = reqwest_client
            .put(format!("{}/{}", &elasticsearch_api, table_name))
            .body(document)
            .headers(server_common::fetch::content_type_json_header())
            .send()
            .await
            .expect(&format!("cant create {} table", table_name));
        if response.status().as_u16() >= 300 {
            panic!("init {} failed {:?}", table_name, response)
        }
    }
}
