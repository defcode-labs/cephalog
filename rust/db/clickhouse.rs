use clickhouse::{Client, Row};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_stream::StreamExt;
//use reqwest::Client;

use crate::schema::DbLogEntry;

pub struct InsertResult {
    pub rows: u64,
    pub bytes: u64,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Row)]
pub struct DbLogRow{
    pub source_ip: String,
    pub event_type: String,
    pub request: String,
    pub status: String,
    pub threat_level: String,
    pub targeted_service: String,
    pub targeted_endpoint: String,
    pub action_taken: String,
}

pub struct ClickHouseDB {
    client: Client,
}

impl ClickHouseDB {
    
    pub fn new() -> Self {
        dotenv().ok();

        let db_url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL not set in .env");
        let db = env::var("CLICKHOUSE_DB").expect("CLICKHOUSE_DB not set in .env");
        let db_user = env::var("CLICKHOUSE_USER").expect("CLICKHOUSE_USER not set in .env");
        println!("Connecting to ClickHouse at {}", db_url);
        let client = Client::default().with_url(&db_url).with_user(&db_user).with_database(&db);

        Self { client }
    }

    pub async fn insert_log(&self, log: DbLogEntry) -> Result<(), Box<dyn std::error::Error>> {
        let mut insert = self.client.insert("logs")?;
        insert.write(&log).await?;
        insert.end().await?;
        println!("Inserted log entry into ClickHouse!");
        Ok(())
    }

    pub async fn insert_logs(&self, logs: Vec<DbLogEntry>) -> Result<(), Box<dyn std::error::Error>> {
        let mut insert = self.client.insert("logs")?;

        //remove timestamp and id from logs
        /* 
         *  The order of the fields in the struct must match the order of the fields in the ClickHouse table
         *  We remove the timestamp and id fields from the logs since they are inserted automatically by ClickHouse
         */
        let row_logs: Vec<DbLogRow> = logs.iter().map(|log| {
            DbLogRow {
                source_ip: log.source_ip.clone(),
                event_type: log.event_type.clone(),
                targeted_service: log.targeted_service.clone(),
                targeted_endpoint: log.targeted_endpoint.clone(),
                request: log.request.clone(),
                status: log.status.clone(),
                action_taken: log.action_taken.clone(),
                threat_level: log.threat_level.clone(),
            }
        }).collect();

        for log in row_logs {
            insert.write(&log).await?;
        }
        insert.end().await?;
        println!("Inserted log entries into ClickHouse!");
        Ok(())
    }

    pub async fn fetch_logs(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<DbLogEntry>, Box<dyn std::error::Error>> {
        // let query = format!(
        //     "SELECT ?fields FROM logs ORDER BY timestamp DESC LIMIT {} FORMAT JSONEachRow",
        //     limit.unwrap_or(50)
        // );

        // let query = "SELECT ?fields FROM logs ORDER BY timestamp DESC LIMIT ?";
        // println!("Fetching logs from ClickHouse...{}", query);
        let mut rows = self.client.query("SELECT toString(id), toString(timestamp), source_ip, event_type, targeted_service, targeted_endpoint, request, status, action_taken, threat_level FROM test_db.logs ORDER BY timestamp DESC LIMIT ?")
        .bind(limit.unwrap_or(50)).fetch::<DbLogEntry>()?;
        let mut logs = Vec::new();
        println!("Fetching logs from ClickHouse...");
        while let Some(log) = rows.next().await? { 
            logs.push(log);
        }

        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    
    use crate::mock::database::{MockDB, Database};
    use tokio::sync::Mutex;
    use IpAddr::V4;

    #[tokio::test]
    async fn test_insert_log() {
        let db = MockDB::new();

        let log = DbLogEntry {
            id: "123".to_string(),
            timestamp: "2021-01-01T00:00:00".to_string(),
            source_ip: "192.168.1.1".to_string(),
            event_type: "login".to_string(),
            targeted_service: "auth".to_string(),
            targeted_endpoint: "/login".to_string(),
            request: "POST /login".to_string(),
            status: "200".to_string(),
            action_taken: "allow".to_string(),
            threat_level: "low".to_string(),
        };

        let result = db.insert_log(log).await.unwrap();
        assert_eq!(result, ());

        let logs = db.fetch_logs(None).await.unwrap();
        assert_eq!(logs.len(), 110);
    }

    #[tokio::test]
    async fn test_fetch_logs() {
        let db = MockDB::new();

        let log = DbLogEntry {
            id: "123".to_string(),
            timestamp: "2021-01-01T00:00:00".to_string(),
            source_ip: "192.168.1.1".to_string(),
            event_type: "login".to_string(),
            targeted_service: "auth".to_string(),
            targeted_endpoint: "/login".to_string(),
            request: "POST /login".to_string(),
            status: "200".to_string(),
            action_taken: "allow".to_string(),
            threat_level: "low".to_string(),
        };

        let result = db.insert_log(log).await.unwrap();
        assert_eq!(result, ());

        let logs = db.fetch_logs(None).await.unwrap();
        assert_eq!(logs.len(), 1);
    }
}