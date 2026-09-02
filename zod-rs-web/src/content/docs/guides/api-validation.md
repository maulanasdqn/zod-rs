---
title: API and Form Validation in Rust
description: Validate API request bodies, query parameters, and form submissions in Rust web applications — with Axum integration, structured error responses, and full field-path reporting.
---

To validate API requests and form submissions in Rust, define schemas with zod-rs and let the framework extractor reject invalid input before your handler runs. zod-rs validates raw JSON directly, reports the full path to each failing field, and integrates with Axum out of the box.

This guide walks through validating request bodies, query parameters, nested objects, and array fields in an Axum application. Every pattern here applies to any JSON boundary — webhooks, message queues, config files — but the examples focus on HTTP because that is where most Rust validation questions start.

## Why validation belongs in the extractor layer

Most Rust web tutorials scatter validation across handler bodies:

```rust
async fn register(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let name = body["name"].as_str().unwrap_or("");
    if name.len() < 2 {
        return (StatusCode::BAD_REQUEST, "name too short").into_response();
    }
    // ... ten more if-checks ...
}
```

This has three problems: the handler mixes business logic with input checking, errors are ad-hoc strings instead of a structured shape the client can parse, and nothing prevents a second handler from forgetting a check. Moving validation into an extractor solves all three — invalid input is rejected with a consistent error shape before the handler ever runs.

## Setting up zod-rs with Axum

Enable the `axum` feature in your `Cargo.toml`:

```toml
[dependencies]
zod-rs = { version = "1.1", features = ["axum"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

## Validating a request body

Define your struct with `ZodSchema` and annotate the fields with validation rules:

```rust
use zod_rs::prelude::*;
use serde::Deserialize;

#[derive(ZodSchema, Deserialize, Debug)]
struct CreateUser {
    #[zod(min_length(2), max_length(50))]
    name: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}
```

Then use the zod-rs extractor in your handler. The extractor validates the raw JSON body and either hands you a typed struct or returns a 422 response with every failing field and its full path:

```rust
use axum::{Router, routing::post};
use zod_rs::axum::ZodJson;

async fn create_user(ZodJson(user): ZodJson<CreateUser>) -> String {
    format!("Welcome, {}!", user.name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

If a client sends `{"name": "", "email": "bad", "age": 5}`, the response is a structured JSON error — not a bare string:

```json
{
  "errors": [
    { "path": "name", "message": "Too small: expected string to have >= 2 characters" },
    { "path": "email", "message": "Invalid email address" },
    { "path": "age", "message": "Too small: expected number to be >= 13" }
  ]
}
```

Every error includes the full path to the failing field, even inside nested data. See the [Axum integration guide](/integrations/axum/) for the full extractor API and custom rejection handling.

## Validating query parameters

Query parameters work the same way — define a struct, derive `ZodSchema`, and validate at the boundary. Build a runtime schema when derive macros don't fit:

```rust
use axum::{extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct ListParams {
    page: Option<u32>,
    per_page: Option<u32>,
    sort: Option<String>,
}

async fn list_users(Query(params): Query<ListParams>) -> String {
    let schema = object()
        .field("page", number().min(1.0).int().optional())
        .field("per_page", number().min(1.0).max(100.0).int().optional())
        .field("sort", string().min(1).optional());

    let data = serde_json::to_value(&params).unwrap();
    match schema.safe_parse(&data) {
        Ok(_) => format!("Listing page {}", params.page.unwrap_or(1)),
        Err(errors) => format!("Invalid query: {}", errors),
    }
}
```

This pattern is useful when the validation rules come from configuration or differ per tenant — something attribute-only validators cannot express. See [Schema Composition](/advanced/schema-composition/) for more on building schemas at runtime.

## Nested object validation

Real API payloads are rarely flat. A shipping address nested inside an order validates automatically when both types derive `ZodSchema`:

```rust
#[derive(ZodSchema, Deserialize, Debug)]
struct Address {
    #[zod(min_length(1))]
    street: String,

    #[zod(min_length(1))]
    city: String,

    #[zod(min_length(2), max_length(2))]
    country_code: String,
}

#[derive(ZodSchema, Deserialize, Debug)]
struct CreateOrder {
    #[zod(min_length(1))]
    product_id: String,

    shipping: Address,
}
```

If `country_code` is empty, the error path reads `shipping.country_code` — the client knows exactly which field in which object failed. Learn more about nesting in the [Object](/complex-types/object/) and [Nested Structs](/derive-macros/nested-structs/) documentation.

## Validating array fields

Form submissions often include repeating fields — tags, line items, file references. Validate them with array schemas:

```rust
#[derive(ZodSchema, Deserialize, Debug)]
struct BlogPost {
    #[zod(min_length(5), max_length(200))]
    title: String,

    #[zod(min_length(50))]
    body: String,

    // Between 1 and 10 tags, each at least 2 characters
    #[zod(min_items(1), max_items(10))]
    tags: Vec<String>,
}
```

Validation errors inside arrays include the index: `tags[3]: Too small: expected string to have >= 2 characters`. This lets the client highlight the exact field in a form. See the [Array](/complex-types/array/) reference for all array constraints.

## Custom error formatting

The default error shape works for most APIs, but you may need to match an existing error contract. Access the raw error issues and format them however you like:

```rust
use axum::response::{IntoResponse, Json, Response};
use axum::http::StatusCode;

async fn create_user_custom(
    body: axum::extract::Json<serde_json::Value>,
) -> Response {
    let schema = CreateUser::schema();
    match schema.safe_parse(&body.0) {
        Ok(_) => (StatusCode::CREATED, "ok").into_response(),
        Err(errors) => {
            let details: Vec<serde_json::Value> = errors
                .issues
                .iter()
                .map(|issue| serde_json::json!({
                    "field": issue.path.join("."),
                    "code": "validation_error",
                    "detail": issue.message,
                }))
                .collect();

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "status": "error", "errors": details })),
            ).into_response()
        }
    }
}
```

See [Error Handling](/advanced/error-handling/) for the full error model, including how to inspect issue codes and attach custom metadata.

## Full example: user registration endpoint

Putting it all together — a registration endpoint that validates a nested payload with optional fields, returns structured errors, and produces a typed struct on success:

```rust
use axum::{Router, routing::post};
use serde::Deserialize;
use zod_rs::prelude::*;
use zod_rs::axum::ZodJson;

#[derive(ZodSchema, Deserialize, Debug)]
struct Profile {
    #[zod(min_length(1), max_length(200))]
    bio: Option<String>,

    #[zod(url)]
    website: Option<String>,
}

#[derive(ZodSchema, Deserialize, Debug)]
struct RegisterRequest {
    #[zod(min_length(3), max_length(30))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min_length(8), max_length(128))]
    password: String,

    profile: Option<Profile>,
}

async fn register(ZodJson(req): ZodJson<RegisterRequest>) -> String {
    // At this point, every field is guaranteed valid.
    format!("Registered {} ({})", req.username, req.email)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/register", post(register));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

A request with a bad nested field:

```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{"username":"ab","email":"bad","password":"short","profile":{"website":"not-a-url"}}'
```

Returns every issue at once, with full paths:

```json
{
  "errors": [
    { "path": "username", "message": "Too small: expected string to have >= 3 characters" },
    { "path": "email", "message": "Invalid email address" },
    { "path": "password", "message": "Too small: expected string to have >= 8 characters" },
    { "path": "profile.website", "message": "Invalid URL" }
  ]
}
```

## Next steps

- [How to Validate JSON in Rust](/guides/validate-json/) — validation fundamentals and common recipes
- [Input Validation in Rust](/guides/input-validation/) — validating strings, numbers, and enums beyond HTTP
- [Axum Integration](/integrations/axum/) — full extractor API and custom rejection handling
- [Error Handling](/advanced/error-handling/) — error types, paths, and custom messages
- [Schema Composition](/advanced/schema-composition/) — building reusable validation rules
