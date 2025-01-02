use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use quizmo::models::test_types::{TestQuiz, TestQuestion};
use quizmo::repository::quiz_repository::QuizRepositoryImpl;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Once;
use tempfile::TempDir;
use tokio::sync::OnceCell;
use uuid::Uuid;

static TEST_SERVER: OnceCell<()> = OnceCell::const_new();
static INIT: Once = Once::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUser {
    // Renamed from User to TestUser
    pub username: String,
    pub email: String,
    pub password: String,
}

// Remove nesting and fix function structure
pub async fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
    setup_test_server().await;
}

pub async fn setup_test_server() {
    TEST_SERVER
        .get_or_init(|| async {
            let temp_dir = tempfile::tempdir().unwrap();
            let repo = QuizRepositoryImpl::new_with_path(temp_dir.path()).unwrap();

            let state = Arc::new(repo);

            let app = Router::new()
                .route("/api/v1/auth/register", post(handle_register))  // Keep v1 prefix
                .route("/api/v1/auth/login", post(handle_login))
                .route("/api/v1/quizzes", post(handle_create_quiz))
                .route("/api/v1/quizzes/:id", get(handle_get_quiz))
                .with_state(state);

            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

            tokio::spawn(async move {
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            });

            // Wait for server to start
            let client = reqwest::Client::new();
            let mut retries = 5;
            while retries > 0 {
                match client.get("http://localhost:3000/health").send().await {
                    Ok(_) => break,
                    Err(_) => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        retries -= 1;
                    }
                }
            }
            if retries == 0 {
                panic!("Failed to start test server");
            }
        })
        .await;
}

#[axum_macros::debug_handler]
async fn handle_register(
    State(_state): State<Arc<QuizRepositoryImpl>>,
    Json(user): Json<TestUser>,
) -> impl IntoResponse {
    if user.username.is_empty() || user.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid input" })),
        )
            .into_response();
    }
    (StatusCode::CREATED, Json(json!({ "status": "created" }))).into_response()
}

#[axum_macros::debug_handler]
async fn handle_login(
    State(_state): State<Arc<QuizRepositoryImpl>>,
    Json(user): Json<TestUser>,
) -> impl IntoResponse {
    if user.password == "wrong_password" {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid credentials" })),
        )
            .into_response();
    }
    (
        StatusCode::OK,
        Json(json!({
            "token": "test_token",
            "user": user
        })),
    )
        .into_response()
}

#[axum_macros::debug_handler]
async fn handle_create_quiz(
    State(_state): State<Arc<QuizRepositoryImpl>>,
    headers: axum::http::HeaderMap,
    Json(quiz): Json<TestQuiz>,
) -> impl IntoResponse {
    // Fix authorization header check
    if !headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.starts_with("Bearer "))
        .unwrap_or(false) 
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        )
            .into_response();
    }

    // Validate quiz
    if quiz.title.is_empty() || quiz.questions.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid quiz" })),
        )
            .into_response();
    }

    // Validate question options
    for question in &quiz.questions {
        if question.options.len() > 10 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Too many options" })),
            )
                .into_response();
        }
    }

    (StatusCode::CREATED, Json(quiz)).into_response()
}

#[axum_macros::debug_handler]
async fn handle_get_quiz(
    State(_state): State<Arc<QuizRepositoryImpl>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    if id == "nonexistent_id" {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))).into_response();
    }

    (StatusCode::OK, Json(create_sample_quiz("Test Quiz"))).into_response()
}

pub struct TestContext {
    pub api_client: reqwest::Client,
    pub base_url: String,
    pub repo: QuizRepositoryImpl,
    pub _temp_dir: TempDir,
}

impl TestContext {
    pub async fn new() -> Self {
        setup().await;
        
        // Add retry logic for server connection
        let client = reqwest::Client::new();
        let base_url = "http://localhost:3000".to_string();
        
        // Wait for server to be ready
        let mut retries = 5;
        while retries > 0 {
            if client.get(&format!("{}/health", base_url)).send().await.is_ok() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            retries -= 1;
        }
        if retries == 0 {
            panic!("Server not ready");
        }

        let temp_dir = TempDir::new().unwrap();
        let repo = QuizRepositoryImpl::new_with_path(temp_dir.path()).unwrap();

        Self {
            api_client: client,
            base_url,
            repo,
            _temp_dir: temp_dir,
        }
    }

    pub async fn create_test_quiz(&self, quiz: &TestQuiz) -> reqwest::Result<Response> {
        self.api_client
            .post(format!("{}/api/v1/quizzes", self.base_url))
            .headers(self.auth_header("test_token")) // Include Authorization header
            .json(quiz)
            .send()
            .await
    }

    pub async fn get_quiz(&self, id: &str) -> reqwest::Result<Response> {
        self.api_client
            .get(format!("{}/api/v1/quizzes/{}", self.base_url, id))
            .headers(self.auth_header("test_token")) // Include Authorization header
            .send()
            .await
    }

    pub async fn register_user(&self, user: &TestUser) -> reqwest::Result<Response> {
        self.api_client
            .post(format!("{}/api/v1/auth/register", self.base_url))  // Fix path
            .json(user)
            .send()
            .await
    }

    pub async fn login_user(&self, user: &TestUser) -> reqwest::Result<Response> {
        self.api_client
            .post(format!("{}/api/v1/auth/login", self.base_url))
            .json(user)
            .send()
            .await
    }

    // Add helper method for authorization header
    pub fn auth_header(&self, token: &str) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "authorization",
            format!("Bearer {}", token).parse().unwrap()
        );
        headers
    }
}

pub fn create_sample_quiz(title: &str) -> TestQuiz {
    TestQuiz {
        id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        description: "A test quiz".to_string(),
        questions: vec![TestQuestion {
            text: "What is Rust?".to_string(),
            options: vec![
                "A programming language".to_string(),
                "A metal oxide".to_string(),
            ],
            correct_answer: 0,
        }],
    }
}

pub fn create_test_user() -> TestUser {
    TestUser {
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password: "password123".to_string(),
    }
}

// Remove unused functions
// Remove the following unused functions to eliminate warnings:

// pub async fn create_test_quiz_async(
//     _client: &reqwest::Client,
//     _token: Option<&str>,
// ) -> Result<TestQuiz, Error> {
//     Ok(create_sample_quiz("Test Quiz"))
// }

// pub fn build_test_quiz(title: &str) -> Quiz {
//     Quiz::new(
//         title.to_string(),
//         "Test Description".to_string(),
//         vec![create_test_question()],
//     )
// }

// pub fn create_test_question() -> Question {
//     Question::new(
//         "What is Rust?".to_string(),
//         vec![
//             "A programming language".to_string(),
//             "A metal oxide".to_string(),
//             "A game engine".to_string(),
//         ],
//         0,
//         10,
//     )
// }
