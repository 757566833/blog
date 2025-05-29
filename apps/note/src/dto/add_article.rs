use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct AddArticleDTO {
    pub account: String,
    pub title: String,
    pub content: String,
}