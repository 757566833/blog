use crate::{
    controller,
    middleware::{extension::with_extension, log::with_log_tracer},
};
use axum::{Extension, Router, middleware, routing::get};

#[derive(Clone)]
pub struct WorkflowAppState {
    pub reqwest_client: reqwest::Client,
    // pub elasticsearch_client:Elasticsearch
}

#[derive(Clone)]
pub struct WorkflowAppExtension {
    pub uid: String,
}

pub async fn init_route() -> Router {
    let reqwest_client = reqwest::Client::new();
    let state = WorkflowAppState { reqwest_client };

    let app: Router = Router::new()
        .route("/api/test", get(controller::test::get))
        .layer(middleware::from_fn(with_log_tracer))
        .layer(middleware::from_fn(with_extension))
        .layer(Extension(WorkflowAppExtension {
            uid: "".to_string(),
        }))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            1024 * 1024 * 1024 * 10,
        ))
        .with_state(state);
    return app;
}
