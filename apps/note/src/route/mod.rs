use crate::{
    controller,
    db::{init_es_db, init_postgres_db},
    middleware::{extension::with_extension, log::with_log_tracer},
};
use axum::{
    Extension, Router, middleware,
    routing::{get, post},
};

#[derive(Clone)]
pub struct NoteAppState {
    pub reqwest_client: reqwest::Client,
    pub postgres_db_pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Clone)]
pub struct NoteAppExtension {
    pub account: String,
}

pub async fn init_route() -> Router {
    let reqwest_client = reqwest::Client::new();
    init_es_db(reqwest_client.clone()).await;
    let postgres_db_pool = init_postgres_db().await;
    let state = NoteAppState {
        reqwest_client,
        postgres_db_pool,
    };

    let app: Router = Router::new()
        .route("/api/test", get(controller::test::get))
        .route(
            "/v1/article/page",
            get(controller::article_controller::article_page),
        )
        .route(
            "/v1/article",
            post(controller::article_controller::add_article),
        )
        .route(
            "/v1/article/{id}",
            get(controller::article_controller::get_article),
        )
        .route(
            "/v1/article/score/average/{id}",
            get(controller::article_score_controller::get_article_score_average),
        )
        .route(
            "/v1/article/score",
            post(controller::article_score_controller::add_article_score),
        )
        .route(
            "/v1/article/score/page",
            get(controller::article_score_controller::article_score_page),
        )
        .layer(middleware::from_fn(with_log_tracer))
        .layer(middleware::from_fn(with_extension))
        .layer(Extension(NoteAppExtension {
            account: "".to_string(),
        }))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            1024 * 1024 * 1024 * 10,
        ))
        .with_state(state);
    return app;
}
