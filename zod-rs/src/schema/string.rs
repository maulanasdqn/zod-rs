use crate::schema::Schema;
use serde_json::Value;
use std::sync::LazyLock;
use zod_rs_util::{
    StringFormat, ValidateResult, ValidationError, ValidationOrigin, ValidationType,
};

static EMAIL_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

#[derive(Debug, Clone)]
pub struct StringSchema {
    min_length: Option<usize>,
    max_length: Option<usize>,
    starts_with: Option<String>,
    ends_with: Option<String>,
    includes: Option<String>,
    pattern: Option<regex::Regex>,
    email: bool,
    url: bool,
}

impl StringSchema {
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            starts_with: None,
            ends_with: None,
            includes: None,
            pattern: None,
            email: false,
            url: false,
        }
    }

    pub fn min(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn length(self, len: usize) -> Self {
        self.min(len).max(len)
    }

    pub fn starts_with(mut self, val: &str) -> Self {
        self.starts_with = Some(val.into());
        self
    }

    pub fn ends_with(mut self, val: &str) -> Self {
        self.ends_with = Some(val.into());
        self
    }

    pub fn includes(mut self, val: &str) -> Self {
        self.includes = Some(val.into());
        self
    }

    /// Sets a regex pattern for validation.
    ///
    /// # Panics
    /// Panics if the pattern is not a valid regex. Use `try_regex()` for fallible version.
    pub fn regex(mut self, pattern: &str) -> Self {
        self.pattern = Some(
            regex::Regex::new(pattern)
                .unwrap_or_else(|e| panic!("Invalid regex pattern '{}': {}", pattern, e)),
        );
        self
    }

    /// Sets a regex pattern for validation, returning an error if the pattern is invalid.
    pub fn try_regex(mut self, pattern: &str) -> Result<Self, regex::Error> {
        self.pattern = Some(regex::Regex::new(pattern)?);
        Ok(self)
    }

    pub fn email(mut self) -> Self {
        self.email = true;
        self
    }

    pub fn url(mut self) -> Self {
        self.url = true;
        self
    }
}

impl Default for StringSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema<String> for StringSchema {
    fn validate(&self, value: &Value) -> ValidateResult<String> {
        let string_val = match value.as_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(ValidationError::invalid_type(
                    ValidationType::String,
                    ValidationType::from(value),
                )
                .into());
            }
        };

        if let Some(min) = self.min_length {
            if string_val.len() < min {
                return Err(ValidationError::too_small(
                    ValidationOrigin::String,
                    min.to_string(),
                    true,
                )
                .into());
            }
        }

        if let Some(max) = self.max_length {
            if string_val.len() > max {
                return Err(ValidationError::too_big(
                    ValidationOrigin::String,
                    max.to_string(),
                    true,
                )
                .into());
            }
        }

        if let Some(starts_with) = &self.starts_with {
            if !string_val.starts_with(starts_with) {
                return Err(ValidationError::invalid_format(
                    StringFormat::StartsWith,
                    Some(starts_with.to_string()),
                )
                .into());
            }
        }

        if let Some(ends_with) = &self.ends_with {
            if !string_val.ends_with(ends_with) {
                return Err(ValidationError::invalid_format(
                    StringFormat::EndsWith,
                    Some(ends_with.to_string()),
                )
                .into());
            }
        }

        if let Some(includes) = &self.includes {
            if !string_val.contains(includes) {
                return Err(ValidationError::invalid_format(
                    StringFormat::Includes,
                    Some(includes.to_string()),
                )
                .into());
            }
        }

        if let Some(pattern) = &self.pattern {
            if !pattern.is_match(&string_val) {
                return Err(ValidationError::invalid_format(
                    StringFormat::Regex,
                    Some(pattern.to_string()),
                )
                .into());
            }
        }

        if self.email && !is_valid_email(&string_val) {
            return Err(
                ValidationError::invalid_format(StringFormat::custom("email"), None).into(),
            );
        }

        if self.url && !is_valid_url(&string_val) {
            return Err(ValidationError::invalid_format(StringFormat::custom("url"), None).into());
        }

        Ok(string_val)
    }
}

fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn string() -> StringSchema {
    StringSchema::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_validation() {
        let schema = string().min(3).max(10);

        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!("hi")).is_err());
        assert!(schema.validate(&json!("this is too long")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_string_starts_with() {
        let schema = string().starts_with("john");

        assert!(schema.validate(&json!("john doe")).is_ok());
        assert!(schema.validate(&json!("marry jane")).is_err());
    }

    #[test]
    fn test_string_ends_with() {
        let schema = string().ends_with("jane");

        assert!(schema.validate(&json!("john doe")).is_err());
        assert!(schema.validate(&json!("marry jane")).is_ok());
    }

    #[test]
    fn test_string_includes() {
        let schema = string().includes("25 years old");

        assert!(schema
            .validate(&json!("I am an 25 years old art director."))
            .is_ok());
        assert!(schema
            .validate(&json!("I AM AN 25 YEARS OLD ART DIRECTOR"))
            .is_err());
    }

    // ==================== EDGE CASE TESTS ====================

    // Boundary Conditions
    #[test]
    fn test_empty_string_with_min_zero() {
        let schema = string().min(0);
        assert!(schema.validate(&json!("")).is_ok());
    }

    #[test]
    fn test_string_exactly_at_min_boundary() {
        let schema = string().min(5);
        assert!(schema.validate(&json!("hello")).is_ok()); // exactly 5 chars
        assert!(schema.validate(&json!("hell")).is_err()); // 4 chars
    }

    #[test]
    fn test_string_exactly_at_max_boundary() {
        let schema = string().max(5);
        assert!(schema.validate(&json!("hello")).is_ok()); // exactly 5 chars
        assert!(schema.validate(&json!("hello!")).is_err()); // 6 chars
    }

    #[test]
    fn test_string_length_exact() {
        let schema = string().length(5);
        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!("hi")).is_err());
        assert!(schema.validate(&json!("hello!")).is_err());
    }

    // Unicode and Multi-byte Characters
    #[test]
    fn test_unicode_emoji() {
        let schema = string();
        assert!(schema.validate(&json!("🦀")).is_ok());
        assert_eq!(schema.validate(&json!("🦀")).unwrap(), "🦀");
    }

    #[test]
    fn test_unicode_chinese() {
        let schema = string();
        assert!(schema.validate(&json!("你好")).is_ok());
        assert_eq!(schema.validate(&json!("你好")).unwrap(), "你好");
    }

    #[test]
    fn test_unicode_mixed() {
        let schema = string();
        assert!(schema.validate(&json!("Hello 🦀 世界")).is_ok());
    }

    #[test]
    fn test_unicode_length_bytes_vs_chars() {
        // Note: Rust's len() counts bytes, not characters
        // "🦀" is 4 bytes but 1 character
        let schema = string().min(1).max(10);
        // This tests that we're counting bytes (current behavior)
        assert!(schema.validate(&json!("🦀")).is_ok());
    }

    // Pattern Edge Cases
    #[test]
    fn test_starts_with_empty_pattern() {
        let schema = string().starts_with("");
        assert!(schema.validate(&json!("anything")).is_ok());
        assert!(schema.validate(&json!("")).is_ok());
    }

    #[test]
    fn test_ends_with_empty_pattern() {
        let schema = string().ends_with("");
        assert!(schema.validate(&json!("anything")).is_ok());
    }

    #[test]
    fn test_includes_empty_pattern() {
        let schema = string().includes("");
        assert!(schema.validate(&json!("anything")).is_ok());
    }

    #[test]
    fn test_pattern_longer_than_string() {
        let schema = string().starts_with("very long pattern");
        assert!(schema.validate(&json!("short")).is_err());
    }

    #[test]
    fn test_unicode_pattern_matching() {
        let schema = string().starts_with("🦀");
        assert!(schema.validate(&json!("🦀 is a crab")).is_ok());
        assert!(schema.validate(&json!("crab 🦀")).is_err());
    }

    #[test]
    fn test_case_sensitivity() {
        let schema = string().starts_with("Hello");
        assert!(schema.validate(&json!("Hello World")).is_ok());
        assert!(schema.validate(&json!("hello World")).is_err());
    }

    // Regex Edge Cases
    #[test]
    fn test_regex_empty_pattern() {
        let schema = string().regex("");
        assert!(schema.validate(&json!("anything")).is_ok());
        assert!(schema.validate(&json!("")).is_ok());
    }

    #[test]
    fn test_regex_anchored() {
        let schema = string().regex("^start");
        assert!(schema.validate(&json!("start here")).is_ok());
        assert!(schema.validate(&json!("not start")).is_err());
    }

    #[test]
    fn test_regex_end_anchor() {
        let schema = string().regex("end$");
        assert!(schema.validate(&json!("the end")).is_ok());
        assert!(schema.validate(&json!("end here")).is_err());
    }

    #[test]
    fn test_regex_special_chars() {
        let schema = string().regex(r"\d+");
        assert!(schema.validate(&json!("abc123def")).is_ok());
        assert!(schema.validate(&json!("no digits")).is_err());
    }

    #[test]
    fn test_try_regex_valid() {
        let schema = string().try_regex(r"\d+").unwrap();
        assert!(schema.validate(&json!("123")).is_ok());
    }

    #[test]
    fn test_try_regex_invalid() {
        let result = string().try_regex("[invalid");
        assert!(result.is_err());
    }

    // Email Validation Edge Cases
    #[test]
    fn test_email_valid() {
        let schema = string().email();
        assert!(schema.validate(&json!("user@example.com")).is_ok());
    }

    #[test]
    fn test_email_missing_at() {
        let schema = string().email();
        assert!(schema.validate(&json!("userexample.com")).is_err());
    }

    #[test]
    fn test_email_multiple_at() {
        let schema = string().email();
        assert!(schema.validate(&json!("user@@example.com")).is_err());
    }

    #[test]
    fn test_email_with_spaces() {
        let schema = string().email();
        assert!(schema.validate(&json!(" user@example.com")).is_err());
        assert!(schema.validate(&json!("user@example.com ")).is_err());
        assert!(schema.validate(&json!("user @example.com")).is_err());
    }

    #[test]
    fn test_email_missing_domain() {
        let schema = string().email();
        assert!(schema.validate(&json!("user@")).is_err());
    }

    #[test]
    fn test_email_missing_tld() {
        let schema = string().email();
        assert!(schema.validate(&json!("user@domain")).is_err());
    }

    // URL Validation Edge Cases
    #[test]
    fn test_url_valid_https() {
        let schema = string().url();
        assert!(schema.validate(&json!("https://example.com")).is_ok());
    }

    #[test]
    fn test_url_valid_http() {
        let schema = string().url();
        assert!(schema.validate(&json!("http://example.com")).is_ok());
    }

    #[test]
    fn test_url_just_protocol() {
        let schema = string().url();
        // Note: current implementation accepts this (minimal validation)
        assert!(schema.validate(&json!("https://")).is_ok());
    }

    #[test]
    fn test_url_missing_protocol() {
        let schema = string().url();
        assert!(schema.validate(&json!("example.com")).is_err());
        assert!(schema.validate(&json!("www.example.com")).is_err());
    }

    #[test]
    fn test_url_with_path() {
        let schema = string().url();
        assert!(schema.validate(&json!("https://example.com/path/to/page")).is_ok());
    }

    #[test]
    fn test_url_with_query() {
        let schema = string().url();
        assert!(schema.validate(&json!("https://example.com?foo=bar")).is_ok());
    }

    #[test]
    fn test_url_with_fragment() {
        let schema = string().url();
        assert!(schema.validate(&json!("https://example.com#section")).is_ok());
    }

    // Type Rejection
    #[test]
    fn test_rejects_null() {
        let schema = string();
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_rejects_boolean() {
        let schema = string();
        assert!(schema.validate(&json!(true)).is_err());
        assert!(schema.validate(&json!(false)).is_err());
    }

    #[test]
    fn test_rejects_array() {
        let schema = string();
        assert!(schema.validate(&json!(["a", "b"])).is_err());
    }

    #[test]
    fn test_rejects_object() {
        let schema = string();
        assert!(schema.validate(&json!({"key": "value"})).is_err());
    }

    // Constraint Conflicts
    #[test]
    fn test_impossible_constraint_min_greater_than_max() {
        let schema = string().min(10).max(5);
        // All strings will fail validation
        assert!(schema.validate(&json!("hello")).is_err());
        assert!(schema.validate(&json!("hi")).is_err());
        assert!(schema.validate(&json!("hello world")).is_err());
    }

    // Combined Constraints
    #[test]
    fn test_combined_min_max_and_pattern() {
        let schema = string().min(3).max(10).starts_with("test");
        assert!(schema.validate(&json!("testing")).is_ok());
        assert!(schema.validate(&json!("te")).is_err()); // too short
        assert!(schema.validate(&json!("testing123456")).is_err()); // too long
        assert!(schema.validate(&json!("hello")).is_err()); // wrong prefix
    }

    #[test]
    fn test_email_and_length() {
        let schema = string().email().max(20);
        assert!(schema.validate(&json!("a@b.com")).is_ok());
        assert!(schema.validate(&json!("verylongemail@verylongdomain.com")).is_err());
    }
}
