---
title: ZodSchema Derive
description: Automatically generate validation schemas from Rust structs
---

The `ZodSchema` derive macro generates validation schemas from Rust structs, making it easy to validate JSON data and deserialize it in one step.

## Basic usage

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(1), max_length(10))]
    interests: Vec<String>,

    bio: Option<String>,

    #[zod(nonnegative)]
    score: f64,

    is_active: bool,
}
```

## Generated methods

The derive macro generates these methods on your struct:

### `schema()` — get the validation schema

```rust
let schema = User::schema();
let result = schema.validate(&data);
```

### `validate_and_parse(value)` — validate and deserialize

Validates the JSON value against the schema and deserializes it into the struct:

```rust
let data = json!({
    "username": "alice_dev",
    "email": "alice@example.com",
    "age": 28,
    "interests": ["rust", "programming"],
    "score": 95.5,
    "is_active": true
});

match User::validate_and_parse(&data) {
    Ok(user) => println!("Valid user: {:?}", user),
    Err(e) => println!("Invalid: {}", e),
}
```

### `from_json(json_str)` — validate and parse from JSON string

```rust
let user = User::from_json(r#"{"username":"alice","email":"alice@example.com","age":28,"interests":["rust"],"score":95.5,"is_active":true}"#)?;
```

### `validate_json(json_str)` — validate JSON string (returns Value)

```rust
let value = User::validate_json(r#"{"username":"alice",...}"#)?;
```

## Type mapping

The derive macro maps Rust types to zod-rs schemas:

| Rust Type | Schema |
|-----------|--------|
| `String` | `string()` |
| `f32`, `f64` | `number()` |
| `i8`..`i64`, `u8`..`u64` | `number().int()` |
| `bool` | `boolean()` |
| `Vec<T>` | `array(T::schema())` |
| `Option<T>` | `optional(T::schema())` |
| Nested struct with `ZodSchema` | `T::schema()` |
