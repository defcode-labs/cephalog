use axum::{Router, routing::get};
use crate::handlers::logs::{get_logs, stream_logs};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_logs))
        .route("/stream", get(stream_logs))
}