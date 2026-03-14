---
title: "Null"
description: Null schema validation with zod-rs
---

The null schema validates that a value is JSON `null`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = null();
assert!(schema.safe_parse(&json!(null)).is_ok());
assert!(schema.safe_parse(&json!("")).is_err());
assert!(schema.safe_parse(&json!(0)).is_err());
```

## Use with unions

Null is useful in union types to represent nullable values:

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = union()
    .variant(string())
    .variant(null());

assert!(schema.safe_parse(&json!("hello")).is_ok());
assert!(schema.safe_parse(&json!(null)).is_ok());
assert!(schema.safe_parse(&json!(42)).is_err());
```
