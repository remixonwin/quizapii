# API Documentation 

## Authentication

All endpoints except /health require JWT authentication:

```http
Authorization: Bearer <token>
```

## Endpoints

### Quiz Management

#### Create Quiz
`POST /api/v1/quizzes`

Request:
```json
{
  "title": "String",
  "description": "String",
  "questions": [
    {
      "text": "String",
      "options": ["String"],
      "correct_answer": 0,
      "points": 10
    }
  ]
}
```

Response (201 Created):
```json
{
  "id": "uuid",
  "title": "String",
  // ...remaining fields
}
```

### Error Responses

All errors follow format:
```json
{
  "error": {
    "message": "String",
    "status": 400
  }
}
```
