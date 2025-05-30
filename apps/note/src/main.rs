use std::panic;

use env::Environment;
use server_common::macro_panic_log_error;
use tokio::signal;

pub mod controller;
pub mod dao;
pub mod db;
pub mod dto;
pub mod env;
pub mod middleware;
pub mod model;
pub mod route;
pub mod service;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("apps/note/.env").ok();
    // build our application with a route

    panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let location = info.location();
        let mut file = "";
        let mut line = 0;
        if let Some(location) = location {
            file = location.file();
            line = location.line();
        }
        let error_message;
        if let Some(message) = payload.downcast_ref::<&str>() {
            error_message = message.to_string();
        } else if let Some(message) = payload.downcast_ref::<String>() {
            error_message = message.clone();
        } else {
            error_message = "无法解析".to_string();
        }
        macro_panic_log_error!(file, line, error_message);
    }));

    let router = route::init_route().await;

    let opentelemetry_server_url = Environment::get_opentelemetry_server_url();
    if opentelemetry_server_url.is_empty() {
        panic!("opentelemetry server url not found in env")
    }
    let (sdk_logger_provider, sdk_tracer_provider, sdk_mete_provider) =
        server_common::opentelemetry::init_opentelemetry(&opentelemetry_server_url, "blog-note");
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11002").await;
    if let Ok(listener) = listener {
        println!("listening on {}", "0.0.0.0:11002");

        // 启动服务和监听退出信号，谁先完成就退出
        tokio::select! {
            _ = axum::serve(listener, router) => {},
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
