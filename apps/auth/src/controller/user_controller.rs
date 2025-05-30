use axum::Extension;
use opentelemetry::trace::{SpanKind, Tracer};
use serde::{Deserialize, Serialize};
use server_common::{error::CustomError, fetch::content_type_json_header, response::axum_response};
use tracing::error;
use typeshare::typeshare;

use crate::{
    env::Environment,
    middleware::log::get_tracer,
    route::{AuthAppExtension, AuthAppState},
    service,
};

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

pub async fn login(
    axum::extract::State(state): axum::extract::State<AuthAppState>,
    Extension(_ext): Extension<AuthAppExtension>,
    axum::extract::Json(payload): axum::extract::Json<LoginRequest>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("user login controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    let result =
        service::user_service::user_service_login(&state.db_pool, payload.account, payload.password).await;
    let token = result.unwrap_or("".to_string());
    let token_value_result = axum::http::HeaderValue::from_str(&format!(
        "{}={}; sameSite=strict; path=/; httpOnly=true; max-age=604800",
        Environment::get_cookie_key(),
        format!("Bearer {}", token)
    ));
    let token_value;
    match token_value_result {
        Ok(t) => {
            token_value = t;
        }
        Err(e) => {
            error!("string token to header value error : {:?}", e);
            token_value = axum::http::HeaderValue::from_static("");
        }
    }
    let mut headers = content_type_json_header();
    headers.insert(axum::http::header::SET_COOKIE, token_value);
    let wrapper: Result<String, CustomError> = Ok(token);
    return axum_response(wrapper, headers);
}

pub async fn logout(
    axum::extract::State(state): axum::extract::State<AuthAppState>,
    Extension(_ext): Extension<AuthAppExtension>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("user logout controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    let _ = service::user_service::user_service_logout(&state.db_pool).await;
    let token = "".to_string();
    let token_value_result = axum::http::HeaderValue::from_str(&format!(
        "{}=\"\"; sameSite=strict; path=/; httpOnly=true; max-age=1",
        Environment::get_cookie_key()
    ));
    let token_value;
    match token_value_result {
        Ok(t) => {
            token_value = t;
        }
        Err(e) => {
            error!("string token to header value error : {:?}", e);
            token_value = axum::http::HeaderValue::from_static("");
        }
    }
    let mut headers = content_type_json_header();
    headers.insert(axum::http::header::SET_COOKIE, token_value);
    let wrapper: Result<String, CustomError> = Ok(token);
    return axum_response(wrapper, headers);
}

pub async fn info(
    axum::extract::State(state): axum::extract::State<AuthAppState>,
    Extension(ext): Extension<AuthAppExtension>,
) -> axum::response::Response {
    let tracer = get_tracer();
    let mut _span = tracer
        .span_builder("user info controller")
        .with_kind(SpanKind::Internal)
        .start(tracer);
    let result = service::user_service::user_service_info(&state.db_pool, ext.account).await;

    return axum_response(result, content_type_json_header());
}
