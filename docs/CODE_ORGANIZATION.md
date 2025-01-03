# Code Organization

## Project Structure

```
src/
├── domain/           # Business logic
│   ├── entities/     # Core types
│   ├── interfaces/   # Trait definitions
│   └── services/     # Business operations
├── infrastructure/   # External services
├── presentation/     # API layer
└── utils/            # Shared utilities

tests/
├── unit/             # Unit tests
├── integration/      # Integration tests
└── property/         # Property-based tests
```

## Component Guidelines

### 1. Interface Definition
```rust
pub trait Component {
    // Required methods
    fn operation(&self) -> Result<Output, Error>;
    
    // Optional methods with default implementations
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
```

### 2. Implementation Pattern
```rust
pub struct ComponentImpl<D> {
    dependencies: D,
}

impl<D: Dependencies> Component for ComponentImpl<D> {
    fn operation(&self) -> Result<Output, Error> {
        // Implementation
    }
}
```

### 3. Testing Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_behavior() {
        // Given
        let component = setup_component();
        
        // When
        let result = component.operation();
        
        // Then
        assert!(result.is_ok());
    }
}
```
