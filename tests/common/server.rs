use axum::{
    routing::{get, post},
    Router, 
    Server,
};
use quizmo::handlers::{AppState, handle_register, handle_login, handle_create_quiz, handle_get_quiz};
use std::net::TcpListener;
use std::time::Duration;
use tokio::time::sleep;

pub async fn setup_test_server() -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let base_url = format!("http://{}", addr);
    
    let state = AppState::default();
    let app = create_test_server(state);
    
    tokio::spawn(async move {
        Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Wait for server to start
    sleep(Duration::from_millis(100)).await;
    
    Ok(base_url)
}

pub fn create_test_server(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/auth/register", post(handle_register))
        .route("/api/v1/auth/login", post(handle_login))
        .route("/api/v1/quizzes", post(handle_create_quiz))
        .route("/api/v1/quizzes/:id", get(handle_get_quiz))
        .with_state(state)
}

// ...existing handler functions from mod.rs...

mod server {
    use std::net::TcpListener;

    #[allow(dead_code)]
    pub async fn setup() -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = listener;
        
        let address = format!("http://127.0.0.1:{}", port);
        
        // Start the server in the background
        tokio::spawn(async move {
            server
        });

        address
    }
}
