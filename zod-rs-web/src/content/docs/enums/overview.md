---
title: Enum Support
description: Validating Rust enums with zod-rs
---

zod-rs fully supports Rust enums with the `ZodSchema` derive macro. Enums use serde's default externally-tagged format.

## Unit variants

```rust
use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
enum Status {
    Active,
    Inactive,
    Pending,
}

let status = json!({"Active": null});
assert!(Status::validate_and_parse(&status).is_ok());
```

## Tuple variants

### Single value

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
enum Message {
    Text(String),
    Number(i32),
}

let msg = json!({"Text": "hello"});
assert!(Message::validate_and_parse(&msg).is_ok());
```

### Multiple values

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
enum Message {
    Text(String),
    Number(i32),
    Coords(i32, i32),
}

let coords = json!({"Coords": [10, 20]});
assert!(Message::validate_and_parse(&coords).is_ok());
```

## Struct variants

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
enum Event {
    Click { x: i32, y: i32 },
    Scroll { delta: f64 },
}

let event = json!({"Click": {"x": 100, "y": 200}});
assert!(Event::validate_and_parse(&event).is_ok());
```

## Mixed variants

Enums can mix all variant types:

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
enum ApiResponse {
    Success,
    Data(String),
    Error { code: i32, message: String },
}
```
