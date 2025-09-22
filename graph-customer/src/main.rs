#![forbid(unsafe_code)]
mod domain;
mod receiver;
mod config;

use crate::config::graph_config::graph_routes;
use crate::config::settings::Settings;
use axum::Router;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tools_server::graceful_shutdown::graceful_shutdown;
use tools_server::observability::{http_metrics_middleware, setup_observability};
use tools_server::routes::default_routes;
use tools_server::trace_span_layer::TraceSpanLayer;
use tools_server::tracing::init_tracing_logging;

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();
    // Initialize structured logging for tracing (logs are NOT sent to Prometheus; they go to stdout)
    init_tracing_logging(settings.loki.url.as_str(), "graph-customer");
    setup_observability();

    let app = Router::new()
        .merge(default_routes())
        .merge(graph_routes())
        .layer(OtelAxumLayer::default())
        .layer(OtelInResponseLayer::default())
        .layer(axum::middleware::from_fn(http_metrics_middleware))
        .layer(TraceSpanLayer::default());

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