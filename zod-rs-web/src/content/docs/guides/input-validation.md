---
title: Rust Input Validation Guide
description: Learn how to validate user input in Rust — enforce rules on strings, numbers, emails, and nested data at every boundary from CLI arguments to API requests.
head:
  - tag: script
    attrs:
      type: application/ld+json
    content: |
      {
        "@context": "https://schema.org",
        "@type": "HowTo",
        "name": "How to Validate User Input in Rust",
        "description": "A step-by-step guide to validating strings, numbers, objects, and nested data in Rust using the zod-rs validation library.",
        "step": [
          {
            "@type": "HowToStep",
            "name": "Add zod-rs to your project",
            "text": "Add zod-rs = \"1.1\" to your Cargo.toml dependencies."
          },
          {
            "@type": "HowToStep",
            "name": "Define a validation schema",
            "text": "Build a schema from primitives like string(), number(), and object(), or derive one from a Rust struct with #[derive(ZodSchema)]."
          },
          {
            "@type": "HowToStep",
            "name": "Validate external input",
            "text": "Call safe_parse() on raw JSON or validate_and_parse() on a struct to enforce every rule and get either valid typed data or a list of errors with full field paths."
          },
          {
            "@type": "HowToStep",
            "name": "Handle validation errors",
            "text": "Inspect the returned errors to report each failing field and its violation to the caller — as an HTTP 422 response, a CLI error message, or a log entry."
          }
        ]
      }
---

Input validation in Rust ensures data from external sources — HTTP requests, CLI arguments, configuration files, message queues — meets your application's rules before processing. While Rust's type system catches structural errors at compile time, runtime input validation with zod-rs enforces business rules like email format, string length, numeric ranges, and custom constraints.

This guide covers every common input validation pattern in Rust, from simple string checks to deeply nested objects, with ready-to-use code examples.

## Why Rust's type system isn't enough

Rust's ownership and type system are the strongest in any mainstream language. A `String` is always valid UTF-8, a `u32` is always non-negative, and an `Option<T>` forces you to handle the absent case. But types alone cannot enforce *business rules*:

```rust
struct User {
    email: String,    // any string — "not-an-email" compiles fine
    age: u32,         // any u32 — 999999 is technically valid
    username: String, // empty string? 10000 characters? no check
}
```

When data crosses a trust boundary — arriving over HTTP, read from a file, or parsed from command-line arguments — you need runtime validation on top of compile-time types. That is what zod-rs provides.

## Where input validation belongs

Validate **at the boundary**, not deep inside business logic:

```
External world  →  [ Boundary validation ]  →  Trusted typed data
                         ↑
                   Validate here, once.
                   After this point, trust the types.
```

This pattern keeps your core logic clean. Every function past the boundary receives data that has already been checked — no scattered `if email.contains('@')` guards buried in service layers.

## String validation

The [string schema](/primitives/string/) covers the most common text rules:

```rust
use zod_rs::prelude::*;

// Length constraints
let username = string().min(3).max(20);

// Email format
let email = string().email();

// URL format
let website = string().url();

// Regex pattern
let slug = string().regex(r"^[a-z0-9-]+$");

// Prefix and suffix
let s3_key = string().starts_with("uploads/");
let image = string().ends_with(".png");

// Substring check
let bio = string().contains("rust");
```

Each rule produces a clear error message. A string failing `min(3)` reports `Too small: expected string to have >= 3 characters`, not a generic "invalid" message.

### Combining string rules

Rules chain naturally:

```rust
let password = string()
    .min(8)
    .max(128)
    .regex(r"[A-Z]")   // at least one uppercase
    .regex(r"[0-9]");   // at least one digit
```

Every rule runs independently — a password that is too short *and* missing digits reports both issues, not just the first.

## Number validation

The [number schema](/primitives/number/) validates numeric values:

```rust
// Range
let age = number().min(13.0).max(120.0);

// Integer only (reject 3.14)
let quantity = number().int();

// Positive values
let price = number().positive();

// Non-negative (zero allowed)
let stock = number().min(0.0);

// Combine rules
let port = number().int().min(1.0).max(65535.0);
```

## Object and struct validation

The [object schema](/complex-types/object/) validates JSON objects with typed fields:

```rust
let user_schema = object()
    .field("username", string().min(3).max(20))
    .field("email", string().email())
    .field("age", number().min(13.0).max(120.0).int());
```

### Nested objects

Objects nest arbitrarily deep — and every error carries the full path:

```rust
let schema = object()
    .field("user", object()
        .field("profile", object()
            .field("email", string().email())
            .field("bio", string().max(500))
        )
        .field("settings", object()
            .field("theme", string().min(1))
        )
    );

// Error on a nested field reports: user.profile.email: Invalid email address
```

### Derive macro for structs

For struct-heavy codebases, derive the schema instead of writing it by hand:

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema, serde::Deserialize, Debug)]
struct Profile {
    #[zod(email)]
    email: String,

    #[zod(max_length(500))]
    bio: String,
}

#[derive(ZodSchema, serde::Deserialize, Debug)]
struct CreateUser {
    #[zod(min_length(3), max_length(20))]
    username: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    profile: Profile, // nested validation is automatic
}

// Validate and deserialize in one call
let user = CreateUser::validate_and_parse(&json_data)?;
```

Nested structs that derive `ZodSchema` are validated recursively. See [nested struct validation](/derive-macros/nested-structs/) for the details.

## Array validation

The [array schema](/complex-types/array/) validates collections with element-level and length rules:

```rust
// Array of valid emails, 1 to 10 items
let emails = array(string().email()).min(1).max(10);

// Array of objects
let items = array(
    object()
        .field("name", string().min(1))
        .field("quantity", number().int().positive())
).min(1);

// Errors include the index: [2].quantity: Expected positive number
```

## Optional and nullable fields

The [optional schema](/complex-types/optional/) handles fields that may be absent or null:

```rust
// Field may be missing from the JSON
object().field("nickname", string().min(1).optional())

// In a derive macro
#[derive(ZodSchema, serde::Deserialize)]
struct UpdateUser {
    #[zod(min_length(1))]
    nickname: Option<String>,  // optional automatically
}
```

When the field is present, every rule applies. When absent or null, validation passes.

## Validating CLI arguments

Command-line tools receive untrusted input too. Parse arguments into JSON values and validate:

```rust
use serde_json::json;
use zod_rs::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let schema = object()
        .field("port", number().int().min(1.0).max(65535.0))
        .field("host", string().min(1));

    let port: f64 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let host = args.get(2)
        .cloned()
        .unwrap_or_default();

    let input = json!({ "port": port, "host": host });

    match schema.safe_parse(&input) {
        Ok(_) => println!("Starting server on {}:{}", host, port as u16),
        Err(errors) => {
            eprintln!("Invalid arguments:\n{}", errors);
            std::process::exit(1);
        }
    }
}
```

## Validating API request bodies

Web services are the most common boundary. With the `axum` feature flag, zod-rs validates request bodies in the extractor layer — invalid input never reaches your handler:

```rust
use zod_rs::prelude::*;

#[derive(ZodSchema, serde::Deserialize)]
struct CreateOrder {
    #[zod(min_length(1))]
    product_id: String,

    #[zod(min(1.0), int)]
    quantity: u32,

    #[zod(email)]
    buyer_email: String,
}

async fn create_order(
    ZodValid(order): ZodValid<CreateOrder>,
) -> impl IntoResponse {
    // `order` is already validated — trust the types here
    Json(json!({ "status": "created" }))
}
```

Invalid requests get a structured 422 response with every failing field and its error. See the full [Axum integration guide](/integrations/axum/) for setup instructions.

## Validating configuration files

Config files deserve the same validation rigor as API inputs:

```rust
use zod_rs::prelude::*;
use std::fs;

#[derive(ZodSchema, serde::Deserialize, Debug)]
struct AppConfig {
    #[zod(min(1.0), max(65535.0), int)]
    port: u32,

    #[zod(min_length(1))]
    database_url: String,

    #[zod(min(1.0), max(3600.0))]
    timeout_seconds: f64,

    #[zod(min_length(1))]
    allowed_origins: Vec<String>,
}

fn load_config(path: &str) -> Result<AppConfig, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("Cannot read config: {}", e))?;

    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    AppConfig::validate_and_parse(&json)
        .map_err(|e| format!("Config validation failed:\n{}", e))
}
```

Every validation error names the exact field — `timeout_seconds: Expected number <= 3600` — so operators know what to fix without reading source code. For more on parsing JSON, see [How to Validate JSON in Rust](/guides/validate-json/).

## Error handling and reporting

Every validation error in zod-rs carries a path, a code, and a human-readable message:

```rust
match schema.safe_parse(&data) {
    Ok(value) => handle_valid(value),
    Err(errors) => {
        for issue in &errors.issues {
            eprintln!("Field '{}': {}", issue.path.join("."), issue.message);
        }
    }
}
```

Errors compose across nesting levels. A form with three invalid fields reports all three, not just the first. See [Error Handling](/advanced/error-handling/) for the full error model, including error codes and programmatic inspection.

For internationalized error messages — useful when validation errors are shown directly to end users — see [i18n support](/advanced/i18n/).

## Best practice: validate once, trust the types after

The strongest validation pattern in Rust is:

1. **At the boundary:** validate every external input with a schema.
2. **After the boundary:** work exclusively with the validated struct.
3. **Never re-validate** the same data deeper in the call stack.

```rust
// Boundary: validate and parse
let order = CreateOrder::validate_and_parse(&raw_json)?;

// Business logic: trust the types
process_order(order);  // order.quantity is guaranteed to be >= 1
send_confirmation(order.buyer_email);  // guaranteed to be a valid email
```

This approach means your core domain code stays clean, testable, and free of defensive checks. The type system carries the guarantee forward from the boundary.

## Next steps

- [How to Validate JSON in Rust](/guides/validate-json/) — a deeper dive into JSON-specific patterns
- [How to Validate Structs in Rust](/guides/validate-structs/) — struct-first workflows with derive macros
- [Axum Integration](/integrations/axum/) — validate HTTP request bodies automatically
- [Error Handling](/advanced/error-handling/) — the full error model
- [Schema Composition](/advanced/schema-composition/) — reusable schema building blocks
