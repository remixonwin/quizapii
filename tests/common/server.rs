use std::net::TcpListener;
use std::time::Duration;
use tokio::time::sleep;
use std::sync::Arc;
use crate::common::TestQuizRepository;
use quizmo::handlers;

pub async fn setup_test_server() -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let base_url = format!("http://{}", addr);
    
    let repo = Arc::new(TestQuizRepository::new());
    let app = handlers::create_app(repo);
    
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    sleep(Duration::from_millis(100)).await;
    
    Ok(base_url)
}

// ...existing handler functions from mod.rs...
