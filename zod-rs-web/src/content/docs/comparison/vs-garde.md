---
title: zod-rs vs the garde Crate
description: Compare zod-rs with the garde crate for Rust data validation, with a migration guide for moving struct validation to runtime schemas.
---

[garde](https://github.com/jprochazk/garde) is a modern rewrite of the validator crate: you derive `Validate` on a struct, annotate fields with `#[garde(...)]` rules, and validate instances after deserializing them. zod-rs works one level earlier — it validates raw JSON values against schemas, so parsing and validation happen in one step.

## Feature comparison

| Feature | zod-rs | garde |
|---------|--------|-------|
| **Validation target** | Raw JSON values | Already-deserialized structs |
| **Schema definition** | Derive macro or runtime builders | Struct attributes only |
| **Runtime flexibility** | Build and compose schemas at runtime | Compile-time only |
| **Parse + validate** | One step from JSON | Deserialize first, then validate |
| **Error messages** | Full path context (`user.addresses[0].zip`) | Path-aware field errors |
| **Context-aware rules** | Custom `Schema` implementations | Built-in validation context |
| **TypeScript codegen** | Generates Zod schemas from Rust types | Not supported |
| **Framework integration** | Built-in Axum support | Via `axum_garde` and similar crates |
| **i18n** | Built-in localized error messages | Custom messages, no built-in locales |

## When garde is the better fit

garde is a good choice when your data is already a well-typed Rust struct and you only need to check invariants on it — especially if you want context-aware rules (validating a field against external state) with minimal ceremony. If that's your whole problem, garde solves it well.

zod-rs earns its place when validation starts at the JSON boundary: API request bodies, webhooks, config files, or anywhere you'd otherwise deserialize first and validate second. It also brings runtime-composable schemas and [TypeScript Zod codegen](/typescript-codegen/zod-ts/) for sharing rules with a frontend — neither of which fits garde's model.

## Migration guide

### Before: garde

```rust
use garde::Validate;

#[derive(Validate)]
struct User {
    #[garde(length(min = 3, max = 20))]
    username: String,

    #[garde(email)]
    email: String,

    #[garde(range(min = 13, max = 120))]
    age: u32,
}
```

### After: zod-rs

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema)]
struct User {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}
```

## What changes in practice

### Validation moves to the JSON boundary

```rust
// garde: deserialize, then validate the struct
let user: User = serde_json::from_value(json_data)?;
user.validate()?;

// zod-rs: validate the JSON and get the struct in one call
let user = User::validate_and_parse(&json_data)?;
```

With garde, malformed JSON fails in serde with a serde error, while invalid values fail later in garde with a garde error — two error shapes from one input. With zod-rs, both come back as one structured validation result.

### Schemas become values

```rust
// Build, store, and compose schemas at runtime — not possible with attribute-only validation
let schema = object()
    .field("name", string().min(2))
    .field("email", string().email());
```

See [Schema Composition](/advanced/schema-composition/) for reuse patterns.
