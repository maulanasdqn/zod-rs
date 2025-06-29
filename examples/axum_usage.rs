use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: f64,
    interests: Vec<String>,
    profile: UserProfile,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    bio: Option<String>,
    website: Option<String>,
    social_links: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: f64,
    interests: Vec<String>,
    profile: UserProfile,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            errors: None,
        }
    }

    fn error(errors: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            errors: Some(errors),
        }
    }
}

fn create_user_schema() -> impl Schema<Value> {
    object()
        .field("name", string().min(2).max(50))
        .field("email", string().email())
        .field("age", number().min(13.0).max(120.0).int())
        .field("interests", array(string().min(1)).min(1).max(10))
        .field("profile", create_profile_schema())
}

fn create_profile_schema() -> impl Schema<Value> {
    object()
        .optional_field("bio", string().max(500))
        .optional_field("website", string().url())
        .field("social_links", create_social_links_schema())
}

fn create_social_links_schema() -> impl Schema<Value> {
    object()
}

fn validate_user_request(data: &Value) -> Result<(), Vec<String>> {
    let schema = create_user_schema();
    match schema.validate(data) {
        Ok(_) => Ok(()),
        Err(validation_result) => {
            let errors = validation_result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();
            Err(errors)
        }
    }
}

async fn health_check() -> impl IntoResponse {
    ResponseJson(ApiResponse::success("Server is running"))
}

async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match validate_user_request(&payload) {
        Ok(_) => {
            let user_data: CreateUserRequest = serde_json::from_value(payload).unwrap();

            let new_user = User {
                id: 1,
                name: user_data.name,
                email: user_data.email,
                age: user_data.age,
                interests: user_data.interests,
                profile: user_data.profile,
            };

            (
                StatusCode::CREATED,
                ResponseJson(ApiResponse::success(new_user)),
            )
        }
        Err(errors) => (
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::<User>::error(errors)),
        ),
    }
}

async fn get_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 28.0,
            interests: vec!["rust".to_string(), "web-development".to_string()],
            profile: UserProfile {
                bio: Some("Full-stack developer passionate about Rust".to_string()),
                website: Some("https://alice.dev".to_string()),
                social_links: {
                    let mut links = HashMap::new();
                    links.insert("github".to_string(), "https://github.com/alice".to_string());
                    links.insert(
                        "twitter".to_string(),
                        "https://twitter.com/alice".to_string(),
                    );
                    links
                },
            },
        },
        User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            age: 32.0,
            interests: vec!["blockchain".to_string(), "gaming".to_string()],
            profile: UserProfile {
                bio: None,
                website: None,
                social_links: HashMap::new(),
            },
        },
    ];

    ResponseJson(ApiResponse::success(users))
}

async fn validate_data(Json(payload): Json<Value>) -> impl IntoResponse {
    let string_schema = string().min(3).max(20).regex(r"^[a-zA-Z0-9_]+$");

    match string_schema.validate(&payload) {
        Ok(validated_string) => {
            let response = json!({
                "input": payload,
                "validated": validated_string,
                "valid": true
            });
            (StatusCode::OK, ResponseJson(ApiResponse::success(response)))
        }
        Err(validation_result) => {
            let errors = validation_result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();
            (
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiResponse::<Value>::error(errors)),
            )
        }
    }
}

fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/validate", post(validate_data))
}

#[tokio::main]
async fn main() {
    println!("🦀 Starting zod-rs Axum Example Server");
    println!("====================================");

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server running on http://localhost:3000");
    println!("\nAvailable endpoints:");
    println!("GET  /health     - Health check");
    println!("GET  /users      - Get all users");
    println!("POST /users      - Create new user (with validation)");
    println!("POST /validate   - Validate arbitrary string data");

    println!("\nExample POST /users payload:");
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "name": "John Doe",
            "email": "john@example.com",
            "age": 25,
            "interests": ["rust", "programming"],
            "profile": {
                "bio": "Software developer",
                "website": "https://johndoe.dev",
                "social_links": {
                    "github": "https://github.com/johndoe"
                }
            }
        }))
        .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
