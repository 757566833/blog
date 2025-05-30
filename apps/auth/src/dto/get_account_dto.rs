#[derive(Debug)]
pub struct GetAccountDto {
    pub account: String,
    pub password_hash: String,
}
