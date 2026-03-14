---
title: Tuple
description: Tuple schema validation with zod-rs
---

Tuple schemas validate fixed-length arrays where each element has a specific type.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = tuple()
    .item(string())
    .item(number());

assert!(schema.safe_parse(&json!(["hello", 42])).is_ok());
assert!(schema.safe_parse(&json!(["hello"])).is_err());       // too few
assert!(schema.safe_parse(&json!([42, "hello"])).is_err());    // wrong types
```

## Usage with enums

Tuples are used internally for enum variants with multiple fields:

```rust
#[derive(ZodSchema)]
enum Message {
    Coords(i32, i32),  // validated as {"Coords": [number, number]}
}
```

See the [Enums](/enums/overview/) section for more details.
