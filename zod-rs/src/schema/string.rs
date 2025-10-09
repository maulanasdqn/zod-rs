use crate::schema::Schema;
use serde_json::Value;
use zod_rs_util::{
    StringFormat, ValidateResult, ValidationError, ValidationOrigin, ValidationType,
};

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

    pub fn regex(mut self, pattern: &str) -> Self {
        self.pattern = Some(regex::Regex::new(pattern).expect("Invalid regex pattern"));
        self
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
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
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
}
