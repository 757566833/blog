use crate::{dto::{add_user_dto::AddUserDto, edit_user_dto::EditUserDto}, model::user_entry::UserEntry};
use server_common::{error::CustomError, macro_log_error};
use sqlx::{Executor, Postgres, postgres::PgPool};
use tracing::instrument;

#[instrument]
pub async fn add_user<'e, E>(executor: E, user: AddUserDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO users (account, nickname, avatar_url)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(&user.account)
    .bind(&user.nickname)
    .bind(&user.avatar_url)
    .execute(executor)
    .await
    .map_err(|error| {
        let custom_error = CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "insert user error"
        ));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    return Ok(result.rows_affected());
}

#[instrument]
pub async fn update_user<'e, E>(executor: E, user: EditUserDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        UPDATE users
        SET nickname = $1, avatar_url = $2
        WHERE account_id = $3
        "#,
    )
    .bind(&user.nickname)
    .bind(&user.avatar_url)
    .bind(&user.account_id)
    .execute(executor)
    .await
    .map_err(|error| {
        let custom_error = CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "update user error"
        ));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    return Ok(result.rows_affected());
}

#[instrument]
pub async fn get_user_by_account(
    pool: &PgPool,
    account: &str,
) -> Result<Option<UserEntry>, CustomError> {
    let result = sqlx::query_as::<_, UserEntry>(
        r#"
        SELECT *
        FROM users
        WHERE account = $1
        "#,
    )
    .bind(account)
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        let custom_error = CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get user by account error"
        ));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    return Ok(result);
}