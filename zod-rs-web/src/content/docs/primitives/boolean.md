---
title: Boolean Validation
description: Validate boolean values in Rust with the zod-rs boolean() schema.
---

Create a boolean schema with `boolean()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = boolean();
assert!(schema.safe_parse(&json!(true)).is_ok());
assert!(schema.safe_parse(&json!(false)).is_ok());
assert!(schema.safe_parse(&json!("true")).is_err());
assert!(schema.safe_parse(&json!(1)).is_err());
```

Boolean schemas accept only JSON `true` or `false` values. Strings like `"true"` and numbers like `1` are rejected.

## Derive macro usage

```rust
#[derive(ZodSchema)]
struct Settings {
    is_active: bool,
    notifications_enabled: bool,
}
```
