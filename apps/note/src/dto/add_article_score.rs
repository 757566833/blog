use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct AddArticleScoreDTO {
    pub account: String,
    pub article_id: String,
    pub score: i32,
    pub comment: String,
}