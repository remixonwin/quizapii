# Quizmo

A REST API for managing quizzes built with Rust using Axum and Sled DB.

## Features

- Create, read, update, and delete quizzes
- Persistent storage using Sled DB
- RESTful API endpoints
- Async/await support
- Error handling with anyhow

## Getting Started

### Prerequisites

- Rust (latest stable)
- Cargo

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/quizmo.git
cd quizmo
```

1. Build the project:

```bash
cargo build
```

1. Run the server:

```bash
cargo run
```

## API Endpoints

- `POST /api/quizzes` - Create a new quiz
- `GET /api/quizzes` - List all quizzes
- `GET /api/quizzes/{id}` - Get a specific quiz
- `PUT /api/quizzes/{id}` - Update a quiz
- `DELETE /api/quizzes/{id}` - Delete a quiz

## Testing

Run the test suite:

```bash
cargo test
```

## License

[MIT License](LICENSE)
