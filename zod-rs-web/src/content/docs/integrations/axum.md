---
title: Axum Integration
description: Using zod-rs with the Axum web framework
---

zod-rs integrates with [Axum](https://github.com/tokio-rs/axum) for request validation in web APIs.

## Setup

```toml
[dependencies]
zod-rs = { version = "0.4", features = ["axum"] }
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Example

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: f64,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    errors: Option<Vec<String>>,
}

fn user_schema() -> impl Schema<Value> {
    object()
        .field("name", string().min(2).max(50))
        .field("email", string().email())
        .field("age", number().min(13.0).max(120.0).int())
}

async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match user_schema().validate(&payload) {
        Ok(_) => {
            let user: CreateUserRequest = serde_json::from_value(payload).unwrap();
            (
                StatusCode::CREATED,
                ResponseJson(ApiResponse {
                    success: true,
                    data: Some(user),
                    errors: None,
                }),
            )
        }
        Err(validation_result) => {
            let errors: Vec<String> = validation_result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();

            (
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiResponse::<CreateUserRequest> {
                    success: false,
                    data: None,
                    errors: Some(errors),
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Using with derive macros

You can also combine the Axum integration with `ZodSchema`:

```rust
#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct CreateUserRequest {
    #[zod(min_length(2), max_length(50))]
    name: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,
}

async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match CreateUserRequest::validate_and_parse(&payload) {
        Ok(user) => { /* handle valid user */ }
        Err(e) => { /* handle validation error */ }
    }
}
```
