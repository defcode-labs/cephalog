use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;

mod logs;
pub fn configure_routes() -> Router {
    Router::new()
    .nest("/api/v1", Router::new()
        .nest("/logs", logs::routes())
    )
    .layer(CorsLayer::permissive())
}
