use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize, ZodSchema)]
struct ApiFieldError {
    path: String,
    message: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ApiFieldErrors {
    errors: Vec<ApiFieldError>,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
struct RegisterDto {
    #[zod(min_length(3))]
    username: String,
    #[zod(email, ends_with("@doe.com"))]
    email: String,
    #[zod(string, min_length(8))]
    password: String,
}

fn main() {
    println!("🦀 Starting zod-rs i18n Example");
    println!("====================================");

    let input_dto = &json!({
        "username": "j.doe",
        "email": "john@gmail.com",
        "password": "pass@123",
    });

    let register_dto = RegisterDto::validate_and_parse(input_dto);

    match register_dto {
        Ok(dto) => println!("✅ Valid dto: {}", dto.username),
        Err(err) => {
            let mut field_errors = ApiFieldErrors::default();

            for issue in err.issues {
                field_errors.errors.push(ApiFieldError {
                    path: issue.path.join("."),
                    message: issue.error.local(Locale::Ar),
                });
            }

            for issue in field_errors.errors {
                println!("❌ Invalid {}: {}\n", issue.path, issue.message)
            }
        }
    }

    println!("\n🎉 i18n example completed!");
}
