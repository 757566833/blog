use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub const UTF_8_JSON: &str = "application/json";
pub const TEXT_PLAIN: &str = "text/plain; charset=UTF-8";
pub const CURRENT_VERSION: i64 = 1;
pub const DEFAULT_CONFIG_ID: i64 = 1;


#[derive(Debug, Serialize, Deserialize)]
pub struct ESSearchResult<T> {
    pub took: u64,
    pub timed_out: bool,
    pub _shards: ESShards,
    pub hits: ESHits<T>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESAnalyzeSearchResult<T, A> {
    pub took: u64,
    pub timed_out: bool,
    pub _shards: ESShards,
    pub hits: ESHitsAnalyze<T, A>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ESShards {
    pub total: u32,
    pub successful: u32,
    pub skipped: Option<u32>,
    pub failed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESHits<T> {
    pub total: ESTotal,
    pub max_score: Option<f32>,
    pub hits: Vec<ESHit<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESHitsAnalyze<T, A> {
    pub total: ESTotal,
    pub max_score: Option<f32>,
    pub hits: Vec<ESHitAnalyze<T, A>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESTotal {
    pub value: u64,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESHit<T> {
    pub _index: String,
    pub _id: String,
    pub _score: Option<f32>,
    pub _source: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESHitAnalyze<T, A> {
    pub _index: String,
    pub _id: String,
    pub _score: Option<f32>,
    pub _source: T,
    pub highlight: Option<A>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESDetail<T> {
    pub _index: String,
    pub _id: String,
    pub _version: Option<i128>,
    pub _seq_no: i128,
    pub _primary_term: i128,
    pub found: bool,
    pub _source: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESEmbeddingResponse {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESInsertOrUpdateResponse {
    pub _index: String,
    pub _id: String,
    pub _version: i128,
    pub result: String,
    pub _shards: ESShards,
    pub _seq_no: i128,
    pub _primary_term: i128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESBatchDelResponse {
    pub took: u64,                     // 删除操作所花费的时间，单位毫秒
    pub timed_out: bool,               // 是否超时
    pub deleted: u64,                  // 删除的文档数量
    pub batches: u64,                  // 处理的批次数量
    pub version_conflicts: u64,        // 版本冲突的数量
    pub noops: u64,                    // 无操作的数量
    pub retries: Retries,              // 重试次数信息
    pub throttled_millis: u64,         // 背景操作的节流时间，单位毫秒
    pub failures: Option<Vec<String>>, // 失败信息
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Retries {
    pub bulk: u64,   // 批量重试次数
    pub search: u64, // 搜索重试次数
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESBulkInsertResponse {
    pub took: u64,
    pub errors: bool,
    pub items: Vec<BulkInsertItemsResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkInsertItemsResponse {
    pub index: BulkInsertItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkInsertItem {
    pub _index: String,
    pub _id: String,
    pub _version: u64,
    pub result: String,
    pub _shards: ShardInfo,
    pub _seq_no: u64,
    pub _primary_term: u64,
    pub status: u64,
    pub error: Option<ESError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShardInfo {
    pub total: u64,
    pub successful: u64,
    pub failed: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESError {
    pub r#type: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESBatchByIdsDelItem {

    pub _index: String,
    pub _id: String,
    pub _version: i128,
    pub result: String,
    pub _shards: ESShards,
    pub _seq_no: i128,
    pub _primary_term: i128,
    pub status: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESBatchByIdsDelIGroup {
    pub delete: Option<ESBatchByIdsDelItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ESBatchByIdsDelResponse {
    pub took: u64,    // 删除操作所花费的时间，单位毫秒
    pub errors: bool, // 是否超时
    pub items: Vec<ESBatchByIdsDelIGroup>,
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ESCountResponse {
    pub count: u64,
    pub _shards: ESShards,
}


#[typeshare]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DBPageResponse<T> {
    pub items: Vec<T>,
    pub total: u32,
}