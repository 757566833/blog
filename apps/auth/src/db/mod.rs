use sqlx::postgres::PgPoolOptions;

use crate::env::Environment;
// "postgres://blog_user:blog_password@192.168.246.22:30200/blog_db"
pub async fn init_db() -> sqlx::Pool<sqlx::Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            Environment::get_postgres_user(),
            Environment::get_postgres_password(),
            Environment::get_postgres_server_address(),
            Environment::get_postgres_db_name()
        ))
        .await
        .expect("Failed to connect to the database");
    return pool;
}
