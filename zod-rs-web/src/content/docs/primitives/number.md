---
title: Number
description: Number schema validation with zod-rs
---

Create a number schema with `number()`.

```rust
use zod_rs::prelude::*;
use serde_json::json;

let schema = number();
assert!(schema.safe_parse(&json!(42.5)).is_ok());
assert!(schema.safe_parse(&json!("42")).is_err());
```

## Validations

### `min(n)` / `max(n)` — range constraints

```rust
let schema = number().min(0.0).max(100.0);
assert!(schema.safe_parse(&json!(50)).is_ok());
assert!(schema.safe_parse(&json!(-1)).is_err());
```

### `int()` — integer only

```rust
let schema = number().int();
assert!(schema.safe_parse(&json!(42)).is_ok());
assert!(schema.safe_parse(&json!(42.5)).is_err());
```

### `positive()` — must be > 0

```rust
let schema = number().positive();
assert!(schema.safe_parse(&json!(1)).is_ok());
assert!(schema.safe_parse(&json!(0)).is_err());
```

### `nonnegative()` — must be >= 0

```rust
let schema = number().nonnegative();
assert!(schema.safe_parse(&json!(0)).is_ok());
assert!(schema.safe_parse(&json!(-1)).is_err());
```

### `negative()` — must be < 0

```rust
let schema = number().negative();
assert!(schema.safe_parse(&json!(-1)).is_ok());
```

### `nonpositive()` — must be <= 0

```rust
let schema = number().nonpositive();
assert!(schema.safe_parse(&json!(0)).is_ok());
```

### `finite()` — excludes NaN and Infinity

```rust
let schema = number().finite();
assert!(schema.safe_parse(&json!(42.0)).is_ok());
```

## Derive macro usage

```rust
#[derive(ZodSchema)]
struct Product {
    #[zod(min(0.0), nonnegative)]
    price: f64,

    #[zod(min(0.0), max(1000.0), int)]
    quantity: u32,
}
```
