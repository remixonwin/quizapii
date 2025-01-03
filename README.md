# Quizmo

A REST API for managing quizzes built with Rust using Axum and Sled DB.

## Current Status

- ✅ Basic API structure implemented with Axum
- ✅ Quiz CRUD operations defined
- ✅ Test infrastructure in place
- ✅ Repository pattern implemented
- ✅ Error handling implemented
- 🚧 Authentication (in progress)
- 🚧 Database integration (in progress)
- ❌ API documentation (planned)

## Project Structure

```
src/
├── handlers/     # API endpoint handlers
├── models/       # Data structures and types
├── repository/   # Data access layer
├── error/        # Error handling and types
└── lib.rs       # Core library exports
```

## Implemented Endpoints

- `POST /api/v1/quizzes` - Create a new quiz
- `GET /api/v1/quizzes/{id}` - Get a specific quiz
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - User login

## Testing

The project includes:
- Integration tests for API endpoints
- Repository tests
- Error handling tests
- Test utilities and helpers

Run tests with:

```bash
cargo test
```

## Getting Started

1. Clone the repository
2. Build the project:
```bash
cargo build
```
3. Run tests:
```bash
cargo test
```

## Development Status

Current focus areas:
- Implementing proper error handling
- Adding authentication middleware
- Completing database integration
- Expanding test coverage

## Error Handling

The application uses a custom `AppError` type that handles:
- Authentication errors
- Database errors
- Validation errors
- Serialization errors
- Not found errors
- Internal server errors

All errors are properly mapped to HTTP status codes:
- 400 Bad Request for validation errors
- 401 Unauthorized for authentication errors
- 404 Not Found for missing resources
- 500 Internal Server Error for database and system errors

## Contributing

The project is in early development. Please check the open issues before contributing.
## Test Section
