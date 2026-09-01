use serde::{Deserialize, Serialize};
use serde_json::json;
use zod_rs::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, ZodSchema)]
enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ZodSchema)]
pub enum AccountVerificationType {
    #[serde(rename = "register")]
    Register,

    #[serde(rename = "login")]
    Login,
}

#[derive(Debug, Serialize, Deserialize, ZodSchema)]
pub struct CreateChallengeBody {
    pub code_type: AccountVerificationType,

    #[zod(email, max_length(254))]
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ZodSchema)]
#[serde(rename_all = "snake_case")]
enum RenamedAll {
    SomeValue,
    AnotherValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ZodSchema)]
enum Message {
    Pending,
    Text(String),
    Error { code: i32 },
}

#[test]
fn unit_variants_validate_as_strings() {
    assert_eq!(
        Status::validate_and_parse(&json!("Active")).unwrap(),
        Status::Active
    );
    assert_eq!(
        Status::validate_and_parse(&json!("Pending")).unwrap(),
        Status::Pending
    );

    assert_eq!(
        Status::validate_and_parse(&json!({"Active": null})).unwrap(),
        Status::Active
    );

    assert!(Status::validate_and_parse(&json!("Unknown")).is_err());
    assert!(Status::validate_and_parse(&json!({"Unknown": null})).is_err());
}

#[test]
fn unit_variants_match_serde_serialization() {
    let serialized = serde_json::to_value(Status::Active).unwrap();
    assert_eq!(serialized, json!("Active"));
    assert!(Status::validate_and_parse(&serialized).is_ok());
}

#[test]
fn serde_rename_is_honored() {
    let value = json!({
        "code_type": "register",
        "email": "user@example.com"
    });

    assert!(serde_json::from_value::<CreateChallengeBody>(value.clone()).is_ok());

    let validated = CreateChallengeBody::validate_and_parse(&value).unwrap();
    assert_eq!(validated.code_type, AccountVerificationType::Register);

    assert!(AccountVerificationType::validate_and_parse(&json!("Register")).is_err());
    assert!(AccountVerificationType::validate_and_parse(&json!("login")).is_ok());
    assert!(AccountVerificationType::validate_and_parse(&json!({"register": null})).is_ok());
    assert!(AccountVerificationType::validate_and_parse(&json!({"Register": null})).is_err());
}

#[test]
fn serde_rename_all_is_honored() {
    assert_eq!(
        RenamedAll::validate_and_parse(&json!("some_value")).unwrap(),
        RenamedAll::SomeValue
    );
    assert_eq!(
        RenamedAll::validate_and_parse(&json!("another_value")).unwrap(),
        RenamedAll::AnotherValue
    );
    assert!(RenamedAll::validate_and_parse(&json!("SomeValue")).is_err());

    let serialized = serde_json::to_value(RenamedAll::SomeValue).unwrap();
    assert_eq!(serialized, json!("some_value"));
    assert!(RenamedAll::validate_and_parse(&serialized).is_ok());
}

#[test]
fn mixed_enums_keep_tagged_objects_for_data_variants() {
    assert_eq!(
        Message::validate_and_parse(&json!("Pending")).unwrap(),
        Message::Pending
    );
    assert_eq!(
        Message::validate_and_parse(&json!({"Text": "hello"})).unwrap(),
        Message::Text("hello".to_string())
    );
    assert_eq!(
        Message::validate_and_parse(&json!({"Error": {"code": 500}})).unwrap(),
        Message::Error { code: 500 }
    );

    assert!(Message::validate_and_parse(&json!("Text")).is_err());
}
