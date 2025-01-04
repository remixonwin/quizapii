# Design Decisions

## 1. Atomic Components

### Decision
Break down all features into atomic units with clear interfaces.

### Rationale
- Easier to test
- Reduced side effects
- Better maintainability
- Clearer boundaries

### Implementation
```rust
// Example of atomic component
pub trait QuizComponent {
    fn create(&self, quiz: Quiz) -> Result<QuizId, Error>;
    fn get(&self, id: QuizId) -> Result<Quiz, Error>;
    fn update(&self, quiz: Quiz) -> Result<Quiz, Error>;
    fn delete(&self, id: QuizId) -> Result<(), Error>;
}
```

### Current Implementation
```rust
pub trait QuizRepository {
    async fn create(&self, quiz: Quiz) -> Result<Quiz, AppError>;
    async fn get(&self, id: String) -> Result<Quiz, AppError>;
    // Additional methods as needed
}
```

## 2. Error Handling

### Decision
Use custom error types with context.

### Rationale
- Clear error boundaries
- Better error handling
- Improved debugging
- Type safety

### Implementation
```rust
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Operation failed: {0}")]
    OperationError(String),
}
```

### Current Implementation Examples
- AppError for centralized error handling
- Repository-specific error types
- HTTP status code mapping

## 3. Interface Design

### Decision
Use trait-based interfaces for all components.

### Rationale
- Clear contracts
- Easy testing
- Dependency inversion
- Mockability

### Current Examples
- TestQuizRepository for testing
- Trait-based repository pattern
- Async interfaces for future scalability

## 4. Testing Strategy

### Decision
Test at component boundaries with comprehensive coverage.

### Rationale
- Catch integration issues
- Verify contracts
- Ensure reliability
- Document behavior

### Decision
Use dedicated test utilities and common setup functions.

### Rationale
- Consistent test setup
- Reusable test components
- Reduced test code duplication
- Better maintenance

### Current Implementation
```rust
pub async fn setup_test_quiz() -> (TestQuizRepository, String) {
    // Implementation from common.rs
}

pub async fn create_multiple_quizzes(repo: &TestQuizRepository, count: usize) -> Vec<String> {
    // Implementation from common.rs
}
```

## 5. API Documentation

### Decision
Use OpenAPI/Swagger with utoipa.

### Rationale
- Standard API documentation
- Interactive testing interface
- Code-first approach
- Auto-generated documentation

### Implementation
```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::create_quiz,
        crate::api::get_quiz,
        crate::api::update_quiz,
        crate::api::delete_quiz,
    ),
    components(schemas(Quiz, QuizId, Error)),
    tags(
        (name = "quiz", description = "Quiz management endpoints")
    )
)]
pub struct ApiDoc;
```

## 6. Authentication

### Decision
Implement JWT-based authentication.

### Rationale
- Secure authentication
- Stateless sessions
- Scalability
- Standardized approach

### Implementation
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}
```
