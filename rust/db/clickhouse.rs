use clickhouse::{Client, Row};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_stream::StreamExt;

#[derive(Row, Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub source_ip: String,
    pub timestamp: String,
    pub event_type: String,
    pub request: String,
    pub status: String,
    pub threat_level: String,
}

pub struct ClickHouseDB {
    client: Client,
}

impl ClickHouseDB {
    /// Create a new ClickHouse connection
    pub fn new() -> Self {
        dotenv().ok(); // Load .env variables

        let db_url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL not set in .env");

        let client = Client::default().with_url(&db_url);

        Self { client }
    }

    /// Insert a log entry
    pub async fn insert_log(&self, log: LogEntry) -> Result<(), Box<dyn std::error::Error>> {
        let mut insert = self.client.insert("logs")?;
        insert.write(&log).await?;
        insert.end().await?;
        println!("Inserted log entry into ClickHouse!");
        Ok(())
    }

    /// Fetch logs with optional filtering
    pub async fn fetch_logs(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
        let query = format!(
            "SELECT source_ip, timestamp, event_type, request, status, threat_level FROM logs ORDER BY timestamp DESC LIMIT {}",
            limit.unwrap_or(50)
        );

        let mut rows = self.client.query(&query).fetch::<LogEntry>()?;
        let mut logs = Vec::new();

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
    
    use crate::mock::database::{MockDB, DbLogEntry, Database};
    use tokio::sync::Mutex;
    use IpAddr::V4;

    #[tokio::test]
    async fn test_insert_log() {
        let db = MockDB::new();

        let log = DbLogEntry {
            uuid: "123".to_string(),
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
    }

    #[tokio::test]
    async fn test_fetch_logs() {
        let db = MockDB::new();

        let log = DbLogEntry {
            uuid: "123".to_string(),
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