---
title: Schema Composition
description: Building complex schemas from reusable parts
---

zod-rs schemas are composable — you can build complex validation rules by combining simpler schemas.

## Reusable schema functions

```rust
use zod_rs::prelude::*;

fn email_schema() -> impl Schema<String> {
    string().email()
}

fn password_schema() -> impl Schema<String> {
    string().min(8).regex(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)")
}

fn login_schema() -> impl Schema<Value> {
    object()
        .field("email", email_schema())
        .field("password", password_schema())
}
```

## Shared field schemas

Extract common validation into reusable functions:

```rust
fn username() -> impl Schema<String> {
    string().min(3).max(20).regex(r"^[a-zA-Z0-9_]+$")
}

fn registration_schema() -> impl Schema<Value> {
    object()
        .field("username", username())
        .field("email", email_schema())
        .field("password", password_schema())
}

fn profile_update_schema() -> impl Schema<Value> {
    object()
        .field("username", username())
        .optional_field("bio", string().max(500))
}
```

## Nested composition

Compose object schemas for nested structures:

```rust
fn address_schema() -> impl Schema<Value> {
    object()
        .field("street", string().min(1))
        .field("city", string().min(1))
        .field("country", string().length(2))
        .field("zip", string().regex(r"^\d{5}(-\d{4})?$"))
}

fn order_schema() -> impl Schema<Value> {
    object()
        .field("id", string())
        .field("shipping_address", address_schema())
        .optional_field("billing_address", address_schema())
        .field("items", array(object()
            .field("name", string())
            .field("quantity", number().int().positive())
            .field("price", number().nonnegative())
        ))
}
```

## Dynamic schemas

Since schemas are built at runtime, you can create them conditionally:

```rust
fn user_schema(require_bio: bool) -> impl Schema<Value> {
    let mut schema = object()
        .field("name", string())
        .field("email", string().email());

    if require_bio {
        schema = schema.field("bio", string().min(10));
    } else {
        schema = schema.optional_field("bio", string());
    }

    schema
}
```
