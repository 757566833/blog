use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct UserEntry {
    pub id: uuid::Uuid,
    pub account: String,
    pub nickname: String,
    pub avatar_url: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct OptionUserEntry {
    pub id: Option<uuid::Uuid>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}
