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