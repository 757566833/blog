use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct ESNoteEntry {
    pub account: String,
    pub title: String,
    pub content: String,
    pub create_time: i32,
    pub update_time: i32,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct ESAnalyzeNoteHighlight {
    pub title: Option<Vec<String>>,
    pub content: Option<Vec<String>>,
}