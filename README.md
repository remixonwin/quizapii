# Quizmo - Quiz Management API

A RESTful Quiz API service built with Rust, using sled for efficient local storage.

## Features

- RESTful API with Axum framework
- Local persistent storage with sled
- Quiz creation and management
- Async request handling
- Secure authentication
- OpenAPI documentation

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager

## Tech Stack

- **Framework**: Axum 0.6
- **Storage**: sled 0.34
- **Authentication**: JSON Web Tokens (JWT)
- **Documentation**: OpenAPI with utoipa
- **Validation**: validator
- **Error Handling**: anyhow

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/yourusername/quizmo.git
cd quizmo
```

1. Run the development server:

```bash
cargo run
```

The API will be available at `http://localhost:3000`

## API Routes

- `POST /api/quizzes` - Create a new quiz
- `GET /api/quizzes` - List all quizzes
- `GET /api/quizzes/{id}` - Get a specific quiz
- `PUT /api/quizzes/{id}` - Update a quiz
- `DELETE /api/quizzes/{id}` - Delete a quiz
- `POST /api/quizzes/{id}/submit` - Submit quiz answers

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Generate API documentation
cargo doc --open
```

## Project Structure

```plaintext
src/
├── handlers/      # Request handlers
├── models/        # Data models
└── repository/    # Database operations
```

## Data Models

### Quiz
- id: Unique identifier
- title: Quiz title
- description: Quiz description
- questions: Array of Question objects
- created_at: Timestamp
- updated_at: Timestamp

### Question
- id: Unique identifier
- text: Question text
- options: Array of possible answers
- correct_answer: Index of the correct answer
- points: Points awarded for correct answer

## Testing

The project includes both unit tests and integration tests:

### Unit Tests

Located in `src/lib.rs`:

- test_quiz_creation
- test_quiz_retrieval
- test_quiz_update
- test_quiz_deletion

### Integration Tests

Located in `tests/api_tests.rs`:

- test_authentication
- test_create_quiz_endpoint
- test_get_quiz_endpoint
- test_submit_quiz_answers

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=test cargo test

# Run specific test suite
cargo test --test api_tests
```

## License

MIT License
