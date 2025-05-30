use bcrypt::DEFAULT_COST;
use chrono::Utc;
use server_common::{error::CustomError, jwt::token::TokenPayload, macro_log_error};
use sqlx::{Postgres, Transaction};
use tracing::instrument;

use crate::{
    dao::{account_dao, user_dao},
    dto,
    model::user_entry::UserEntry,
};
#[instrument]
pub async fn user_service_login(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: String,
    password: String,
) -> Result<String, CustomError> {
    let option_account = account_dao::account_dao_get_account_by_account(pool, &account).await?;
    if let Some(account) = option_account {
        let hashed = account.password_hash.clone();
        let is_valid = bcrypt::verify(password, &hashed).map_err(|error| {
            let custom_error = CustomError::Service(format!(
                "bcrypt verify error: {},{}",
                error.to_string(),
                "verify password error"
            ));
            macro_log_error!(custom_error);
            return custom_error;
        })?;
        if is_valid {
            // 密码验证成功，生成JWT令牌
            let token = server_common::jwt::token::generate_token(TokenPayload {
                account: account.account.clone(),
            })?;
            return Ok(token);
        } else {
            // 密码验证失败
            let custom_error = CustomError::Service("Invalid account or password".to_string());
            macro_log_error!(custom_error);
            return Err(custom_error);
        }
    } else {
        let hashed = bcrypt::hash(password, DEFAULT_COST).map_err(|error| {
            let custom_error = CustomError::Service(format!(
                "bcrypt hash error: {},{}",
                error.to_string(),
                "hash password error"
            ));
            macro_log_error!(custom_error);
            return custom_error;
        })?;
        let mut tx: Transaction<'_, Postgres> = pool.begin().await.map_err(|error| {
            // 开启事务失败
            let custom_error = CustomError::Postgres(format!(
                "postgres error: {},{}",
                error.to_string(),
                "begin transaction error"
            ));
            macro_log_error!(custom_error);
            return custom_error;
        })?;
        let add_account = dto::add_account_dto::AddAccountDto {
            account: account.clone(),
            password_hash: hashed.clone(), // 假设密码已经被哈希处理
        };

        let add_account_id = account_dao::account_dao_add_account(tx.as_mut(), add_account).await?;
        let add_user = dto::add_user_dto::AddUserDto {
            account: account.clone(),
            nickname: account.clone(),
            avatar_url: "".to_string(), // 默认头像URL
        };
        let add_user_id = user_dao::user_dao_add_user(tx.as_mut(), add_user).await?;
        if add_account_id > 0 && add_user_id > 0 {
            // 提交事务
            tx.commit().await.map_err(|error| {
                // 提交事务失败
                let custom_error = CustomError::Postgres(format!(
                    "postgres error: {},{}",
                    error.to_string(),
                    "commit transaction error"
                ));
                macro_log_error!(custom_error);
                return custom_error;
            })?;
        }
        let token = server_common::jwt::token::generate_token(TokenPayload {
            account: account.clone(),
        })?;
        return Ok(token);
    }
}

#[instrument]
pub async fn user_service_info(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: String,
) -> Result<Option<UserEntry>, CustomError> {
    let user: Option<UserEntry> = user_dao::user_dao_get_user_by_account(pool, &account).await?;
    return Ok(user);
}
#[instrument]
pub async fn user_service_logout(_pool: &sqlx::Pool<sqlx::Postgres>) -> Result<i64, CustomError> {
    let current_timestamp_millis = Utc::now().timestamp_millis();
    return Ok(current_timestamp_millis);
}
