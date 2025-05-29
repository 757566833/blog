use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct ESArticleEntry {
    pub account: String,
    pub title: String,
    pub content: String,
    #[typeshare(serialized_as = "i32")]
    pub create_time: i64,
    #[typeshare(serialized_as = "i32")]
    pub update_time: i64,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct ESAnalyzeArticleHighlight {
    pub title: Option<Vec<String>>,
    pub content: Option<Vec<String>>,
}