use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct LoginWorkDomainRequest {
    #[zod(email, ends_with("@work_domain.com"))]
    email: String,

    #[zod(min_length(8))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct User {
    #[zod(min_length(2), max_length(50), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(1), max_length(10))]
    interests: Vec<String>,

    bio: Option<String>,

    #[zod(nonnegative)]
    score: f64,

    is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Product {
    #[zod(min_length(1), max_length(100))]
    name: String,

    #[zod(positive)]
    price: f64,

    #[zod(min_length(1))]
    category: String,

    #[zod(max_length(20))]
    tags: Vec<String>,

    available: bool,

    #[zod(min_length(3), max_length(500))]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct CreatePostRequest {
    #[zod(min_length(5), max_length(200))]
    title: String,

    #[zod(min_length(10))]
    content: String,

    #[zod(positive, int)]
    author_id: u64,

    #[zod(min_length(1), max_length(5))]
    tags: Vec<String>,

    published: Option<bool>,

    #[zod(url)]
    featured_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Phone {
    #[zod(length(2))]
    alpha_code: String,

    #[zod(starts_with("+"))]
    code: String,

    #[zod(min_length(3))]
    country: String,

    #[zod(regex(r"^\d+$"))]
    number: String,

    #[zod(starts_with("+"), includes("-"))]
    full_number: String,
}

// ==================== ENUM EXAMPLES ====================

/// Unit variants only - maps to tagged null values
#[derive(Debug, Clone, Serialize, Deserialize, ZodSchema, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending,
}

/// Single-element tuple variants
#[derive(Debug, Clone, Serialize, Deserialize, ZodSchema, PartialEq)]
enum Message {
    Text(String),
    Number(i32),
}

/// Multi-element tuple variants
#[derive(Debug, Clone, Serialize, Deserialize, ZodSchema, PartialEq)]
enum Shape {
    Point,
    Line(i32, i32),
    Rectangle(i32, i32, i32, i32),
}

/// Struct variants with named fields
#[derive(Debug, Clone, Serialize, Deserialize, ZodSchema, PartialEq)]
enum Event {
    Click { x: i32, y: i32 },
    Scroll { delta: f64 },
    Resize { width: u32, height: u32 },
}

/// Mixed variant types
#[derive(Debug, Clone, Serialize, Deserialize, ZodSchema, PartialEq)]
enum ApiResponse {
    Success,
    Data(String),
    Error { code: i32, message: String },
}

fn main() {
    println!("🦀 zod-rs Derive Schema Examples");
    println!("=================================");

    println!("\n📝 User Validation with Derived Schema:");

    let valid_user_json = json!({
        "username": "alice_dev",
        "email": "alice@example.com",
        "age": 28,
        "interests": ["rust", "programming"],
        "bio": "Software developer passionate about Rust",
        "score": 95.5,
        "is_active": true
    });

    match User::validate_and_parse(&valid_user_json) {
        Ok(user) => println!("✅ Valid user: {user:#?}"),
        Err(e) => println!("❌ Invalid user: {e}"),
    }

    let invalid_user_json = json!({
        "username": "a",
        "email": "not-an-email",
        "age": 5,
        "interests": [],
        "score": -10.0,
        "is_active": "yes"
    });

    match User::validate_and_parse(&invalid_user_json) {
        Ok(user) => println!("✅ Valid user: {user:#?}"),
        Err(e) => println!("❌ Invalid user: {e}"),
    }

    println!("\n🛍️ Product Validation:");

    let valid_product_json = json!({
        "name": "Rust Programming Book",
        "price": 39.99,
        "category": "Books",
        "tags": ["rust", "programming", "systems"],
        "available": true,
        "description": "A comprehensive guide to Rust programming"
    });

    match Product::validate_and_parse(&valid_product_json) {
        Ok(product) => println!("✅ Valid product: {product:#?}"),
        Err(e) => println!("❌ Invalid product: {e}"),
    }

    println!("\n📄 Post Creation Validation:");

    let valid_post_json = json!({
        "title": "Getting Started with Rust",
        "content": "Rust is a systems programming language that focuses on safety, speed, and concurrency...",
        "author_id": 42,
        "tags": ["rust", "tutorial", "beginners"],
        "published": true,
        "featured_image": "https://example.com/rust-logo.png"
    });

    match CreatePostRequest::validate_and_parse(&valid_post_json) {
        Ok(post) => println!("✅ Valid post request: {post:#?}"),
        Err(e) => println!("❌ Invalid post request: {e}"),
    }

    let invalid_post_json = json!({
        "title": "Hi",
        "content": "Too short",
        "author_id": -1,
        "tags": [],
        "featured_image": "not-a-url"
    });

    match CreatePostRequest::validate_and_parse(&invalid_post_json) {
        Ok(post) => println!("✅ Valid post request: {post:#?}"),
        Err(e) => println!("❌ Invalid post request: {e}"),
    }

    println!("\n🔍 Schema Inspection:");

    println!("User schema validation:");
    let user_schema = User::schema();
    let test_data = json!({
        "username": "test_user",
        "email": "test@example.com",
        "age": 25,
        "interests": ["testing"],
        "score": 100.0,
        "is_active": true
    });

    match user_schema.validate(&test_data) {
        Ok(_) => println!("✅ Schema validation passed"),
        Err(e) => println!("❌ Schema validation failed: {e}"),
    }

    println!("\n📊 JSON String Validation:");

    let json_string = r#"{
        "username": "json_user",
        "email": "json@example.com",
        "age": 30,
        "interests": ["json", "validation"],
        "score": 88.5,
        "is_active": false
    }"#;

    match User::from_json(json_string) {
        Ok(user) => println!("✅ Valid user from JSON string: {user:#?}"),
        Err(e) => println!("❌ Invalid JSON: {e}"),
    }

    println!("\n🚀 Performance Comparison:");

    let iterations = 1000;
    let test_user_json = json!({
        "username": "performance_test",
        "email": "perf@example.com",
        "age": 25,
        "interests": ["performance", "testing"],
        "score": 95.0,
        "is_active": true
    });

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = User::validate_and_parse(&test_user_json);
    }
    let duration = start.elapsed();
    println!(
        "✅ Validated {} users in {:?} ({:.2} μs per validation)",
        iterations,
        duration,
        duration.as_micros() as f64 / iterations as f64
    );

    println!("\n🎉 Derive schema examples complete!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_schema_generation() {
        let schema = User::schema();
        let valid_data = json!({
            "username": "test_user",
            "email": "test@example.com",
            "age": 25,
            "interests": ["testing"],
            "score": 100.0,
            "is_active": true
        });

        assert!(schema.validate(&valid_data).is_ok());
    }

    #[test]
    fn test_user_validation_fails_on_invalid_email() {
        let invalid_data = json!({
            "username": "test_user",
            "email": "not-an-email",
            "age": 25,
            "interests": ["testing"],
            "score": 100.0,
            "is_active": true
        });

        assert!(User::validate_and_parse(&invalid_data).is_err());
    }

    #[test]
    fn test_user_validation_fails_on_short_username() {
        let invalid_data = json!({
            "username": "a",
            "email": "test@example.com",
            "age": 25,
            "interests": ["testing"],
            "score": 100.0,
            "is_active": true
        });

        assert!(User::validate_and_parse(&invalid_data).is_err());
    }

    #[test]
    fn test_user_validation_fails_on_negative_score() {
        let invalid_data = json!({
            "username": "test_user",
            "email": "test@example.com",
            "age": 25,
            "interests": ["testing"],
            "score": -10.0,
            "is_active": true
        });

        assert!(User::validate_and_parse(&invalid_data).is_err());
    }

    #[test]
    fn test_product_schema_generation() {
        let schema = Product::schema();
        let valid_data = json!({
            "name": "Test Product",
            "price": 29.99,
            "category": "Test",
            "tags": ["test"],
            "available": true
        });

        assert!(schema.validate(&valid_data).is_ok());
    }

    #[test]
    fn test_optional_fields() {
        let data_without_optionals = json!({
            "username": "test_user",
            "email": "test@example.com",
            "age": 25,
            "interests": ["testing"],
            "score": 100.0,
            "is_active": true
        });

        assert!(User::validate_and_parse(&data_without_optionals).is_ok());

        let data_with_optionals = json!({
            "username": "test_user",
            "email": "test@example.com",
            "age": 25,
            "interests": ["testing"],
            "bio": "Test bio",
            "score": 100.0,
            "is_active": true
        });

        assert!(User::validate_and_parse(&data_with_optionals).is_ok());
    }

    #[test]
    fn test_work_domain() {
        let vaild_login = json!({
            "email": "test@work_domain.com",
            "password": "TestPass123",

        });

        assert!(LoginWorkDomainRequest::validate_and_parse(&vaild_login).is_ok());

        let invaild_login = json!({
            "email": "test@hotmail.com",
            "password": "TestPass123",
        });

        assert!(LoginWorkDomainRequest::validate_and_parse(&invaild_login).is_err());
    }

    #[test]
    fn test_phone_dto() {
        let vaild_phone_dto = json!({
            "alpha_code": "JP",
            "country": "Japan",
            "code": "+81",
            "number": "9012345678",
            "full_number": "+81-9012345678"
        });

        assert!(Phone::validate_and_parse(&vaild_phone_dto).is_ok());

        let invaild_phone_dto_code_full_number = json!({
            "alpha_code": "JP",
            "country": "Japan",
            "code": "0081",
            "number": "9012345678",
            "full_number": "00819012345678"
        });

        assert!(Phone::validate_and_parse(&invaild_phone_dto_code_full_number).is_err());

        let invaild_phone_dto_full_number = json!({
            "alpha_code": "JP",
            "country": "Japan",
            "code": "+81",
            "number": "9012345678",
            "full_number": "81-9012345678"
        });

        assert!(Phone::validate_and_parse(&invaild_phone_dto_full_number).is_err());
    }

    // ==================== ENUM TESTS ====================

    #[test]
    fn test_unit_variant_enum() {
        // Valid unit variants
        let active = json!({"Active": null});
        let result = Status::validate_and_parse(&active);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Status::Active);

        let inactive = json!({"Inactive": null});
        assert!(Status::validate_and_parse(&inactive).is_ok());

        let pending = json!({"Pending": null});
        assert!(Status::validate_and_parse(&pending).is_ok());

        // Invalid: unknown variant
        let unknown = json!({"Unknown": null});
        assert!(Status::validate_and_parse(&unknown).is_err());

        // Invalid: wrong format (string instead of object)
        let wrong_format = json!("Active");
        assert!(Status::validate_and_parse(&wrong_format).is_err());
    }

    #[test]
    fn test_single_tuple_variant_enum() {
        // Valid tuple variants
        let text = json!({"Text": "hello"});
        let result = Message::validate_and_parse(&text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Message::Text("hello".to_string()));

        let number = json!({"Number": 42});
        let result = Message::validate_and_parse(&number);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Message::Number(42));

        // Invalid: wrong type in tuple
        let wrong_type = json!({"Text": 123});
        assert!(Message::validate_and_parse(&wrong_type).is_err());

        let wrong_type2 = json!({"Number": "not a number"});
        assert!(Message::validate_and_parse(&wrong_type2).is_err());
    }

    #[test]
    fn test_multi_tuple_variant_enum() {
        // Unit variant within mixed enum
        let point = json!({"Point": null});
        let result = Shape::validate_and_parse(&point);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Shape::Point);

        // Two-element tuple
        let line = json!({"Line": [0, 10]});
        let result = Shape::validate_and_parse(&line);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Shape::Line(0, 10));

        // Four-element tuple
        let rect = json!({"Rectangle": [0, 0, 100, 200]});
        let result = Shape::validate_and_parse(&rect);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Shape::Rectangle(0, 0, 100, 200));

        // Invalid: wrong tuple length
        let wrong_len = json!({"Line": [0]});
        assert!(Shape::validate_and_parse(&wrong_len).is_err());

        let wrong_len2 = json!({"Rectangle": [0, 0, 100]});
        assert!(Shape::validate_and_parse(&wrong_len2).is_err());
    }

    #[test]
    fn test_struct_variant_enum() {
        // Valid struct variants
        let click = json!({"Click": {"x": 100, "y": 200}});
        let result = Event::validate_and_parse(&click);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Event::Click { x: 100, y: 200 });

        let scroll = json!({"Scroll": {"delta": 1.5}});
        let result = Event::validate_and_parse(&scroll);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Event::Scroll { delta: 1.5 });

        let resize = json!({"Resize": {"width": 800, "height": 600}});
        let result = Event::validate_and_parse(&resize);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Event::Resize { width: 800, height: 600 });

        // Invalid: missing field
        let missing = json!({"Click": {"x": 100}});
        assert!(Event::validate_and_parse(&missing).is_err());

        // Invalid: wrong field type
        let wrong_type = json!({"Click": {"x": "hundred", "y": 200}});
        assert!(Event::validate_and_parse(&wrong_type).is_err());
    }

    #[test]
    fn test_mixed_variant_enum() {
        // Unit variant
        let success = json!({"Success": null});
        assert_eq!(
            ApiResponse::validate_and_parse(&success).unwrap(),
            ApiResponse::Success
        );

        // Tuple variant
        let data = json!({"Data": "some payload"});
        assert_eq!(
            ApiResponse::validate_and_parse(&data).unwrap(),
            ApiResponse::Data("some payload".to_string())
        );

        // Struct variant
        let error = json!({"Error": {"code": 404, "message": "Not found"}});
        assert_eq!(
            ApiResponse::validate_and_parse(&error).unwrap(),
            ApiResponse::Error {
                code: 404,
                message: "Not found".to_string()
            }
        );
    }

    #[test]
    fn test_enum_from_json() {
        let json_str = r#"{"Active": null}"#;
        let result = Status::from_json(json_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Status::Active);

        let json_str = r#"{"Click": {"x": 10, "y": 20}}"#;
        let result = Event::from_json(json_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Event::Click { x: 10, y: 20 });
    }

    #[test]
    fn test_enum_validate_json() {
        let json_str = r#"{"Text": "hello"}"#;
        let result = Message::validate_json(json_str);
        assert!(result.is_ok());

        let invalid_json_str = r#"{"Text": 123}"#;
        let result = Message::validate_json(invalid_json_str);
        assert!(result.is_err());
    }
}
