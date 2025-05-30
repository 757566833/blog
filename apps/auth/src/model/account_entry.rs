use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct AccountEntry {
    #[typeshare(serialized_as = "String")]
    pub id: uuid::Uuid,
    pub account: String,
    pub password_hash: String,
    #[typeshare(serialized_as = "String")]
    pub create_time: DateTime<Utc>,
    #[typeshare(serialized_as = "String")]
    pub update_time: DateTime<Utc>,
}
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct OptionAccountEntry {
    pub id: Option<uuid::Uuid>,
    pub account: Option<String>,
    pub password_hash: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}
