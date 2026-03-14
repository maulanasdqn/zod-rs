---
title: Union
description: Union type validation with zod-rs
---

Create a union schema with `union()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = union()
    .variant(string())
    .variant(number());

assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!(42)).is_ok());
assert!(schema.safe_parse(&json!(true)).is_err());
```

The value must match at least one variant. Variants are tested in order.

## Literal unions

Use literal variants to create enum-like types:

```rust
use zod_rs::prelude::*;
use serde_json::json;

let size = union()
    .variant(literal("small".to_string()))
    .variant(literal("medium".to_string()))
    .variant(literal("large".to_string()));

assert!(size.safe_parse(&json!("medium")).is_ok());
assert!(size.safe_parse(&json!("xl")).is_err());
```

## Complex unions

Variants can be any schema type, including objects:

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = union()
    .variant(object()
        .field("type", literal("text".to_string()))
        .field("content", string()))
    .variant(object()
        .field("type", literal("image".to_string()))
        .field("url", string().url()));
```

## Error handling

When no variant matches, you get an `InvalidUnion` error listing all failed attempts.
