use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::HeaderValue,
    middleware,
    routing::{any, get},
};
use env::Environment;
use opentelemetry::{
    KeyValue, global,
    trace::{SpanKind, TraceContextExt, Tracer},
};
use opentelemetry_http::HeaderInjector;
use server_common::{constant::TEXT_PLAIN, fetch::content_type_json_header};
use tokio::signal;
use tracing::{Level, error, span};
use uuid::Uuid;

pub mod env;

#[derive(Clone)]
pub struct GatewayAppState {
    pub reqwest_client: reqwest::Client,
    // pub elasticsearch_client:Elasticsearch
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename("apps/gateway/.env").ok();
    let client = Arc::new(reqwest::Client::new());
    let opentelemetry_server_url = Environment::get_opentelemetry_server_url();
    if opentelemetry_server_url.is_empty() {
        panic!("opentelemetry server url not found in env")
    }
    let (sdk_logger_provider, sdk_tracer_provider, sdk_mete_provider) =
        server_common::opentelemetry::init_opentelemetry(&opentelemetry_server_url, "blog-gateway");
    let reqwest_client = reqwest::Client::new();
    let state = GatewayAppState { reqwest_client };
    let app = Router::new()
        .route(
            "/{*path}",
            any(move |request| proxy_handler(request, client.clone())),
        )
        .route("/gateway/v1/token/info", get(handle_get_token_info))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            add_gateway_header,
        ))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            1024 * 1024 * 1024 * 10,
        ))
        .with_state(state);
    // .layer(ServiceBuilder::new());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11000").await;
    if let Ok(listener) = listener {
        println!("listening on {}", "0.0.0.0:11000");
        // let sdk_logger_provider_clone = sdk_logger_provider.clone();

        // 启动服务和监听退出信号，谁先完成就退出
        tokio::select! {
            _ = axum::serve(listener, app) => {},
            _ = signal::ctrl_c() => {
                println!("SIGINT/SIGTERM received, shutting down...");
            },
        }
        let _ = sdk_logger_provider.shutdown();
        let _ = sdk_tracer_provider.shutdown();
        let _ = sdk_mete_provider.shutdown();
    } else {
        println!("http server bind error");
    }
}

async fn handle_get_token_info(req: axum::http::Request<Body>) -> axum::response::Response {
    let default_account = HeaderValue::from_static("");
    let authorization = req
        .headers()
        .get(axum::http::header::COOKIE)
        .unwrap_or(&default_account)
        .to_str()
        .unwrap_or("");
    let mut token_payload_option = None;
    if !authorization.is_empty() {
        let cookies = authorization.split(";");
        let mut bearer_token = "".to_string();
        for cookie in cookies {
            if let Some((k, v)) = cookie.split_once('=') {
                if k.trim() == Environment::get_cookie_key() {
                    bearer_token = v.to_string();
                    break;
                }
            }
        }
        if !bearer_token.is_empty() {
            let token = bearer_token.trim_start_matches("Bearer ");
            let token_payload_result = server_common::jwt::token::parse_token(token.to_string());
            if let Ok(t) = token_payload_result {
                token_payload_option = Some(t);
            }
        }
    }
    match token_payload_option {
        Some(token_payload) => {
            server_common::response::axum_response(
                Ok(token_payload),
                content_type_json_header(),
            )
        }
        None => {
            server_common::response::axum_response(
                Ok(""),
                content_type_json_header(),
            )
        }
    }


}

async fn proxy_handler(
    req: axum::http::Request<Body>,
    client: Arc<reqwest::Client>,
) -> axum::response::Response {
    let uri = req.uri();
    let uri_path = uri.path();
    let query = uri.query().unwrap_or("");

    // 拆分路径
    let mut segments = uri_path.trim_start_matches('/').split('/');
    let first_segment = segments.next().unwrap_or("");
    let remaining_path: String = segments.collect::<Vec<_>>().join("/");

    // 构建新的 URI 路径 + 查询参数
    let mut new_uri = format!("/{}", remaining_path);
    if !query.is_empty() {
        new_uri.push('?');
        new_uri.push_str(query);
    }
    let span = span!(Level::INFO, "proxy_handler");
    let _enter = span.enter();
    // info!(
    //     "origin url : {}, target server : {}, proxy url: {}, request method: {}",
    //     uri,
    //     first_segment,
    //     new_uri,
    //     req.method()
    // );
    let target_origin: Option<String> = match first_segment {
        "auth" => env::get_auth_origin(),
        "note" => env::get_note_origin(),
        _ => None,
    };
    if let Some(target) = target_origin {
        let target_uri_str = format!("{}{}", target, new_uri);
        let target_uri = target_uri_str.parse::<reqwest::Url>();
        let method = req.method();
        match target_uri {
            Ok(target_uri) => {
                let mut proxy_request = reqwest::Request::new(method.clone(), target_uri);
                let headers = req.headers();
                proxy_request.headers_mut().extend(headers.clone());
                let body = req.into_body();
                let stream = body.into_data_stream();
                let request_body = reqwest::Body::wrap_stream(stream);
                *proxy_request.body_mut() = Some(request_body);

                let proxy_result = client.execute(proxy_request).await;
                // info!("gateway proxy_result:{:?}",proxy_result);
                match proxy_result {
                    Ok(result) => {
                        let mut response = server_common::fetch::reqwest_response_to_axum_response(
                            result, None, None,
                        );
                        // info!("gateway:{:?}",response.headers());
                        let headers = response.headers();
                        let context_type = headers.get(axum::http::header::CONTENT_TYPE);
                        if None == context_type {
                            let header_ref = response.headers_mut();
                            // let header = *header_ref;

                            header_ref.insert(
                                axum::http::header::CONTENT_TYPE,
                                axum::http::HeaderValue::from_static(TEXT_PLAIN),
                            );
                        }

                        // info!("response success");
                        return response;
                    }
                    Err(e) => {
                        error!("request error: {:?}", e);
                        return server_common::response::empty_response(
                            axum::http::StatusCode::BAD_GATEWAY,
                            None,
                        );
                    }
                }
            }
            Err(e) => {
                error!("target uri parse error {:?}", e);
                return server_common::response::empty_response(
                    axum::http::StatusCode::BAD_GATEWAY,
                    None,
                );
            }
        }

        // return (axum::http::StatusCode::NOT_FOUND, "path not found");
    } else {
        // return (axum::http::StatusCode::NOT_FOUND, "path not found");
        error!("target origin not found");
        return server_common::response::empty_response(axum::http::StatusCode::BAD_GATEWAY, None);
    }
}
static WHITELIST: [&'static str; 5] = [
    "/auth/v1/user/login",
    "/auth/v1/user/logout",
    "/auth/v1/user/info",
    "/note/v1/article/page",
    "/note/v1/article/*",
];

fn is_whitelisted(path: &str) -> bool {
    for rule in WHITELIST.iter() {
        if rule.ends_with("/*") {
            let prefix = &rule[..rule.len() - 1]; // remove '*'
            if path.starts_with(prefix) {
                return true;
            }
        } else if path == *rule {
            return true;
        }
    }

    false
}
// 中间件：为每个响应添加一个 Header uid 应该是解析token 获取，但是这个项目没有登录，直接在前端设置了 这里直接获取
async fn add_gateway_header(
    axum::extract::State(_state): axum::extract::State<GatewayAppState>,
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response<Body>, axum::http::StatusCode> {
    let uri = req.uri();
    let path = uri.path();
    let query = uri.query().unwrap_or("");
    let method = req.method();
    let req_header = req.headers();

    let default_account = HeaderValue::from_static("");

    let authorization = req
        .headers()
        .get(axum::http::header::COOKIE)
        .unwrap_or(&default_account)
        .to_str()
        .unwrap_or("");
    let mut token_payload_option = None;
    if !authorization.is_empty() {
        let cookies = authorization.split(";");
        let mut bearer_token = "".to_string();
        for cookie in cookies {
            if let Some((k, v)) = cookie.split_once('=') {
                if k.trim() == Environment::get_cookie_key() {
                    bearer_token = v.to_string();
                    break;
                }
            }
        }
        if !bearer_token.is_empty() {
            let token = bearer_token.trim_start_matches("Bearer ");
            let token_payload_result = server_common::jwt::token::parse_token(token.to_string());
            if let Ok(t) = token_payload_result {
                token_payload_option = Some(t);
            }
        }
    }

    if let Some(token_payload) = token_payload_option {
        let account = token_payload.account;
        let tracer = global::tracer(format!("blog-gateway-account-{}", account));
        let span = tracer
            .span_builder(String::from(format!(
                "blog-gateway-proxy-account-{}",
                account
            )))
            .with_kind(SpanKind::Server)
            .with_attributes(vec![
                KeyValue::new("account", account.to_string()),
                KeyValue::new("http.method", method.to_string()),
                KeyValue::new("http.path", path.to_string()),
                KeyValue::new("http.query", query.to_string()),
                KeyValue::new(
                    "http.request.headers",
                    format!("{:?}", req_header.to_owned()),
                ),
                // KeyValue::new("http.response.headers", format!("{:?}", req.headers())),
            ])
            .start(&tracer);
        let cx = opentelemetry::Context::current_with_span(span);

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut HeaderInjector(req.headers_mut()))
        });
        cx.span().add_event(
            "gateway add trace id to header",
            // vec![KeyValue::new("status")],
            vec![],
        );
        let uid_str = account.to_string();
        let header_value_result = HeaderValue::from_str(&uid_str);
        let header_value;
        match header_value_result {
            Ok(t) => {
                header_value = t;
            }
            Err(e) => {
                error!("string uid to header value error : {:?}", e);
                return Err(axum::http::StatusCode::UNAUTHORIZED);
            }
        }
        req.headers_mut()
            .insert(axum::http::header::AUTHORIZATION, header_value);
        let res = next.run(req).await;

        cx.span().add_event(
            "gateway got response",
            vec![KeyValue::new("status", res.status().to_string())],
        );
        return Ok(res);
    } else {
        if is_whitelisted(&path) {
            let uuid = Uuid::new_v4();
            let tracer_name = format!("blog-gateway-{}", uuid);
            let tracer = global::tracer(tracer_name);
            let span = tracer
                .span_builder(String::from(format!("blog-gateway-proxy-{}", uuid)))
                .with_kind(SpanKind::Server)
                .with_attributes(vec![
                    KeyValue::new("uuid", uuid.to_string()),
                    KeyValue::new("http.method", method.to_string()),
                    KeyValue::new("http.path", path.to_string()),
                    KeyValue::new("http.query", query.to_string()),
                    KeyValue::new(
                        "http.request.headers",
                        format!("{:?}", req_header.to_owned()),
                    ),
                    // KeyValue::new("http.response.headers", format!("{:?}", req.headers())),
                ])
                .start(&tracer);
            let cx = opentelemetry::Context::current_with_span(span);

            global::get_text_map_propagator(|propagator| {
                propagator.inject_context(&cx, &mut HeaderInjector(req.headers_mut()))
            });
            cx.span().add_event(
                "gateway add trace id to header",
                // vec![KeyValue::new("status")],
                vec![],
            );

            let res = next.run(req).await;

            Ok(res)
        } else {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
    }
}
