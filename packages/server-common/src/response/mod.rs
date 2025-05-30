use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{CustomError, extract_info_from_custom_error};

pub fn empty_response(
    status_code: axum::http::StatusCode,
    headers: Option<axum::http::HeaderMap>,
) -> axum::response::Response<axum::body::Body> {
    let mut response = axum::response::Response::new(axum::body::Body::empty());
    let status = response.status_mut();
    *status = status_code;
    if let Some(h) = headers {
        let headers = response.headers_mut();
        *headers = h;
    }
    return response;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AxumResponse<T> {
    pub code: u64,
    pub data: T,
    pub message: String,
}

pub fn axum_response<T>(
    data_result: Result<T, CustomError>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response
where
    T: Serialize,
{
    let mut response = empty_response(axum::http::StatusCode::OK, Some(headers));
    let body = response.body_mut();
    match data_result {
        Ok(data) => {
            let response_data = AxumResponse {
                code: 0,
                data,
                message: "".to_string(),
            };
            let json_data_result = serde_json::to_vec(&response_data);
            match json_data_result {
                Ok(vec) => {
                    *body = axum::body::Body::from(vec);
                }
                Err(error) => {
                    let info = extract_info_from_custom_error(CustomError::JSON(
                        "response json serialize error".to_string(),
                    ));
                    let response_data = AxumResponse {
                        code: info.code,
                        data: "",
                        message: format!("{}:{}", error.to_string(), info.message),
                    };
                    let str = json!(response_data).to_string();
                    *body = axum::body::Body::from(str);
                }
            }
        }
        Err(error) => {
            let info = extract_info_from_custom_error(error);
            let response_data = AxumResponse {
                code: info.code,
                data: "",
                message: info.message,
            };
            let str = json!(response_data).to_string();

            *body = axum::body::Body::from(str);
        }
    }
    return response;
    // let json_data = serde_json::to_vec(&es_response).unwrap_or_default();
}
