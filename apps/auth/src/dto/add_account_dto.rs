#[derive(Debug)]
pub struct AddAccountDto {
    pub account: String,
    pub password_hash: String,
}
