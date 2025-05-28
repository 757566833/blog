
#[derive(sqlx::FromRow)]
pub struct AccountEntry { 
    pub id: String,
    pub account: String, 
    pub password_hash: String,
    pub create_time: i64,
    pub update_time: i64, 
}

#[derive(sqlx::FromRow)]
pub struct OptionAccountEntry { 
    pub id: Option<String>,
    pub account: Option<String>, 
    pub password_hash: Option<String>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>, 
}