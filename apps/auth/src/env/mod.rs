use std::env;

pub struct Environment;

impl Environment {
    pub fn get_opentelemetry_server_url() -> String {
        env::var("OPENTELEMETRY_SERVER_URL").unwrap_or("".to_string())
    }
    pub fn get_elasticsearch_api() -> String {
        env::var("ELASTICSEARCH_API").unwrap_or("".to_string())
    }
    pub fn get_account_table_name() -> String {
        env::var("ACCOUNT_TABLE_NAME").unwrap_or("".to_string())
    }
     pub fn get_user_table_name() -> String {
        env::var("USER_TABLE_NAME").unwrap_or("".to_string())
    }
  
}
