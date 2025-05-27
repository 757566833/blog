use env::Environment;
use tokio::signal;

pub mod controller;
pub mod env;
pub mod middleware;
pub mod repository;
pub mod route;
pub mod service;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("apps/auth/.env").ok();
    // build our application with a route
    let router = route::init_route().await;

    let opentelemetry_server_url = Environment::get_opentelemetry_server_url();
    if opentelemetry_server_url.is_empty() {
        panic!("opentelemetry server url not found in env")
    }
    let (sdk_logger_provider, sdk_tracer_provider, sdk_mete_provider) =
        server_common::opentelemetry::init_opentelemetry(&opentelemetry_server_url,"ai-npc-workflow");
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11001").await;
    if let Ok(listener) = listener {
        println!("listening on {}", "0.0.0.0:11001");

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
