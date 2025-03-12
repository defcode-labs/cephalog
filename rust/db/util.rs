

use std::fs;
use std::path::PathBuf;
use serde_json::{from_str, json, Value};
use crate::schema::DbLogEntry;

pub async fn get_test_logs() -> Result<Vec<DbLogEntry>, Box::<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("db/logs.json");
    match fs::read_to_string(path) {
        Ok(logs) => {
            let parsed_logs: Value = from_str(&logs).unwrap();
            let logs_vec: Vec<DbLogEntry> = serde_json::from_value(parsed_logs)?;
            Ok(logs_vec)
        }
        Err(e) => Err(Box::new(e)),
    }
}
