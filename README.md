# Quizmo - OpenAPI Quiz Service

A RESTful Quiz API service built with Rust, following OpenAPI specifications.

## Features

- OpenAPI 3.0 compliant REST API
- Quiz creation and management
- Question and answer handling
- Score tracking and analytics
- Secure authentication and authorization

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Docker (optional)

## Getting Started

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

The API will be available at `http://localhost:8080`

## API Documentation

The OpenAPI documentation is available at `/docs` endpoint when the server is running.

## Development

This project uses a dev container configuration for consistent development environments. To use it:

1. Install VS Code and the Remote - Containers extension
2. Open the project in VS Code
3. Click "Reopen in Container" when prompted

## Testing

Run the tests using:

```bash
cargo test
```

## License

MIT License
