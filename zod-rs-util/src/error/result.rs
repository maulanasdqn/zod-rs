use crate::error::issue::ValidationIssue;
use crate::locales::Locale;
use crate::ValidationError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for issue in &self.issues {
            write!(f, "\n  - {issue}")?;
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

    pub fn local(&self, lang: Locale) -> String {
        let issues = self
            .issues
            .iter()
            .map(|issue| issue.local(lang))
            .collect::<Vec<_>>();

        issues.join("\n")
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
