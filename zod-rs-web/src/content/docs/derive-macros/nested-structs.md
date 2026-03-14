---
title: Nested Structs
description: Using ZodSchema with nested struct types
---

The `ZodSchema` derive macro automatically handles nested structs. When a field's type also derives `ZodSchema`, its schema is used for validation.

## Example

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Address {
    #[zod(min_length(5), max_length(200))]
    street: String,

    #[zod(min_length(2), max_length(50))]
    city: String,

    #[zod(length(2))]
    country_code: String,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct UserProfile {
    #[zod(min_length(2), max_length(50))]
    name: String,

    #[zod(email)]
    email: String,

    address: Option<Address>,
}
```

## How it works

When the derive macro encounters a field whose type implements `ZodSchema`, it calls `T::schema()` to get the nested validation schema. This happens automatically — no extra attributes are needed.

## Optional nested structs

Use `Option<T>` for optional nested objects:

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Order {
    #[zod(min_length(1))]
    id: String,

    shipping_address: Address,          // required
    billing_address: Option<Address>,   // optional
}
```

## Nested Vec

Vectors of structs work the same way:

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Team {
    name: String,

    #[zod(min_length(1))]
    members: Vec<UserProfile>,
}
```

## Validation errors

Errors in nested structs include the full path:

```rust
let data = json!({
    "name": "Alice",
    "email": "alice@example.com",
    "address": {
        "street": "1",
        "city": "B",
        "country_code": "USA"
    }
});

match UserProfile::validate_and_parse(&data) {
    Err(e) => println!("{}", e),
    // Output:
    //   - address.street: Too small: expected string length >= 5
    //   - address.country_code: Expected exactly 2 characters
    _ => {}
}
```
