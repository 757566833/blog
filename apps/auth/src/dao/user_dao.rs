use crate::dto::{add_user_dto::AddUserDto, edit_user_dto::EditUserDto};
use server_common::error::{CustomError, log_error};
use sqlx::{Executor, Postgres, postgres::PgPool};

// 根据add_user_dto 生成对应的用户添加方法
pub async fn add_user<'e, E>(executor: E, user: AddUserDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO users (account_id, nickname, avatar_url)
        VALUES (?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(&user.account_id)
    .bind(&user.nickname)
    .bind(&user.avatar_url)
    .execute(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "insert user error"
        )))
    })?;
    return Ok(result.rows_affected());
}

// 生成与上述对应，修改用户的相关方法 参数是EditUserDto
pub async fn update_user<'e, E>(executor: E, user: EditUserDto) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        UPDATE users
        SET nickname = ?, avatar_url = ?
        WHERE account_id = ?
        "#,
    )
    .bind(&user.nickname)
    .bind(&user.avatar_url)
    .bind(&user.account_id)
    .execute(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "update user error"
        )))
    })?;
    return Ok(result.rows_affected());
}

// get user count by account_id
pub async fn get_user_count_by_account_id(
    pool: &PgPool,
    account_id: &str,
) -> Result<i64, CustomError> {
    let result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE account_id = ?
        "#,
    )
    .bind(account_id)
    .fetch_one(pool)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get user count by account_id error"
        )))
    })?;
    return Ok(result);
}
