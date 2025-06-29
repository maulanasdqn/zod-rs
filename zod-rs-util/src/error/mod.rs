use std::error::Error as StdError;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    #[error("Value is required but was not provided")]
    Required,

    #[error("Invalid type: expected {expected}, got {actual}")]
    InvalidType { expected: String, actual: String },

    #[error("Value '{value}' is too small: minimum is {min}")]
    TooSmall { value: String, min: String },

    #[error("Value '{value}' is too big: maximum is {max}")]
    TooBig { value: String, max: String },

    #[error("String length {length} is invalid: {constraint}")]
    InvalidLength { length: usize, constraint: String },

    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },

    #[error("Pattern validation failed: {pattern}")]
    PatternMismatch { pattern: String },

    #[error("Value '{value}' is not included in allowed values")]
    NotIncluded { value: String },

    #[error("Array validation failed at index {index}: {error}")]
    ArrayElement {
        index: usize,
        error: Box<ValidationError>,
    },

    #[error("Object validation failed at key '{key}': {error}")]
    ObjectProperty {
        key: String,
        error: Box<ValidationError>,
    },

    #[error("Union validation failed: none of the variants matched")]
    UnionMismatch,

    #[error("Custom validation failed: {message}")]
    Custom { message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    pub path: Vec<String>,
    pub error: ValidationError,
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.error)
        } else {
            write!(f, "at {}: {}", self.path.join("."), self.error)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed with {} error(s):", self.issues.len())?;
        for issue in &self.issues {
            write!(f, "\n  - {}", issue)?;
        }
        Ok(())
    }
}

impl StdError for ValidationResult {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn with_error(error: ValidationError) -> Self {
        Self {
            issues: vec![ValidationIssue {
                path: Vec::new(),
                error,
            }],
        }
    }

    pub fn with_issue(issue: ValidationIssue) -> Self {
        Self {
            issues: vec![issue],
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.issues.push(ValidationIssue {
            path: Vec::new(),
            error,
        });
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn add_error_at_path(&mut self, path: Vec<String>, error: ValidationError) {
        self.issues.push(ValidationIssue { path, error });
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.issues.extend(other.issues);
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn len(&self) -> usize {
        self.issues.len()
    }

    pub fn prefix_path(&mut self, prefix: String) {
        for issue in &mut self.issues {
            issue.path.insert(0, prefix.clone());
        }
    }

    pub fn into_result<T>(self) -> Result<T, Self> {
        if self.is_empty() {
            panic!("Cannot convert empty validation result to error")
        } else {
            Err(self)
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ValidationError> for ValidationResult {
    fn from(error: ValidationError) -> Self {
        Self::with_error(error)
    }
}

impl From<ValidationIssue> for ValidationResult {
    fn from(issue: ValidationIssue) -> Self {
        Self::with_issue(issue)
    }
}

pub type ValidateResult<T> = Result<T, ValidationResult>;

impl ValidationError {
    pub fn required() -> Self {
        Self::Required
    }

    pub fn invalid_type(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::InvalidType {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn too_small(value: impl Into<String>, min: impl Into<String>) -> Self {
        Self::TooSmall {
            value: value.into(),
            min: min.into(),
        }
    }

    pub fn too_big(value: impl Into<String>, max: impl Into<String>) -> Self {
        Self::TooBig {
            value: value.into(),
            max: max.into(),
        }
    }

    pub fn invalid_length(length: usize, constraint: impl Into<String>) -> Self {
        Self::InvalidLength {
            length,
            constraint: constraint.into(),
        }
    }

    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            message: message.into(),
        }
    }

    pub fn pattern_mismatch(pattern: impl Into<String>) -> Self {
        Self::PatternMismatch {
            pattern: pattern.into(),
        }
    }

    pub fn not_included(value: impl Into<String>) -> Self {
        Self::NotIncluded {
            value: value.into(),
        }
    }

    pub fn array_element(index: usize, error: ValidationError) -> Self {
        Self::ArrayElement {
            index,
            error: Box::new(error),
        }
    }

    pub fn object_property(key: impl Into<String>, error: ValidationError) -> Self {
        Self::ObjectProperty {
            key: key.into(),
            error: Box::new(error),
        }
    }

    pub fn union_mismatch() -> Self {
        Self::UnionMismatch
    }

    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }
}
