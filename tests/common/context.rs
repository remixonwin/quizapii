use quizmo::models::test_types::{TestQuiz, TestUser};
use quizmo::repository::quiz_repository::QuizRepositoryImpl;
use quizmo::handlers; // Add handlers import
use reqwest::Response;
use tempfile::TempDir;
use axum::Router;
use reqwest::Client;
use std::sync::Arc;
use super::TestQuizRepository;

pub struct TestContext {
    pub app: Router,
    pub api_client: Client,
    pub base_url: String,
    pub repo: QuizRepositoryImpl,
    pub _temp_dir: TempDir,
}

impl TestContext {
    pub async fn new() -> Self {
        let repo = Arc::new(TestQuizRepository::new());
        let app = handlers::create_app(repo);

        let client = reqwest::Client::new();
        let base_url = "http://localhost:3000".to_string();
        let temp_dir = TempDir::new().unwrap();
        let repo = QuizRepositoryImpl::new_with_path(temp_dir.path()).unwrap();

        Self {
            app,
            api_client: client,
            base_url,
            repo,
            _temp_dir: temp_dir,
        }
    }

    pub fn app(&self) -> Router {
        self.app.clone()
    }

    pub async fn create_test_quiz(&self, quiz: &TestQuiz) -> reqwest::Result<Response> {
        self.api_client
            .post(format!("{}/api/v1/quizzes", self.base_url))
            .json(quiz)
            .send()
            .await
    }

    #[allow(dead_code)]
    pub async fn get_quiz(&self, id: impl AsRef<str>) -> reqwest::Result<Response> {
        self.api_client
            .get(format!("{}/api/v1/quizzes/{}", self.base_url, id.as_ref()))
            .send()
            .await
    }

    #[allow(dead_code)]
    pub async fn register_user(&self, test_user: &TestUser) -> Result<Response, reqwest::Error> {
        self.api_client
            .post(format!("{}/auth/register", &self.base_url))
            .json(test_user)
            .send()
            .await
    }

    #[allow(dead_code)]
    pub async fn login_user(&self, test_user: &TestUser) -> Result<Response, reqwest::Error> {
        self.api_client
            .post(format!("{}/auth/login", &self.base_url))
            .json(test_user)
            .send()
            .await
    }
}
