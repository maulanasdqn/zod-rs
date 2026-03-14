---
title: Getting Started
description: Install zod-rs and write your first schema
---

## Installation

Add zod-rs to your `Cargo.toml`:

```toml
[dependencies]
zod-rs = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Feature flags

| Feature | Description |
|---------|-------------|
| `axum` | Axum web framework integration |
| `ts` | TypeScript Zod schema generation |

```toml
# With Axum integration
zod-rs = { version = "0.4", features = ["axum"] }

# With TypeScript codegen
zod-rs = { version = "0.4", features = ["ts"] }
```

## Quick start

```rust
use serde_json::json;
use zod_rs::prelude::*;

fn main() {
    // Define a schema
    let user_schema = object()
        .field("name", string().min(2).max(50))
        .field("email", string().email())
        .field("age", number().min(0.0).max(120.0).int());

    // Validate data
    let user_data = json!({
        "name": "Alice",
        "email": "alice@example.com",
        "age": 25
    });

    match user_schema.safe_parse(&user_data) {
        Ok(validated_data) => println!("Valid: {:?}", validated_data),
        Err(errors) => println!("Invalid: {}", errors),
    }
}
```

## Schema methods

All schemas support these methods:

### `safe_parse(value)` — parse with Result

```rust
let schema = string();
match schema.safe_parse(&json!("hello")) {
    Ok(value) => println!("Valid: {}", value),
    Err(errors) => println!("Invalid: {}", errors),
}
```

### `parse(value)` — parse with panic on error

```rust
let schema = string();
let result = schema.parse(&json!("hello")); // Panics on failure
```

### `validate(value)` — alias for safe_parse

```rust
let schema = string();
let result = schema.validate(&json!("hello"));
```

## Using derive macros

For struct-based validation, use the `ZodSchema` derive macro:

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}

let data = json!({
    "username": "alice_dev",
    "email": "alice@example.com",
    "age": 28
});

let user = User::validate_and_parse(&data).unwrap();
```

## Workspace structure

The project is organized as a Cargo workspace:

| Crate | Description |
|-------|-------------|
| `zod-rs` | Main validation library with schema types |
| `zod-rs-macros` | Derive macros for `ZodSchema` |
| `zod-rs-ts` | TypeScript Zod schema generation |
| `zod-rs-util` | Utility functions, error handling, and i18n |

## Next steps

- Learn about [primitive types](/primitives/string/)
- Explore [complex types](/complex-types/object/)
- Set up [derive macros](/derive-macros/zod-schema/)
- Generate [TypeScript schemas](/typescript-codegen/zod-ts/)
