use quizmo::models::test_types::{TestQuiz, TestUser};
use quizmo::repository::quiz_repository::QuizRepositoryImpl;
use reqwest::Response;
use tempfile::TempDir;


pub struct TestContext {
    pub api_client: reqwest::Client,
    pub base_url: String,
    pub repo: QuizRepositoryImpl,
    pub _temp_dir: TempDir,
}

impl TestContext {
    pub async fn new() -> Self {
        let client = reqwest::Client::new();
        let base_url = "http://localhost:3000".to_string();
        let temp_dir = TempDir::new().unwrap();
        let repo = QuizRepositoryImpl::new_with_path(&temp_dir.path().to_path_buf()).unwrap();

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
            .json(quiz)
            .send()
            .await
    }

    pub async fn get_quiz(&self, id: impl AsRef<str>) -> reqwest::Result<Response> {
        self.api_client
            .get(format!("{}/api/v1/quizzes/{}", self.base_url, id.as_ref()))
            .send()
            .await
    }

    pub async fn register_user(&self, test_user: &TestUser) -> reqwest::Result<Response> {
        self.api_client
            .post(&format!("{}/auth/register", &self.base_url))
            .json(test_user)
            .send()
            .await
    }

    pub async fn login_user(&self, test_user: &TestUser) -> reqwest::Result<Response> {
        self.api_client
            .post(&format!("{}/auth/login", &self.base_url))
            .json(test_user)
            .send()
            .await
    }
}
