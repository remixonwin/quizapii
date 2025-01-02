use serde::{Deserialize, Serialize};
use std::sync::Once;
use uuid::Uuid;
use quizmo::db::Database;
use actix_web::{
    App, web,
    dev::{Service, ServiceResponse},
    HttpResponse,
    http::Method,
    test,
    Error,
};
use actix_http::Request;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::pin::Pin;
use futures_util::Future;
use actix_web::body::MessageBody;

static INIT: Once = Once::new();

type TestAppService = impl Service<Request, Response = ServiceResponse, Error = Error> + 'static;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestQuiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<TestQuestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestQuestion {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUser {
    pub username: String,
    pub password: String,
}

pub fn setup() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        env_logger::init();
    });
}

pub async fn spawn_test_server() -> actix_web::dev::Server {
    // Test server implementation will go here
    unimplemented!()
}

pub struct TestContext {
    pub api_client: reqwest::Client,
    pub base_url: String,
    pub app: Arc<TestServiceWrapper<TestAppService>>,
    pub db: Database,
}

impl TestContext {
    pub async fn new() -> Self {
        // Initialize test environment
        dotenv::dotenv().ok();
        let db = Database::new_test_connection().expect("Failed to connect to test database");
        let app = create_test_app().await;
        
        Self { 
            api_client: reqwest::Client::new(),
            base_url: "http://localhost:3000".to_string(),
            app,
            db,
        }
    }

    pub async fn create_test_quiz(&self, quiz: &TestQuiz) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/api/v1/quizzes", self.base_url))
            .json(quiz)
            .send()
            .await
            .expect("Failed to create quiz")
    }

    pub async fn get_quiz(&self, id: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/v1/quizzes/{}", self.base_url, id))
            .send()
            .await
            .expect("Failed to get quiz")
    }

    pub async fn register_user(&self, user: &TestUser) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/api/v1/auth/register", self.base_url))
            .json(user)
            .send()
            .await
            .expect("Failed to register user")
    }

    pub async fn login_user(&self, user: &TestUser) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/api/v1/auth/login", self.base_url))
            .json(user)
            .send()
            .await
            .expect("Failed to login user")
    }

    pub async fn create_test_quiz_auth(&self, quiz: &TestQuiz, token: &str) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/api/v1/quizzes", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(quiz)
            .send()
            .await
            .expect("Failed to create quiz")
    }
}

async fn create_test_app() -> Arc<TestServiceWrapper<TestAppService>> {
    let db = Database::new_test_connection().unwrap();
    let db_data = web::Data::new(db);
    
    let app = App::new()
        .app_data(db_data.clone())
        .default_service(web::to(|| async { 
            HttpResponse::NotFound().finish()
        }));
    
    let service = test::init_service(app).await;
    Arc::new(TestServiceWrapper(service))
}

pub struct TestServiceWrapper<S>(S);

impl<S, B> Service<Request> for TestServiceWrapper<S>
where
    S: Service<Request, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse, Error>> + 'static>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&self, req: Request) -> Self::Future {
        let fut = self.0.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}

pub async fn make_request<'a, S>(
    service: &'a TestServiceWrapper<S>,
    method: &'a str,
    path: &'a str,
    payload: Vec<u8>,
) -> ServiceResponse 
where
    S: Service<Request, Response = ServiceResponse, Error = Error> + 'static,
{
    let req = test::TestRequest::with_uri(path)
        .method(Method::from_bytes(method.as_bytes()).unwrap())
        .set_payload(payload)
        .to_request();

    service
        .call(req)
        .await
        .expect("Failed to process request")
}

pub fn create_sample_quiz() -> TestQuiz {
    TestQuiz {
        id: Uuid::new_v4().to_string(),
        title: "Test Quiz".to_string(),
        description: "A test quiz".to_string(),
        questions: vec![
            TestQuestion {
                text: "What is Rust?".to_string(),
                options: vec![
                    "A programming language".to_string(),
                    "A metal oxide".to_string(),
                ],
                correct_answer: 0,
            }
        ],
    }
}

pub fn create_test_user() -> TestUser {
    TestUser {
        username: format!("test_user_{}", Uuid::new_v4()),
        password: "test_password_123".to_string(),
    }

}
