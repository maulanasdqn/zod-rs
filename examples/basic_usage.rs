use serde_json::json;
use zod_rs::prelude::*;

fn main() {
    println!("🦀 Welcome to zod-rs - Rust Schema Validation Library!");
    println!("====================================================");

    // String validation
    println!("\n📝 String Validation:");
    let name_schema = string().min(2).max(50);

    match name_schema.safe_parse(&json!("John")) {
        Ok(name) => println!("✅ Valid name: {name}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match name_schema.safe_parse(&json!("J")) {
        Ok(name) => println!("✅ Valid name: {name}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Email validation
    println!("\n📧 Email Validation:");
    let email_schema = string().email();

    match email_schema.safe_parse(&json!("user@example.com")) {
        Ok(email) => println!("✅ Valid email: {email}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match email_schema.safe_parse(&json!("not-an-email")) {
        Ok(email) => println!("✅ Valid email: {email}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Number validation
    println!("\n🔢 Number Validation:");
    let age_schema = number().min(0.0).max(120.0).int();

    match age_schema.safe_parse(&json!(25)) {
        Ok(age) => println!("✅ Valid age: {age}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match age_schema.safe_parse(&json!(-5)) {
        Ok(age) => println!("✅ Valid age: {age}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Boolean validation
    println!("\n✓ Boolean Validation:");
    let is_active_schema = boolean();

    match is_active_schema.safe_parse(&json!(true)) {
        Ok(is_active) => println!("✅ Valid boolean: {is_active}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Array validation
    println!("\n📋 Array Validation:");
    let tags_schema = array(string()).min(1).max(5);

    match tags_schema.safe_parse(&json!(["rust", "validation", "schema"])) {
        Ok(tags) => println!("✅ Valid tags: {tags:?}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match tags_schema.safe_parse(&json!([])) {
        Ok(tags) => println!("✅ Valid tags: {tags:?}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Optional validation
    println!("\n❓ Optional Validation:");
    let optional_name_schema = optional(string());

    match optional_name_schema.safe_parse(&json!(null)) {
        Ok(name) => println!("✅ Valid optional name: {name:?}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match optional_name_schema.safe_parse(&json!("Alice")) {
        Ok(name) => println!("✅ Valid optional name: {name:?}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    // Literal validation
    println!("\n🎯 Literal Validation:");
    let status_schema = literal("active".to_string());

    match status_schema.safe_parse(&json!("active")) {
        Ok(status) => println!("✅ Valid status: {status}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    match status_schema.safe_parse(&json!("inactive")) {
        Ok(status) => println!("✅ Valid status: {status}"),
        Err(err) => println!("❌ Invalid: {err}"),
    }

    println!("\n🎉 zod-rs validation examples complete!");
}
