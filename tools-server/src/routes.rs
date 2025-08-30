use axum::routing::get;
use axum::Router;

use crate::{health, metrics};

pub fn default_routes() -> Router {
    Router::new()
        .route("/health", get(health::get_health))
        .route("/metrics", get(metrics::get_metrics))
}