---
title: Array
description: Array schema validation with zod-rs
---

Create an array schema with `array(element_schema)`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = array(string());
assert!(schema.safe_parse(&json!(["a", "b", "c"])).is_ok());
assert!(schema.safe_parse(&json!([1, 2, 3])).is_err());
```

## Validations

### `min(n)` — minimum length

```rust
let schema = array(string()).min(1);
assert!(schema.safe_parse(&json!(["a"])).is_ok());
assert!(schema.safe_parse(&json!([])).is_err());
```

### `max(n)` — maximum length

```rust
let schema = array(string()).max(5);
```

### `length(n)` — exact length

```rust
let schema = array(number()).length(3);
assert!(schema.safe_parse(&json!([1, 2, 3])).is_ok());
assert!(schema.safe_parse(&json!([1, 2])).is_err());
```

## Nested arrays

```rust
let schema = array(array(string()));
assert!(schema.safe_parse(&json!([["a", "b"], ["c", "d"]])).is_ok());
```

## Derive macro usage

```rust
#[derive(ZodSchema)]
struct Config {
    #[zod(min_length(1), max_length(10))]
    tags: Vec<String>,
}
```
