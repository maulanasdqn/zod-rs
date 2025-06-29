use serde_json::json;
use zod_rs::prelude::*;

fn main() {
    println!("🦀 zod-rs Demo");

    let name_schema = string().min(2).max(50);
    match name_schema.safe_parse(&json!("John")) {
        Ok(name) => println!("✅ Valid name: {}", name),
        Err(err) => println!("❌ Invalid: {}", err),
    }

    let age_schema = number().min(0.0).max(120.0);
    match age_schema.safe_parse(&json!(25)) {
        Ok(age) => println!("✅ Valid age: {}", age),
        Err(err) => println!("❌ Invalid: {}", err),
    }

    let tags_schema = array(string()).min(1).max(3);
    match tags_schema.safe_parse(&json!(["rust", "validation"])) {
        Ok(tags) => println!("✅ Valid tags: {:?}", tags),
        Err(err) => println!("❌ Invalid: {}", err),
    }

    println!("🎉 Demo complete!");
}
