use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct SignupRequest {
    #[zod(min_length(3), max_length(20), regex(r"^[a-zA-Z0-9_]+$"))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min_length(8))]
    password: String,

    #[zod(min(13.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(2), max_length(100))]
    full_name: String,

    #[zod(url)]
    website: Option<String>,

    #[zod(min_length(1), max_length(100))]
    bio: Option<String>,

    #[zod(min_length(1), max_length(10))]
    interests: Vec<String>,

    newsletter_subscription: bool,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct ProductCreateRequest {
    #[zod(min_length(1), max_length(200))]
    name: String,

    #[zod(min_length(10), max_length(2000))]
    description: String,

    #[zod(positive)]
    price: f64,

    #[zod(min_length(1))]
    category: String,

    #[zod(min_length(1), max_length(20))]
    tags: Vec<String>,

    #[zod(nonnegative)]
    stock_quantity: u32,

    #[zod(url)]
    image_url: Option<String>,

    #[zod(min(0.0), max(5.0))]
    discount_percentage: Option<f64>,

    active: bool,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct UserProfile {
    #[zod(min_length(2), max_length(50))]
    first_name: String,

    #[zod(min_length(2), max_length(50))]
    last_name: String,

    #[zod(email)]
    email: String,

    #[zod(min(18.0), max(100.0), int)]
    age: u32,

    #[zod(min_length(10), max_length(15), regex(r"^\+?[\d\s\-()]+$"))]
    phone: Option<String>,

    address: Option<Address>,

    #[zod(min_length(1), max_length(5))]
    languages: Vec<String>,

    preferences: UserPreferences,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct Address {
    #[zod(min_length(5), max_length(200))]
    street: String,

    #[zod(min_length(2), max_length(50))]
    city: String,

    #[zod(length(2))]
    country_code: String,

    #[zod(regex(r"^\d{5}(-\d{4})?$"))]
    postal_code: String,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct UserPreferences {
    #[zod(min_length(1))]
    theme: String,

    #[zod(length(2))]
    language: String,

    newsletter: bool,

    notifications: NotificationSettings,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct NotificationSettings {
    email_notifications: bool,
    push_notifications: bool,
    sms_notifications: bool,
}

fn validate_signup_example() {
    println!("🔐 Signup Validation Example:");

    let valid_signup = json!({
        "username": "alice_dev",
        "email": "alice@example.com",
        "password": "MySecurePass123",
        "age": 28,
        "full_name": "Alice Johnson",
        "website": "https://alice.dev",
        "bio": "Rust developer and open source enthusiast",
        "interests": ["rust", "programming", "open-source"],
        "newsletter_subscription": true
    });

    match SignupRequest::validate_and_parse(&valid_signup) {
        Ok(signup) => println!("✅ Valid signup: {:#?}", signup),
        Err(e) => println!("❌ Invalid signup: {}", e),
    }

    let invalid_signup = json!({
        "username": "a",
        "email": "not-an-email",
        "password": "weak",
        "age": 12,
        "full_name": "A",
        "website": "not-a-url",
        "interests": [],
        "newsletter_subscription": "yes"
    });

    match SignupRequest::validate_and_parse(&invalid_signup) {
        Ok(signup) => println!("✅ Valid signup: {:#?}", signup),
        Err(e) => println!("❌ Invalid signup: {}", e),
    }
}

fn validate_product_example() {
    println!("\n🛍️ Product Validation Example:");

    let valid_product = json!({
        "name": "Premium Rust Programming Course",
        "description": "Comprehensive course covering advanced Rust programming concepts and real-world applications",
        "price": 199.99,
        "category": "Education",
        "tags": ["rust", "programming", "course", "advanced"],
        "stock_quantity": 50,
        "image_url": "https://example.com/course-image.jpg",
        "discount_percentage": 4.5,
        "active": true
    });

    match ProductCreateRequest::validate_and_parse(&valid_product) {
        Ok(product) => println!("✅ Valid product: {:#?}", product),
        Err(e) => println!("❌ Invalid product: {}", e),
    }

    let invalid_product = json!({
        "name": "",
        "description": "Too short",
        "price": -50.0,
        "category": "",
        "tags": [],
        "stock_quantity": -1,
        "discount_percentage": 150.0,
        "active": "yes"
    });

    match ProductCreateRequest::validate_and_parse(&invalid_product) {
        Ok(product) => println!("✅ Valid product: {:#?}", product),
        Err(e) => println!("❌ Invalid product: {}", e),
    }
}

fn validate_nested_structure_example() {
    println!("\n👤 Nested Structure Validation Example:");

    let valid_profile = json!({
        "first_name": "John",
        "last_name": "Doe",
        "email": "john.doe@example.com",
        "age": 30,
        "phone": "+1 (555) 123-4567",
        "address": {
            "street": "123 Main Street, Apt 4B",
            "city": "New York",
            "country_code": "US",
            "postal_code": "10001"
        },
        "languages": ["en", "es", "fr"],
        "preferences": {
            "theme": "dark",
            "language": "en",
            "newsletter": true,
            "notifications": {
                "email_notifications": true,
                "push_notifications": false,
                "sms_notifications": true
            }
        }
    });

    match UserProfile::validate_and_parse(&valid_profile) {
        Ok(profile) => println!("✅ Valid profile: {:#?}", profile),
        Err(e) => println!("❌ Invalid profile: {}", e),
    }

    let invalid_profile = json!({
        "first_name": "J",
        "last_name": "D",
        "email": "invalid-email",
        "age": 15,
        "phone": "123",
        "address": {
            "street": "St",
            "city": "N",
            "country_code": "USA",
            "postal_code": "123"
        },
        "languages": [],
        "preferences": {
            "theme": "",
            "language": "english",
            "newsletter": "yes",
            "notifications": {
                "email_notifications": "true",
                "push_notifications": 1,
                "sms_notifications": "false"
            }
        }
    });

    match UserProfile::validate_and_parse(&invalid_profile) {
        Ok(profile) => println!("✅ Valid profile: {:#?}", profile),
        Err(e) => println!("❌ Invalid profile: {}", e),
    }
}

fn performance_comparison_example() {
    println!("\n⚡ Performance Comparison:");

    let test_data = json!({
        "username": "performance_test",
        "email": "test@example.com",
        "password": "TestPass123",
        "age": 25,
        "full_name": "Performance Test User",
        "interests": ["testing", "performance"],
        "newsletter_subscription": true
    });

    let iterations = 10000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = SignupRequest::validate_and_parse(&test_data);
    }
    let duration = start.elapsed();

    println!(
        "✅ Validated {} signup requests in {:?} ({:.2} μs per validation)",
        iterations,
        duration,
        duration.as_micros() as f64 / iterations as f64
    );

    let product_data = json!({
        "name": "Test Product",
        "description": "This is a test product with sufficient description length",
        "price": 99.99,
        "category": "Test",
        "tags": ["test", "product"],
        "stock_quantity": 100,
        "active": true
    });

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = ProductCreateRequest::validate_and_parse(&product_data);
    }
    let duration = start.elapsed();

    println!(
        "✅ Validated {} product requests in {:?} ({:.2} μs per validation)",
        iterations,
        duration,
        duration.as_micros() as f64 / iterations as f64
    );
}

fn schema_reuse_example() {
    println!("\n🔄 Schema Reuse and Composition:");

    let signup_schema = SignupRequest::schema();
    let product_schema = ProductCreateRequest::schema();

    let test_signup_data = json!({
        "username": "schema_test",
        "email": "schema@example.com",
        "password": "SchemaTest123",
        "age": 27,
        "full_name": "Schema Test User",
        "interests": ["schemas", "validation"],
        "newsletter_subscription": false
    });

    match signup_schema.validate(&test_signup_data) {
        Ok(_) => println!("✅ Signup schema validation passed"),
        Err(e) => println!("❌ Signup schema validation failed: {}", e),
    }

    let test_product_data = json!({
        "name": "Schema Test Product",
        "description": "Testing schema validation and reusability features",
        "price": 49.99,
        "category": "Testing",
        "tags": ["schema", "test"],
        "stock_quantity": 25,
        "active": true
    });

    match product_schema.validate(&test_product_data) {
        Ok(_) => println!("✅ Product schema validation passed"),
        Err(e) => println!("❌ Product schema validation failed: {}", e),
    }
}

fn main() {
    println!("🦀 zod-rs as Validator Crate Replacement");
    println!("==========================================");
    println!("Demonstrating how zod-rs can replace the Validator crate\n");

    validate_signup_example();
    validate_product_example();
    validate_nested_structure_example();
    performance_comparison_example();
    schema_reuse_example();

    println!("\n🎯 Key Advantages over Validator Crate:");
    println!("  ✅ Type-safe schema generation from structs");
    println!("  ✅ Comprehensive validation with detailed error messages");
    println!("  ✅ Composable and reusable validation schemas");
    println!("  ✅ Built-in JSON validation and deserialization");
    println!("  ✅ Zero-cost abstractions with compile-time validation");
    println!("  ✅ Seamless integration with serde ecosystem");
    println!("  ✅ Rich validation constraints via attributes");
    println!("  ✅ Performance optimized for production use");

    println!("\n🎉 zod-rs validator replacement examples complete!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signup_validation() {
        let valid_data = json!({
            "username": "test_user",
            "email": "test@example.com",
            "password": "TestPass123",
            "age": 25,
            "full_name": "Test User",
            "interests": ["testing"],
            "newsletter_subscription": true
        });

        assert!(SignupRequest::validate_and_parse(&valid_data).is_ok());

        let invalid_data = json!({
            "username": "a",
            "email": "not-email",
            "password": "weak",
            "age": 10,
            "full_name": "T",
            "interests": [],
            "newsletter_subscription": true
        });

        assert!(SignupRequest::validate_and_parse(&invalid_data).is_err());
    }

    #[test]
    fn test_product_validation() {
        let valid_data = json!({
            "name": "Test Product",
            "description": "Valid product description with sufficient length",
            "price": 99.99,
            "category": "Test",
            "tags": ["test"],
            "stock_quantity": 10,
            "active": true
        });

        assert!(ProductCreateRequest::validate_and_parse(&valid_data).is_ok());
    }

    #[test]
    fn test_nested_validation() {
        let valid_data = json!({
            "first_name": "John",
            "last_name": "Doe",
            "email": "john@example.com",
            "age": 30,
            "languages": ["en"],
            "preferences": {
                "theme": "dark",
                "language": "en",
                "newsletter": true,
                "notifications": {
                    "email_notifications": true,
                    "push_notifications": false,
                    "sms_notifications": true
                }
            }
        });

        assert!(UserProfile::validate_and_parse(&valid_data).is_ok());
    }

    #[test]
    fn test_optional_fields() {
        let data_without_optionals = json!({
            "username": "test_user",
            "email": "test@example.com",
            "password": "TestPass123",
            "age": 25,
            "full_name": "Test User",
            "interests": ["testing"],
            "newsletter_subscription": true
        });

        assert!(SignupRequest::validate_and_parse(&data_without_optionals).is_ok());

        let data_with_optionals = json!({
            "username": "test_user",
            "email": "test@example.com",
            "password": "TestPass123",
            "age": 25,
            "full_name": "Test User",
            "website": "https://example.com",
            "bio": "Test bio",
            "interests": ["testing"],
            "newsletter_subscription": true
        });

        assert!(SignupRequest::validate_and_parse(&data_with_optionals).is_ok());
    }
}
