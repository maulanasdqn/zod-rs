---
title: Literal
description: Literal value validation with zod-rs
---

Create a literal schema with `literal(value)`. The value must match exactly.

## String literals

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = literal("active".to_string());
assert!(schema.safe_parse(&json!("active")).is_ok());
assert!(schema.safe_parse(&json!("inactive")).is_err());
```

## Number literals

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = literal(42.0);
assert!(schema.safe_parse(&json!(42.0)).is_ok());
assert!(schema.safe_parse(&json!(43.0)).is_err());
```

## Use with unions

Literals are commonly used with unions to create enum-like types:

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = union()
    .variant(literal("small".to_string()))
    .variant(literal("medium".to_string()))
    .variant(literal("large".to_string()));

assert!(schema.safe_parse(&json!("small")).is_ok());
assert!(schema.safe_parse(&json!("xl")).is_err());
```
