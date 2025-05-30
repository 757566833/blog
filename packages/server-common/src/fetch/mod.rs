use std::collections::HashSet;

use axum::http::{HeaderMap, HeaderName};
use opentelemetry::{KeyValue, trace::TraceContextExt};
use serde::de::DeserializeOwned;
use tracing::{Span, instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{constant::UTF_8_JSON, error::CustomError, macro_log_error, response::empty_response};

pub fn reqwest_response_to_axum_response(
    reqwest_response: reqwest::Response,
    axum_response: Option<axum::response::Response>,
    filter_header: Option<HashSet<HeaderName>>,
) -> axum::response::Response {
    let mut response: axum::response::Response;
    if let Some(mut response_origin) = axum_response {
        let status_ref = response_origin.status_mut();
        *status_ref = reqwest_response.status();
        response = response_origin;
    } else {
        response = empty_response(reqwest_response.status(), None);
    }
    let headers = reqwest_response.headers();
    let headers_ref = response.headers_mut();
    if let Some(set) = filter_header {
        let mut response_header = HeaderMap::new();
        for (k, v) in headers {
            if set.contains(k) {
                response_header.append(k, v.clone());
            }
        }
        *headers_ref = response_header;
    } else {
        *headers_ref = reqwest_response.headers().clone();
    }

    let body_ref = response.body_mut();
    *body_ref = axum::body::Body::from_stream(reqwest_response.bytes_stream());

    return response;
}

pub fn content_type_json_header() -> axum::http::HeaderMap {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static(UTF_8_JSON),
    );
    return headers;
}
#[instrument]
pub async fn json_request_wrapper<T>(
    client: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    headers: Option<HeaderMap>,
    body: Option<String>,
) -> Result<T, CustomError>
where
    T: DeserializeOwned,
{
    let cx = Span::current().context();
    let otel_span = cx.span(); // op
    let mut builder = client.request(method.clone(), url);

    if let Some(h) = headers {
        otel_span.set_attribute(KeyValue::new("request.header", format!("{:?}", h)));
        builder = builder.headers(h);
    }

    if let Some(b) = &body {
        otel_span.set_attribute(KeyValue::new("request.body", format!("{:?}", b)));
        builder = builder.body(b.clone());
    }

    let request = builder.build().map_err(|error| {
        let custom_error =
            CustomError::HTTP(format!("http request build error: {}", error.to_string(),));
        macro_log_error!(custom_error);
        return custom_error;
    })?;

    let response = client.execute(request).await.map_err(|error| {
        let custom_error = CustomError::HTTP(format!("http execute error: {}", error.to_string(),));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    let status = response.status();
    otel_span.set_attribute(KeyValue::new("response.status", status.to_string()));
    let text = response.text().await.map_err(|error| {
        let custom_error = CustomError::HTTP(format!(
            "http response to string error: {}",
            error.to_string(),
        ));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    otel_span.set_attribute(KeyValue::new("response.body", text.clone()));
    if status.as_u16() >= 300 {
        let custom_error = CustomError::HTTP(format!(
            "http request error: {},{}",
            status.to_string(),
            text.clone()
        ));
        macro_log_error!(custom_error);

        return Err(custom_error);
    }
    let json = serde_json::from_str::<T>(&text).map_err(|error| {
        let custom_error = CustomError::JSON(format!(
            "http response string to json error: {}",
            error.to_string(),
        ));
        macro_log_error!(custom_error);
        return custom_error;
    })?;
    return Ok(json);
}
