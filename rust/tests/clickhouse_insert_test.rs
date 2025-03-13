use db::clickhouse::ClickHouseDB;
use db::schema::DbLogEntry;
use db::util::get_test_logs;

#[tokio::test]
async fn test_insert_log() {
    let db = ClickHouseDB::new();

    let logs = get_test_logs().await.unwrap();
    let result = db.insert_logs(logs).await.unwrap();
    assert_eq!(result, ());

    let logs = db.fetch_logs(None).await.unwrap();
    assert_eq!(logs.len(), 10);

}