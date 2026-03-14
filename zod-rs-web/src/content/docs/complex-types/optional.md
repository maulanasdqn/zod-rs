---
title: Optional
description: Optional value validation with zod-rs
---

Create an optional schema with `optional(schema)` or `.optional()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

// Function form
let schema = optional(string());
assert!(schema.safe_parse(&json!(null)).is_ok());
assert!(schema.safe_parse(&json!("hello")).is_ok());

// Method chaining
let schema = string().optional();
assert!(schema.safe_parse(&json!(null)).is_ok());
```

Optional schemas accept `null` in addition to the wrapped type. Non-null values are validated against the inner schema.

```rust
let schema = optional(string().email());
assert!(schema.safe_parse(&json!(null)).is_ok());
assert!(schema.safe_parse(&json!("user@example.com")).is_ok());
assert!(schema.safe_parse(&json!("not-email")).is_err());
```

## With objects

Use `optional_field` on object schemas for optional fields:

```rust
let schema = object()
    .field("name", string())
    .optional_field("bio", string());
```

## Derive macro usage

`Option<T>` fields are automatically treated as optional:

```rust
#[derive(ZodSchema)]
struct UserProfile {
    name: String,
    bio: Option<String>,       // automatically optional
    avatar: Option<String>,    // automatically optional
}
```
