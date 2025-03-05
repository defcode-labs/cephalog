use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub source: LogSource,
    pub ip_address: Option<String>,
    pub user: Option<String>,
    pub request: Option<String>,
    pub status_code: Option<u16>,
    pub user_agent: Option<String>,
    pub auth_action: Option<String>,
    pub success: Option<bool>,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogSource {
    NginxAccess,
    AuthLog,
}

impl LogEntry {
    pub fn from_nginx_log(line: &str) -> Option<Self> {
        let re = Regex::new(r#"(?P<ip>\d+\.\d+\.\d+\.\d+) - - \[(?P<timestamp>.*?)\] \"(?P<request>.*?)\" (?P<status>\d+) .*?\"(?P<user_agent>.*?)\""#).ok()?;

        let caps = re.captures(line)?;

        let timestamp_str = caps.name("timestamp")?.as_str();
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str).ok()?.with_timezone(&Utc);

        Some(LogEntry {
            timestamp,
            source: LogSource::NginxAccess,
            ip_address: Some(caps["ip"].to_string()),
            request: Some(caps["request"].to_string()),
            status_code: caps["status"].parse().ok(),
            user_agent: Some(caps["user_agent"].to_string()),
            auth_action: None,
            success: None,
            user: None,
            raw: line.to_string(),
        })
    }

    pub fn from_auth_log(line: &str) -> Option<Self> {
        let re = Regex::new(r"(?P<timestamp>\w+ \d+ \d+:\d+:\d+) .* sshd\[.*\]: (?P<auth_action>Failed|Accepted) password for (?P<user>\w+) from (?P<ip>\d+\.\d+\.\d+\.\d+) port \d+ ssh2$").ok()?;
        let caps = re.captures(line)?;

        let timestamp_str = caps.name("timestamp")?.as_str();
        let naive_datetime = NaiveDateTime::parse_from_str(timestamp_str, "%b %d %H:%M:%S")
            .ok()?;
        let timestamp = DateTime::<Utc>::from_utc(naive_datetime, Utc);

        Some(LogEntry {
            timestamp,
            source: LogSource::AuthLog,
            ip_address: Some(caps["ip"].to_string()),
            user: Some(caps["user"].to_string()),
            auth_action: Some(caps["auth_action"].to_string()),
            success: Some(caps["auth_action"].to_string() == "Accepted"),
            request: None,
            status_code: None,
            user_agent: None,
            raw: line.to_string(),
        })
    }
}

pub fn parse_logs(file_path: &str, source: LogSource) -> Vec<LogEntry> {
    let file = File::open(file_path).expect("Failed to open log file");
    let reader = BufReader::new(file);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| match source {
            LogSource::NginxAccess => LogEntry::from_nginx_log(&line),
            LogSource::AuthLog => LogEntry::from_auth_log(&line),
        })
        .collect()
}

pub fn parse_all_logs() -> Vec<LogEntry> {
    let mut all_logs = Vec::new();

    let nginx_log_path = env::var("NGINX_LOG_PATH").unwrap_or_else(|_| "/var/log/nginx/access.log".to_string());
    let auth_log_path = env::var("AUTH_LOG_PATH").unwrap_or_else(|_| "/var/log/auth.log".to_string());

    let nginx_logs = parse_logs(&nginx_log_path, LogSource::NginxAccess);
    all_logs.extend(nginx_logs);

    let auth_logs = parse_logs(&auth_log_path, LogSource::AuthLog);
    all_logs.extend(auth_logs);

    all_logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    all_logs
}
