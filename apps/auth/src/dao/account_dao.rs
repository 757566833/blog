use server_common::error::{CustomError, log_error};
use sqlx::{Executor, Postgres};

use crate::{
    dto::{
        add_account_dto::AddAccountDto, edit_account_dto::EditAccountDto,
        get_account_dto::GetAccountDto,
    },
    model::account_entry::AccountEntry,
};

pub async fn add_account<'e, E>(executor: E, account: AddAccountDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let _ = account;
    let result = sqlx::query(
        r#"
        INSERT INTO accounts (account, password_hash)
        VALUES ($1, $2)
        RETURNING id, account, create_time
        "#,
    )
    .bind(&account.account)
    .bind(&account.password_hash)
    .execute(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "insert account error"
        )))
    })?;
    return Ok(result.rows_affected());
}

// 生成与上述对应，修改account 的方法
pub async fn update_account<'e, E>(executor: E, account: EditAccountDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let _ = account;
    let result = sqlx::query(
        r#"
        UPDATE accounts
        SET password_hash = $1
        WHERE id = $2
        "#,
    )
    .bind(&account.password_hash)
    .bind(&account.id)
    .execute(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "update account error"
        )))
    })?;
    return Ok(result.rows_affected());
}
// 生成与上述对应，查询account 的方法 根据 get_account_dto 的 account 和 password_hash 查询， 存在就返回id 没有就是空字符串
pub async fn get_account(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: GetAccountDto,
) -> Result<String, CustomError> {
    let _ = account;
    let result = sqlx::query_as::<_, AccountEntry>(
        r#"
        SELECT id FROM accounts
        WHERE account = $1 AND password_hash = $2
        "#,
    )
    .bind(&account.account)
    .bind(&account.password_hash)
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get account error"
        )))
    })?;

    let id;
    if let Some(entry) = result {
        id = entry.id.to_string();
    } else {
        id = String::new();
    }

    return Ok(id);
}

// get account count by account
pub async fn get_account_count(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: &str,
) -> Result<i64, CustomError> {
    let result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM accounts
        WHERE account = $1
        "#,
    )
    .bind(account)
    .fetch_one(pool)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get account count error"
        )))
    })?;

    return Ok(result);
}

// get account by account
pub async fn get_account_by_account(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: &str,
) -> Result<Option<AccountEntry>, CustomError> {
    let result = sqlx::query_as::<_, AccountEntry>(
        r#"
        SELECT * FROM accounts
        WHERE account = $1
        "#,
    )
    .bind(account)
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get account by account error"
        )))
    })?;

    return Ok(result);
}
