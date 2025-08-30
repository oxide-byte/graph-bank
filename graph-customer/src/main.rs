#![forbid(unsafe_code)]
mod domain;
mod receiver;
mod config;

use axum::Router;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tools_server::graceful_shutdown::graceful_shutdown;
use tools_server::observability::{http_metrics_middleware, setup_observability};
use tools_server::tracing::init_tracing_logging;
use tools_server::routes::default_routes;
use crate::config::graph_config::graph_routes;

#[tokio::main]
async fn main() {
    // Initialize structured logging for tracing (logs are NOT sent to Prometheus; they go to stdout)
    init_tracing_logging("http://loki:3100", "graph-customer");
    setup_observability();

    let app = Router::new()
        .layer(OtelInResponseLayer::default()) // Contains no Spawn:
        .merge(default_routes())
        .layer(OtelAxumLayer::default()) // Contains Spawn: "_spans":["api_demo_handler"]
        .merge(graph_routes())
        .layer(axum::middleware::from_fn(http_metrics_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!(
        "Metrics server listening on {}",
        listener.local_addr().unwrap()
    );
    println!("Server can be stopped by CTRL-C");
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}