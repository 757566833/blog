use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::{Deserialize, Serialize};
use server_common::{fetch::content_type_json_header, response::axum_response};

use crate::{
    middleware::log::get_tracer,  route::{NoteAppExtension, NoteAppState}, service
};

#[derive(Deserialize, Serialize)]
pub struct AiChatRequest {
    pub content: String,
    pub chat_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct AiAddQuestionRequest {
    pub query: String,
}
pub async fn get(
    axum::extract::State(_state): axum::extract::State<NoteAppState>,
    Extension(_ext): Extension<NoteAppExtension>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("get test controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    let result = service::test::get(_state.reqwest_client, "".to_string()).await;
    return axum_response(result,content_type_json_header());
}
