use std::env;

pub struct Environment;

impl Environment {
    pub fn get_opentelemetry_server_url() -> String {
        env::var("OPENTELEMETRY_SERVER_URL").unwrap_or("".to_string())
    }
    pub fn get_postgres_server_address() -> String {
        env::var("POSTGRES_SERVER_ADDRESS").unwrap_or("".to_string())
    }
    pub fn get_postgres_db_name() -> String {
        env::var("POSTGRES_DB_NAME").unwrap_or("".to_string())
    }
    pub fn get_postgres_user() -> String {
        env::var("POSTGRES_USER").unwrap_or("".to_string())
    }
    pub fn get_postgres_password() -> String {
        env::var("POSTGRES_PASSWORD").unwrap_or("".to_string())
    }
}
