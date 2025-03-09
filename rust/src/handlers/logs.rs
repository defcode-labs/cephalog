use axum::response::IntoResponse;
use std::fs;
use axum::response::Json;
use crate::models::log::LogEntry;
use serde_json::{from_str, json, Value};
use std::path::PathBuf;

pub async fn get_logs() -> impl IntoResponse {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("db/logs.json");
    match fs::read_to_string(path) {
        Ok(logs) => {
            let parsed_logs: Value = from_str(&logs).unwrap();
            Json(parsed_logs)
        }
        ,
        Err(_) => Json(json!({"error": "Failed to read logs"})),
    }
}

pub async fn stream_logs() -> impl IntoResponse {
    // TODO: implement streaming logs
    Json::<Vec<LogEntry>>(Vec::new())  
}