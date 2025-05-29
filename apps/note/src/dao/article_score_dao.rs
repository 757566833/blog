use server_common::
    error::{CustomError, log_error}
;
use sqlx::{Executor, Postgres};

use crate::{
    dto::add_article_score::AddArticleScoreDTO, model::article_score_entry::ArticleScoreEntry,
};

pub async fn add_article_score<'e, E>(
    executor: E,
    article_score: AddArticleScoreDTO,
) -> Result<u64, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO article_score (account, article_id, score, comment)
        VALUES ($1, $2, $3, $4)
        RETURNING id, account, article_id, score, comment, created_at, updated_at
        "#,
    )
    .bind(&article_score.account)
    .bind(&article_score.article_id)
    .bind(&article_score.score)
    .bind(&article_score.comment)
    .execute(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "insert article_score error"
        )))
    })?;
    return Ok(result.rows_affected());
}

// get article score by account and article_id
pub async fn get_article_score<'e, E>(
    executor: E,
    account: &str,
    article_id: &str,
) -> Result<Option<ArticleScoreEntry>, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query_as::<_, ArticleScoreEntry>(
        r#"
        SELECT id, account, article_id, score, comment, created_at, updated_at
        FROM article_score
        WHERE account = $1 AND article_id = $2
        "#,
    )
    .bind(account)
    .bind(article_id)
    .fetch_optional(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get article_score error"
        )))
    })?;

    Ok(result)
}

//  page article scores
pub async fn page<'e, E>(
    executor: E,
    account: &str,
    from: u32,
    size: u32,
) -> Result<Vec<ArticleScoreEntry>, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query_as::<_, ArticleScoreEntry>(
        r#"
        SELECT *
        FROM article_score
        WHERE account = $1
        ORDER BY create_time DESC
        OFFSET $2 LIMIT $3
        "#,
    )
    .bind(account)
    .bind(from as i64)
    .bind(size as i64)
    .fetch_all(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "page article_score error"
        )))
    })?;

    Ok(result)
}

// get sum  score by article_id
pub async fn get_sum_score_by_article_id<'e, E>(
    executor: E,
    article_id: &str,
) -> Result<i32, CustomError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query_scalar::<_, i32>(
        r#"
        SELECT SUM(score)
        FROM article_score
        WHERE article_id = $1
        "#,
    )
    .bind(article_id)
    .fetch_one(executor)
    .await
    .map_err(|error| {
        log_error(CustomError::Postgres(format!(
            "postgres error: {},{}",
            error.to_string(),
            "get sum score by article_id error"
        )))
    })?;

    Ok(result)
}