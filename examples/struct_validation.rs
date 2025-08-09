use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    username: String,
    email: String,
    age: f64,
    bio: Option<String>,
    interests: Vec<String>,
    settings: UserSettings,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSettings {
    notifications_enabled: bool,
    theme: String,
    language: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
    category: String,
    tags: Vec<String>,
    available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    author_id: f64,
    tags: Vec<String>,
    published: Option<bool>,
}

fn create_user_schema() -> impl Schema<Value> {
    object()
        .field(
            "username",
            string().min(3).max(20).regex(r"^[a-zA-Z0-9_]+$"),
        )
        .field("email", string().email())
        .field("age", number().min(13.0).max(120.0).int())
        .optional_field("bio", string().max(500))
        .field("interests", array(string().min(1)).min(1).max(10))
        .field("settings", create_settings_schema())
        .field("metadata", create_metadata_schema())
}

fn create_settings_schema() -> impl Schema<Value> {
    object()
        .field("notifications_enabled", boolean())
        .field(
            "theme",
            union()
                .variant(literal("light".to_string()))
                .variant(literal("dark".to_string())),
        )
        .field("language", string().regex(r"^[a-z]{2}$"))
}

fn create_metadata_schema() -> impl Schema<Value> {
    object()
}

fn create_product_schema() -> impl Schema<Value> {
    object()
        .field("name", string().min(1).max(100))
        .field("price", number().min(0.0))
        .field("category", string().min(1))
        .field("tags", array(string()).max(20))
        .field("available", boolean())
}

fn create_post_schema() -> impl Schema<Value> {
    object()
        .field("title", string().min(5).max(200))
        .field("content", string().min(10))
        .field("author_id", number().int().positive())
        .field("tags", array(string().min(1)).min(1).max(5))
        .optional_field("published", boolean())
}

fn validate_and_deserialize<T>(schema: impl Schema<Value>, data: &Value) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    match schema.validate(data) {
        Ok(_) => match serde_json::from_value::<T>(data.clone()) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Deserialization failed: {e}")),
        },
        Err(validation_result) => {
            let errors: Vec<String> = validation_result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();
            Err(format!("Validation failed: {}", errors.join("; ")))
        }
    }
}

fn main() {
    println!("🦀 zod-rs Struct Validation Examples");
    println!("=====================================");

    println!("\n📝 User Profile Validation:");
    let valid_user_data = json!({
        "username": "alice_dev",
        "email": "alice@example.com",
        "age": 28,
        "bio": "Software developer passionate about Rust",
        "interests": ["rust", "web-development", "open-source"],
        "settings": {
            "notifications_enabled": true,
            "theme": "dark",
            "language": "en"
        },
        "metadata": {
            "signup_source": "github",
            "referrer": "friend"
        }
    });

    match validate_and_deserialize::<UserProfile>(create_user_schema(), &valid_user_data) {
        Ok(user) => println!("✅ Valid user profile: {user:#?}"),
        Err(e) => println!("❌ Invalid user profile: {e}"),
    }

    let invalid_user_data = json!({
        "username": "a",
        "email": "not-an-email",
        "age": 5,
        "interests": [],
        "settings": {
            "notifications_enabled": "yes",
            "theme": "rainbow",
            "language": "english"
        },
        "metadata": {}
    });

    match validate_and_deserialize::<UserProfile>(create_user_schema(), &invalid_user_data) {
        Ok(user) => println!("✅ Valid user profile: {user:#?}"),
        Err(e) => println!("❌ Invalid user profile: {e}"),
    }

    println!("\n🛍️ Product Validation:");
    let valid_product_data = json!({
        "name": "Rust Programming Book",
        "price": 39.99,
        "category": "Books",
        "tags": ["rust", "programming", "systems"],
        "available": true
    });

    match validate_and_deserialize::<Product>(create_product_schema(), &valid_product_data) {
        Ok(product) => println!("✅ Valid product: {product:#?}"),
        Err(e) => println!("❌ Invalid product: {e}"),
    }

    println!("\n📄 Post Creation Validation:");
    let valid_post_data = json!({
        "title": "Getting Started with Rust",
        "content": "Rust is a systems programming language that focuses on safety, speed, and concurrency...",
        "author_id": 42,
        "tags": ["rust", "tutorial", "beginners"]
    });

    match validate_and_deserialize::<CreatePostRequest>(create_post_schema(), &valid_post_data) {
        Ok(post) => println!("✅ Valid post request: {post:#?}"),
        Err(e) => println!("❌ Invalid post request: {e}"),
    }

    let invalid_post_data = json!({
        "title": "Hi",
        "content": "Too short",
        "author_id": -1,
        "tags": ["", "too-long-tag-that-exceeds-reasonable-length"]
    });

    match validate_and_deserialize::<CreatePostRequest>(create_post_schema(), &invalid_post_data) {
        Ok(post) => println!("✅ Valid post request: {post:#?}"),
        Err(e) => println!("❌ Invalid post request: {e}"),
    }

    println!("\n🧪 Individual Field Validation:");

    let email_schema = string().email();
    let test_emails = vec![
        "valid@example.com",
        "user.name+tag@domain.co.uk",
        "invalid.email",
        "@example.com",
    ];

    for email in test_emails {
        match email_schema.safe_parse(&json!(email)) {
            Ok(valid_email) => println!("✅ Valid email: {valid_email}"),
            Err(_) => println!("❌ Invalid email: {email}"),
        }
    }

    println!("\n🔢 Number Validation with Constraints:");
    let age_schema = number().min(0.0).max(150.0).int();
    let test_ages = vec![25.0, 0.0, 150.0, -5.0, 200.0, 25.5];

    for age in test_ages {
        match age_schema.safe_parse(&json!(age)) {
            Ok(valid_age) => println!("✅ Valid age: {valid_age}"),
            Err(_) => println!("❌ Invalid age: {age}"),
        }
    }

    println!("\n📋 Array Validation:");
    let tags_schema = array(string().min(1).max(20)).min(1).max(5);
    let test_tag_arrays = vec![
        json!(["rust", "programming"]),
        json!([]),
        json!(["a", "b", "c", "d", "e", "f"]),
        json!(["", "valid-tag"]),
    ];

    for tags in test_tag_arrays {
        match tags_schema.safe_parse(&tags) {
            Ok(valid_tags) => println!("✅ Valid tags: {valid_tags:?}"),
            Err(_) => println!("❌ Invalid tags: {tags}"),
        }
    }

    println!("\n🎉 Struct validation examples complete!");
}
