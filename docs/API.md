# API Documentation

## Endpoints

### Quizzes

#### Create Quiz
```http
POST /api/v1/quizzes
Content-Type: application/json

{
  "title": "Test Quiz",
  "description": "Test Description",
  "questions": []
}
```

#### Get Quiz
```http
GET /api/v1/quizzes/{id}
```

#### List Quizzes
```http
GET /api/v1/quizzes
```

## Authentication

All protected endpoints require Bearer token:

#### Register
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

## Error Responses

All errors follow the format:
```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

Common status codes:
- 200: Success
- 201: Created
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error
