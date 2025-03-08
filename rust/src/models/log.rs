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
        let (ip, timestamp, request, status, _, _, user_agent) = parse_nginx_log(line);
        let naive_datetime = NaiveDateTime::parse_from_str(&timestamp, "%d/%b/%Y:%H:%M:%S %z").ok()?;
        let timestamp = DateTime::<Utc>::from_utc(naive_datetime, Utc);

        Some(LogEntry {
            timestamp,
            source: LogSource::NginxAccess,
            ip_address: Some(ip),
            request: Some(request),
            status_code: Some(status),
            user_agent: Some(user_agent),
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

fn parse_nginx_log(log: &str) -> (String, String, String, u16, u32, String, String) {
    let re = Regex::new(
        r#"^\s*(?P<ip>\d+\.\d+\.\d+\.\d+)\s+- -\s+\[(?P<timestamp>[^\]]+)\]\s+\"(?P<request>[^\"]+)\"(?:\s+(?P<status>\d+))?(?:\s+(?P<size>\d+))?(?:\s+\"(?P<referer>[^\"]*)\")?(?:\s+\"(?P<user_agent>[^\"]*)\")?\s*$"#
        ).unwrap();

    let caps = re.captures(log);

    let ip = caps.as_ref().and_then(|c| c.name("ip")).map_or("0.0.0.0".to_string(), |m| m.as_str().to_string());
    let timestamp = caps.as_ref().and_then(|c| c.name("timestamp")).map_or("unknown".to_string(), |m| m.as_str().to_string());
    let request = caps.as_ref().and_then(|c| c.name("request")).map_or("-".to_string(), |m| m.as_str().to_string());

    let status = caps.as_ref().and_then(|c| c.name("status"))
        .and_then(|m| m.as_str().parse::<u16>().ok())
        .unwrap_or(0);

    // let status: u16 = caps.as_ref()?.name("status")
    // .ok_or("Missing status code".to_string())?
    // .as_str()
    // .parse::<u16>()
    // .map_err(|_| "Invalid status code (not a number)".to_string())?;

    let size = caps.as_ref().and_then(|c| c.name("size"))
        .and_then(|m| if m.as_str() == "-" { Some(0) } else { m.as_str().parse::<u32>().ok() })
        .unwrap_or(0);

    let referer = caps.as_ref().and_then(|c| c.name("referer")).map_or("-".to_string(), |m| m.as_str().to_string());
    let user_agent = caps.as_ref().and_then(|c| c.name("user_agent")).map_or("unknown".to_string(), |m| m.as_str().to_string());
    // println!("{:?}", (ip.clone(), timestamp.clone(), request.clone(), status, size, referer.clone(), user_agent.clone()));
    (ip, timestamp, request, status, size, referer, user_agent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_nginx_log() {
        let log_entry = r#"192.168.1.1 - - [12/Mar/2024:14:56:23 +0000] "GET /index.html HTTP/1.1" 200 512"#;
        let parsed = LogEntry::from_nginx_log(log_entry);

        let parsed = parsed.unwrap();
        assert_eq!(parsed.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(parsed.request, Some("GET /index.html HTTP/1.1".to_string()));
        assert_eq!(parsed.timestamp.to_string(), "2024-03-12 14:56:23 UTC".to_string());
        assert_eq!(parsed.status_code, Some(200));
    }

    #[test]
    fn test_parse_log_with_missing_fields() {
        let log_entry = r#"192.168.1.1 - - [12/Mar/2024:14:56:23 +0000] "GET /index.html HTTP/1.1""#;
        let result = LogEntry::from_nginx_log(log_entry);
        let result = result.unwrap();
        assert_eq!(result.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(result.request, Some("GET /index.html HTTP/1.1".to_string()));
        assert_eq!(result.timestamp.to_string(), "2024-03-12 14:56:23 UTC".to_string());
        assert_eq!(result.status_code, Some(0));
        assert_eq!(result.user_agent, Some("unknown".to_string()));       
    }

    /**
     * TODO: Implement this test
     * Do we need to handle invalid status code?
     */
    // #[test]
    // fn test_parse_log_with_invalid_status_code() {
    //     let log_entry = r#"192.168.1.1 - - [12/Mar/2024:14:56:23 +0000] "GET /index.html HTTP/1.1" XYZ 512"#;
    //     let result = LogEntry::from_nginx_log(log_entry);
    //     let result = result.unwrap();
    //     // if status code is 0 it is invalid or missing'
    //     assert_eq!(result.status_code, Some(0));
    // }

    #[test]
    fn test_parse_log_with_extra_spaces() {
        let log_entry = r#"   192.168.1.1    - -   [12/Mar/2024:14:56:23 +0000]  "GET   /index.html HTTP/1.1"   200   512 "#;
        let parsed = LogEntry::from_nginx_log(log_entry);
        let parsed = parsed.unwrap();
        assert_eq!(parsed.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(parsed.request, Some("GET   /index.html HTTP/1.1".to_string()));
        assert_eq!(parsed.timestamp.to_string(), "2024-03-12 14:56:23 UTC".to_string());
        assert_eq!(parsed.status_code, Some(200));
    }

    #[test]
    fn test_parse_log_with_empty_line() {
        let log_entry = "";
        let result = LogEntry::from_nginx_log(log_entry);

        assert!(result.is_none()); // empty line should return None
    }

    #[test]
    fn test_parse_multiple_log_entries() {
        let logs = vec![
            r#"192.168.1.1 - - [12/Mar/2024:14:56:23 +0000] "GET /index.html HTTP/1.1" 200 512"#,
            r#"10.0.0.2 - - [12/Mar/2024:15:10:45 +0000] "POST /api/data HTTP/1.1" 201 1024"#,
        ];

        for log in logs {
            let parsed = LogEntry::from_nginx_log(log);
            let parsed = parsed.unwrap();
            assert!(parsed.ip_address.as_ref().map_or(false, |ip| !ip.is_empty()));
            assert!(parsed.status_code.unwrap_or(0) > 0);
        }
    }
}
