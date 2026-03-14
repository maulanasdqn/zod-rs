---
title: Object
description: Object schema validation with zod-rs
---

Create an object schema with `object()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = object()
    .field("name", string())
    .field("age", number());

let data = json!({"name": "Alice", "age": 25});
assert!(schema.safe_parse(&data).is_ok());
```

## Methods

### `field(name, schema)` — required field

```rust
let schema = object()
    .field("name", string())
    .field("email", string().email());
```

### `optional_field(name, schema)` — optional field

```rust
let schema = object()
    .field("name", string())
    .optional_field("bio", string());

let data = json!({"name": "Alice"});
assert!(schema.safe_parse(&data).is_ok());
```

### `strict()` — no additional properties

By default, objects allow extra fields. Use `strict()` to reject them.

```rust
let schema = object()
    .field("name", string())
    .strict();

let data = json!({"name": "Alice", "extra": true});
assert!(schema.safe_parse(&data).is_err()); // UnrecognizedKeys error
```

## Nested objects

Objects can be nested:

```rust
use zod_rs::prelude::*;
use serde_json::json;

fn address_schema() -> impl Schema<Value> {
    object()
        .field("street", string().min(1))
        .field("city", string().min(1))
        .field("country", string().length(2))
        .field("zip", string().regex(r"^\d{5}(-\d{4})?$"))
}

fn user_schema() -> impl Schema<Value> {
    object()
        .field("name", string())
        .field("email", string().email())
        .field("address", address_schema())
        .optional_field("billing_address", address_schema())
}

let data = json!({
    "name": "John Doe",
    "email": "john@example.com",
    "address": {
        "street": "123 Main St",
        "city": "Boston",
        "country": "US",
        "zip": "02101"
    }
});

assert!(user_schema().safe_parse(&data).is_ok());
```
