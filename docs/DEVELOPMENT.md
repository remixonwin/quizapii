# Development Guide

## Setup

1. Required Tools:
```bash
rustup install stable
rustup install nightly
cargo install cargo-tarpaulin cargo-watch
```

2. Environment:
```bash
cp .env.example .env
# Edit .env with your settings
```

## Development Workflow

### Running Tests
```bash
# Watch mode
cargo watch -x test

# With coverage
cargo tarpaulin
```

### Code Organization

```
src/
├── api/        # API endpoints
├── models/     # Data structures
├── repository/ # Data access
└── services/   # Business logic
```

### Git Workflow

1. Branch naming:
- feature/description
- fix/description
- docs/description

2. Commit messages:
```
type: short description

Detailed explanation if needed
```
