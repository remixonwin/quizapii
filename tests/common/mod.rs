use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // Initialize test environment
        dotenv::dotenv().ok();
        env_logger::init();
    });
}

pub struct TestContext {
    pub api_client: reqwest::Client,
    pub base_url: String,
}

impl TestContext {
    pub fn new() -> Self {
        setup();
        Self {
            api_client: reqwest::Client::new(),
            base_url: std::env::var("TEST_API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }
    }

    pub async fn create_test_quiz(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Implementation for creating a test quiz
        Ok("test_quiz_id".to_string())
    }
}
