#[derive(sqlx::FromRow)]
pub struct UserEntry {
    pub id: String,
    pub account_id: String,
    pub nickname: String,
    pub avatar_url: String,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(sqlx::FromRow)]
pub struct  OptionUserEntry {
    pub id: Option<String>,
    pub account_id: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
}