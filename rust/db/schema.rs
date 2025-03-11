use clickhouse::Client;

pub async fn setup_schema(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS logs (
            uuid UUID DEFAULT generateUUIDv4(),
            timestamp DateTime default now(),
            source_ip String,
            event_type LowCardinality(String),
            targeted_service String,
            targeted_endpoint String,
            request String,
            status String,
            action_taken String,
            threat_level String
        ) ENGINE = MergeTree()
        ORDER BY (timestamp, source_ip, event_type)
        PARTITION BY toYYYYMM(timestamp)
        TTL timestamp + INTERVAL 90 DAY;
    "#;

    client.query(query).execute().await?;
    println!("ClickHouse logs table ensured!");
    Ok(())
}
