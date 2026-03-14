---
title: Error Handling
description: Understanding zod-rs validation errors
---

zod-rs provides detailed error information with full path tracking for nested structures.

## Basic error handling

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = string().email();

match schema.safe_parse(&json!("not-an-email")) {
    Ok(value) => println!("Valid: {}", value),
    Err(errors) => println!("Invalid: {}", errors),
}
```

## Path tracking

Errors in nested objects include the full field path:

```rust
let schema = object()
    .field("user", object()
        .field("name", string().min(2))
        .field("email", string().email())
    );

let data = json!({
    "user": {
        "name": "A",
        "email": "invalid-email"
    }
});

match schema.safe_parse(&data) {
    Err(errors) => {
        println!("{}", errors);
        // Output:
        //   - user.name: Too big: expected string to have >= 2 characters
        //   - user.email: Invalid email address
    }
    _ => {}
}
```

## Error types

| Error | Description |
|-------|-------------|
| `ValidationError::Required` | Missing required field |
| `ValidationError::InvalidType` | Wrong data type |
| `ValidationError::InvalidValue` | Value doesn't match expected value |
| `ValidationError::InvalidValues` | Value doesn't match any expected values |
| `ValidationError::TooSmall` | Value or length below minimum |
| `ValidationError::TooBig` | Value or length above maximum |
| `ValidationError::InvalidFormat` | String format validation failed (starts_with, ends_with, includes, regex) |
| `ValidationError::InvalidNumber` | Number constraint failed (finite, positive, etc.) |
| `ValidationError::UnrecognizedKeys` | Object has unrecognized keys (strict mode) |
| `ValidationError::InvalidUnion` | No union variant matched |
| `ValidationError::Custom` | Custom validation error |

## Accessing individual issues

```rust
match schema.safe_parse(&data) {
    Err(validation_result) => {
        for issue in &validation_result.issues {
            println!("Error: {}", issue);
        }
    }
    _ => {}
}
```

## Custom validation

Implement the `Schema` trait for custom validation logic:

```rust
use zod_rs::prelude::*;
use zod_rs_util::{ValidationError, ValidateResult};
use serde_json::Value;

struct MinWords {
    min: usize,
}

impl Schema<String> for MinWords {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        let s = value.as_str()
            .ok_or_else(|| ValidationError::invalid_type("string", "other"))?
            .to_string();

        let word_count = s.split_whitespace().count();
        if word_count < self.min {
            return Err(ValidationError::custom(
                format!("Must contain at least {} words", self.min)
            ).into());
        }

        Ok(s)
    }
}

let schema = MinWords { min: 3 };
assert!(schema.safe_parse(&json!("hello world rust")).is_ok());
assert!(schema.safe_parse(&json!("hello")).is_err());
```
