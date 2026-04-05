---
title: Rust Error Handling Patterns
date: 2026-03-28
tags:
  - rust
  - error-handling
  - patterns
---

# Rust Error Handling Patterns

## The `?` Operator

Use `?` for concise error propagation in functions that return `Result`.

```rust
fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```

## Custom Error Types

Define domain-specific errors with `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("config not found: {0}")]
    ConfigNotFound(String),
    #[error("parse failed at line {line}")]
    ParseError { line: usize },
}
```

## When to Use `unwrap()`

- Tests and prototypes only
- When the invariant is logically guaranteed
- Never in library code
