use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct AccountEntry { 
    pub id: uuid::Uuid,
    pub account: String, 
    pub password_hash: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>, 
}

#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct OptionAccountEntry { 
    pub id: Option<uuid::Uuid>,
    pub account: Option<String>, 
    pub password_hash: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>, 
}