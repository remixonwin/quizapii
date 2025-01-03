# Testing Guidelines

## Test Structure

### Common Test Utilities
Located in `tests/common.rs`:
- `setup_test_quiz()`: Creates a test quiz and repository
- `create_multiple_quizzes()`: Bulk quiz creation for tests

## Test Categories

1. Unit Tests
   - Repository implementations
   - Model validations
   - Error handling

2. Integration Tests
   - API endpoints
   - Authentication flow
   - Database operations

3. Component Tests
   - Repository layer
   - Service layer
   - Handler layer

## Best Practices

### Setup
```rust
// Use common test utilities
let (repo, quiz_id) = setup_test_quiz().await;
```

### Test Organization
- Group related tests in modules
- Use descriptive test names
- Include happy and error paths
- Test edge cases

### Assertions
- Verify expected outcomes
- Check error conditions
- Validate state changes

## Coverage Requirements

- Minimum 80% line coverage
- 100% coverage for critical paths
- All error paths tested
- All API endpoints tested

## Running Tests

Execute integration tests:
```bash
cargo test integration_tests
```

Run specific integration test:
```bash
cargo test tests::integration_tests
```

## Test Coverage

First install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Then run coverage:
```bash
# Basic coverage report
cargo tarpaulin

# With HTML output
cargo tarpaulin --out Html

# Coverage including doc tests
cargo tarpaulin --doc

# Coverage for specific tests
cargo tarpaulin --test lib
```

## Test Coverage Goals

### Repository Layer (Target: 95%)
- CRUD operations
- Error handling
- Concurrent operations
- Edge cases

### Handler Layer (Target: 90%)
- Request validation
- Response formatting
- Error handling
- Authentication/Authorization

### Integration Tests (Target: 85%)
- End-to-end flows
- Error scenarios
- Concurrent operations
- Performance benchmarks

## Test Patterns
1. Use common test helpers for setup
2. Test both success and failure cases
3. Include concurrent operation tests
4. Verify error handling

## Current Coverage

Current test coverage metrics:
- Overall: ~60%
- Core modules: ~80%
- Integration tests: ~40%

### Areas for Improvement

1. Add more integration tests covering:
   - Quiz CRUD operations
   - Error handling scenarios
   - Edge cases
2. Increase unit test coverage for:
   - Handler functions
   - Model validations
   - Repository operations