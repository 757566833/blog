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
    let account_table_name = Environment::get_account_table_name();
    let user_table_name = Environment::get_user_table_name();

    if account_table_name.is_empty() || user_table_name.is_empty() {
        panic!("cant find table by env")
    }
    let task1 = init_account(reqwest_client.clone(), &elasticsearch_api, &account_table_name);
    let task2 = init_user(
        reqwest_client.clone(),
        &elasticsearch_api,
        &user_table_name,
    );
   
    tokio::join!(
        task1, task2
    );
}
async fn init_account(reqwest_client: reqwest::Client, elasticsearch_api: &str, table_name: &str) {
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
              "password": { "type": "keyword" },
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

async fn init_user(reqwest_client: reqwest::Client, elasticsearch_api: &str, table_name: &str) {
    let check = reqwest_client
        .head(format!("{}/{}", &elasticsearch_api, table_name))
        .send()
        .await
        .expect(&format!("cant connect es in init {} ", table_name));
    if check.status() != axum::http::StatusCode::OK {
        let document = json!({
          "mappings": {
            "properties": {
              "aid": { "type": "keyword" },
              "username": { "type": "keyword" },
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
