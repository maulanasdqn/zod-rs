---
title: String
description: String schema validation with zod-rs
---

Create a string schema with `string()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = string();
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!(42)).is_err());
```

## Validations

### `min(n)` — minimum length

```rust
let schema = string().min(3);
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!("hi")).is_err());
```

### `max(n)` — maximum length

```rust
let schema = string().max(10);
assert!(schema.safe_parse(&json!("hello")).is_ok());
```

### `length(n)` — exact length

```rust
let schema = string().length(5);
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!("hi")).is_err());
```

### `email()` — email format

```rust
let schema = string().email();
assert!(schema.safe_parse(&json!("user@example.com")).is_ok());
assert!(schema.safe_parse(&json!("not-an-email")).is_err());
```

### `url()` — URL format

```rust
let schema = string().url();
assert!(schema.safe_parse(&json!("https://example.com")).is_ok());
```

### `regex(pattern)` — regular expression

```rust
let schema = string().regex(r"^[a-zA-Z]+$");
assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!("hello123")).is_err());
```

### `starts_with(value)`

```rust
let schema = string().starts_with("hello");
assert!(schema.safe_parse(&json!("hello world")).is_ok());
```

### `ends_with(value)`

```rust
let schema = string().ends_with("world");
assert!(schema.safe_parse(&json!("hello world")).is_ok());
```

### `includes(value)`

```rust
let schema = string().includes("world");
assert!(schema.safe_parse(&json!("hello world")).is_ok());
```

## Chaining validations

Validations can be chained together:

```rust
let schema = string()
    .min(3)
    .max(50)
    .email();
```

## Derive macro usage

```rust
#[derive(ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,
}
```
