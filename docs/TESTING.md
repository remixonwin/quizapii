# Testing Guide

## Test Categories

### Unit Tests

Located in each module alongside the code:

```rust
#[cfg(test)]
mod tests {
    // Unit tests
}
```

### Integration Tests

Located in `/tests/integration/`:

- API endpoint tests
- Authentication flow tests
- Error handling tests

### Repository Tests

Located in `/tests/repository/`:

- CRUD operations
- Data persistence
- Concurrent access

## Running Tests

### Basic Test Run

```bash
cargo test
```

### Test with Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

### Documentation Tests

```bash
cargo test --doc
cargo +nightly tarpaulin --doc
```

## Coverage Requirements

| Component    | Minimum | Target |
|-------------|---------|---------|
| Core Logic  | 80%     | 90%     |
| API Layer   | 75%     | 85%     |
| Models      | 70%     | 80%     |
| Critical    | 95%     | 100%    |

## Writing Tests

### Test Structure

```rust
#[tokio::test]
async fn test_name() {
    // Arrange
    let ctx = TestContext::new().await;
    
    // Act
    let result = // ...
    
    // Assert
    assert_eq!(result, expected);
}
```

### Common Test Utilities

See `/tests/common/mod.rs` for shared test utilities:

- `TestContext`
- `create_test_quiz`
- `create_test_user`
