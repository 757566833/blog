use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;


#[typeshare]
#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct ArticleScoreEntry { 
    #[typeshare(serialized_as = "String")]
    pub id: uuid::Uuid,
    pub account: String, 
    pub article_id: String,
    pub score: i32,
    pub comment: String,
    #[typeshare(serialized_as = "String")]
    pub create_time: DateTime<Utc>,
    #[typeshare(serialized_as = "String")]
    pub update_time: DateTime<Utc>, 
}
