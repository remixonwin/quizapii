use quizapii::{start_server, config, repository};
use tokio::test;

#[test]
async fn test_server_startup() {
    let result = start_server("127.0.0.1:0").await;
    assert!(result.is_ok());
    
    let server = result.unwrap();
    assert!(server.local_addr().port() > 0);
}

#[test]
async fn test_server_configuration() {
    let config = config::get_configuration();
    assert!(config.is_ok());
    
    let conf = config.unwrap();
    assert!(!conf.database_url.is_empty());
    assert!(!conf.jwt_secret.is_empty());
}

#[test]
async fn test_database_connection() {
    let db = repository::get_connection().await;
    assert!(db.is_ok());
    
    let conn = db.unwrap();
    let ping = conn.execute("SELECT 1").await;
    assert!(ping.is_ok());
}
