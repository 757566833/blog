// 生成一个注册的方法

use server_common::error::{CustomError, log_error};
use sqlx::{Postgres, Transaction};

use crate::{
    dao::{account_dao, user_dao},
    dto,
};

pub async fn register(
    pool: &sqlx::Pool<sqlx::Postgres>,
    account: String,
    password: String,
) -> Result<u64, CustomError> {
    // 开启事务

    let existing_account_count = account_dao::get_account_count(pool, &account).await?;
    if existing_account_count > 0 {
        return Err(CustomError::Service("Account already exists".to_string()));
    }
    let existing_user_count = user_dao::get_user_count_by_account_id(pool, &account).await?;
    if existing_user_count > 0 {
        return Err(CustomError::Service("User already exists".to_string()));
    }
    let mut tx: Transaction<'_, Postgres> = pool.begin().await.map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "begin transaction error"
        )))
    })?;
    let add_account = dto::add_account_dto::AddAccountDto {
        account: account.clone(),
        password_hash: password, // 假设密码已经被哈希处理
    };

    let add_account_id = account_dao::add_account(tx.as_mut(), add_account).await?;
    let add_user = dto::add_user_dto::AddUserDto {
        account_id: account.clone(),
        nickname: account.clone(),
        avatar_url: "".to_string(), // 默认头像URL
    };
    let add_user_id = user_dao::add_user(tx.as_mut(), add_user).await?;
    if add_account_id > 0 && add_user_id > 0 {
        // 提交事务
        tx.commit().await.map_err(|error| {
            log_error(CustomError::Postgres(format!(
                "postgres error: {},{}",
                error.to_string(),
                "commit transaction error"
            )))
        })?;
    }

    Ok(add_user_id)
}
