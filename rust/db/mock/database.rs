use async_trait::async_trait;
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::schema::DbLogEntry;

#[async_trait::async_trait]
pub trait Database: Send + Sync {
    async fn insert_log(&self, log: DbLogEntry) -> Result<(), String>;
    async fn fetch_logs(&self, limit: Option<u32>) -> Result<Vec<DbLogEntry>, String>;
}

pub struct ClickHouseDB;

#[async_trait::async_trait]
impl Database for ClickHouseDB {
    async fn insert_log(&self, log: DbLogEntry) -> Result<(), String> {
        // Real ClickHouse implementation
        println!("Inserting log: {:?}", log);
        Ok(())
    }

    async fn fetch_logs(&self, limit: Option<u32>) -> Result<Vec<DbLogEntry>, String> {
        // Real ClickHouse implementation
        Ok(vec![])
    }
}

pub struct MockDB {
    logs: Arc<Mutex<HashMap<String, DbLogEntry>>>,
}

impl MockDB {
    pub fn new() -> Self {
        MockDB {
            logs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}


#[async_trait::async_trait]
impl Database for MockDB {
    async fn insert_log(&self, log: DbLogEntry) -> Result<(), String> {
        let mut logs = self.logs.lock().await;
        logs.insert(log.uuid.clone(), log);
        Ok(())
    }

    async fn fetch_logs(&self, limit: Option<u32>) -> Result<Vec<DbLogEntry>, String> {
        let logs = self.logs.lock().await;
        let logs: Vec<DbLogEntry> = logs.values().cloned().take(limit.unwrap_or(logs.len() as u32) as usize).collect();
        Ok(logs)
    }
}
