use axum::response::IntoResponse;
use std::fs;
use axum::response::Json;
use crate::models::log::LogEntry;

pub async fn get_logs() -> impl IntoResponse {
    // TODO: implement get_logs
    Json::<Vec<LogEntry>>(Vec::new())  
}

pub async fn stream_logs() -> impl IntoResponse {
    // TODO: implement streaming logs
    Json::<Vec<LogEntry>>(Vec::new())  
}