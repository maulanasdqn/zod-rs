---
title: vs Validator Crate
description: Comparing zod-rs to the validator crate, with migration guide
---

## Feature comparison

| Feature | zod-rs | validator |
|---------|--------|-----------|
| **Schema definition** | Derive macro with attributes | Struct attributes only |
| **Runtime flexibility** | Dynamic schema creation | Compile-time only |
| **Error messages** | Detailed with full path context | Basic field-level errors |
| **JSON integration** | Built-in JSON validation/parsing | Manual serde integration |
| **Nested validation** | Automatic nested struct support | Manual implementation |
| **Schema reuse** | Composable and reusable schemas | Struct-bound validation |
| **Type safety** | Full type inference | Limited type information |
| **Extensibility** | Custom validators and schemas | Custom validation functions |
| **Framework integration** | Built-in Axum support | Manual integration |
| **i18n** | Built-in localized error messages | No i18n support |

## Migration guide

### Before: validator crate

```rust
use validator::{Validate, ValidationError};

#[derive(Validate)]
struct User {
    #[validate(length(min = 3, max = 20))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 13, max = 120))]
    age: u32,
}
```

### After: zod-rs

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}
```

## What you gain

### Validate and parse in one step

```rust
// validator: validate then manually deserialize
let user = User { ... };
user.validate()?;

// zod-rs: validate and deserialize from JSON in one call
let user = User::validate_and_parse(&json_data)?;
```

### Parse from JSON strings directly

```rust
let user = User::from_json(r#"{"username":"alice","email":"alice@example.com","age":25}"#)?;
```

### Reusable schemas

```rust
// Create a schema once, use it many times
let schema = User::schema();
let is_valid = schema.validate(&data).is_ok();
```

### Dynamic schema creation

```rust
// Build schemas at runtime — not possible with validator
let schema = object()
    .field("name", string().min(2))
    .field("email", string().email());
```

### Full path error messages

```rust
// validator: "email: Invalid email"
// zod-rs:    "user.profile.email: Invalid email address"
```
