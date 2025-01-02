# Quizmo

[![codecov](https://codecov.io/gh/yourusername/quizmo/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/quizmo)
[![CI](https://github.com/yourusername/quizmo/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/quizmo/actions/workflows/rust.yml)

A REST API for managing quizzes built with Rust using Axum and Sled DB.

## Features

- RESTful API with Axum
- Persistent storage using Sled DB
- JWT Authentication
- Comprehensive test suite
- API documentation with Swagger UI
- Async/await support
- Error handling with anyhow

## Getting Started

### Prerequisites

- Rust (stable and nightly toolchains)
- Cargo
- Docker (optional)

### Installation

1. Install Rust toolchains:

```bash
rustup install stable
rustup install nightly
rustup component add --toolchain nightly rust-docs
```

1. Clone and build:

```bash
git clone https://github.com/yourusername/quizmo.git
cd quizmo
cargo build
```

1. Run tests:

```bash
cargo test
cargo +nightly tarpaulin --doc # For coverage with doc tests
```

### Environment Variables

```env
DATABASE_URL=path/to/db
JWT_SECRET=your-secret-key
RUST_LOG=debug
```

## API Endpoints

- `POST /api/quizzes` - Create a new quiz
- `GET /api/quizzes` - List all quizzes
- `GET /api/quizzes/{id}` - Get a specific quiz
- `PUT /api/quizzes/{id}` - Update a quiz
- `DELETE /api/quizzes/{id}` - Delete a quiz

## API Documentation

### Authentication

All protected endpoints require a JWT token in the Authorization header:

```bash
Authorization: Bearer <token>
```

### Example Requests

Create a quiz:

```bash
curl -X POST http://localhost:3000/api/quizzes \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rust Quiz",
    "description": "Test your Rust knowledge",
    "questions": [
      {
        "text": "What is Rust?",
        "options": ["A language", "A metal"],
        "correct_answer": 0
      }
    ]
  }'
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage reporting
cargo +nightly tarpaulin
cargo +nightly tarpaulin --doc # Include doc tests

# Run specific test categories
cargo test --test integration
cargo test --test api
cargo test --test repository
```

### Coverage Gates

The project enforces strict coverage requirements:
- Core Logic: 90% minimum
- API Layer: 85% minimum
- Models: 80% minimum
- Critical Paths: 95% minimum

View detailed coverage metrics on [Codecov](https://codecov.io/gh/yourusername/quizmo).

### Code Coverage

Current coverage requirements:

- Minimum coverage: 80%
- Target coverage: 90%
- Critical paths: 95%

View the coverage report:

```bash
# Generate and view coverage report
cargo tarpaulin --out Html && open tarpaulin-report.html
```

### Test Organization

- `/tests/api/` - API endpoint tests
- `/tests/integration/` - Integration tests
- `/tests/repository/` - Repository tests
- `/tests/common/` - Shared test utilities

### Continuous Integration

All tests are automatically run on:

- Every push to main
- All pull requests
- Nightly builds

View the latest test results on the [Actions page](https://github.com/yourusername/quizmo/actions).

## License

[MIT License](LICENSE)
