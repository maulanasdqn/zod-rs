pub mod issue;
pub mod result;

use crate::{
    locales::{localizer, Locale},
    ValidationIssue,
};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    Required,
    InvalidType {
        expected: ValidationType,
        input: ValidationType,
    },
    InvalidValue {
        value: String,
    },
    InvalidValues {
        values: Vec<String>,
    },
    TooBig {
        origin: ValidationOrigin,
        maximum: String,
        inclusive: bool,
    },
    TooSmall {
        origin: ValidationOrigin,
        minimum: String,
        inclusive: bool,
    },
    InvalidFormat {
        format: StringFormat,
        detail: Option<String>,
    },
    InvalidNumber {
        constraint: NumberConstraint,
    },
    UnrecognizedKeys {
        keys: Vec<String>,
    },
    InvalidUnion {
        issues: Vec<ValidationIssue>,
    },
    Custom {
        message: String,
    },
}

impl ValidationError {
    pub fn local(&self, locale: Locale) -> String {
        localizer(locale).localize(self)
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.local(Locale::En))?;
        Ok(())
    }
}

impl ValidationError {
    pub fn required() -> Self {
        Self::Required
    }

    pub fn invalid_type(expected: ValidationType, input: ValidationType) -> Self {
        Self::InvalidType { expected, input }
    }

    pub fn invalid_value(value: impl Into<String>) -> Self {
        Self::InvalidValue {
            value: value.into(),
        }
    }

    pub fn invalid_values(values: Vec<String>) -> Self {
        Self::InvalidValues { values }
    }

    pub fn too_big(origin: ValidationOrigin, maximum: impl Into<String>, inclusive: bool) -> Self {
        Self::TooBig {
            origin,
            maximum: maximum.into(),
            inclusive,
        }
    }

    pub fn too_small(
        origin: ValidationOrigin,
        minimum: impl Into<String>,
        inclusive: bool,
    ) -> Self {
        Self::TooSmall {
            origin,
            minimum: minimum.into(),
            inclusive,
        }
    }

    pub fn invalid_format(format: StringFormat, detail: Option<String>) -> Self {
        Self::InvalidFormat { format, detail }
    }

    pub fn invalid_number(constraint: NumberConstraint) -> Self {
        Self::InvalidNumber { constraint }
    }

    pub fn unrecognized_keys(keys: Vec<String>) -> Self {
        Self::UnrecognizedKeys { keys }
    }

    pub fn invalid_union(issues: Vec<ValidationIssue>) -> Self {
        Self::InvalidUnion { issues }
    }

    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationOrigin {
    String,
    Array,
    Number,
}

impl fmt::Display for ValidationOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ValidationOrigin::String => "string",
            ValidationOrigin::Array => "array",
            ValidationOrigin::Number => "number",
        };

        write!(f, "{value}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
    Undefined,
    Custom(String),
}

impl ValidationType {
    pub fn custom(val_type: impl Into<String>) -> Self {
        Self::Custom(val_type.into())
    }
}

impl From<&Value> for ValidationType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => ValidationType::Null,
            Value::Bool(_) => ValidationType::Bool,
            Value::Number(_) => ValidationType::Number,
            Value::String(_) => ValidationType::String,
            Value::Array(_) => ValidationType::Array,
            Value::Object(_) => ValidationType::Object,
        }
    }
}

impl fmt::Display for ValidationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ValidationType::Null => "null",
            ValidationType::Bool => "bool",
            ValidationType::Number => "number",
            ValidationType::String => "string",
            ValidationType::Array => "array",
            ValidationType::Object => "object",
            ValidationType::Undefined => "undefined",
            ValidationType::Custom(_type) => _type,
        };

        write!(f, "{value}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringFormat {
    StartsWith,
    EndsWith,
    Includes,
    Regex,
    Custom(String),
}

impl StringFormat {
    pub fn custom(val_type: impl Into<String>) -> Self {
        Self::Custom(val_type.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberConstraint {
    Finite,
    Positive,
    Negative,
    NonNegative,
    NonPositive,
}
