use crate::{locales::Locale, ValidationError};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    pub path: Vec<String>,
    pub error: ValidationError,
}

impl ValidationIssue {
    pub fn local(&self, lang: Locale) -> String {
        if self.path.is_empty() {
            self.error.local(lang)
        } else {
            format!("{}: {}", self.path.join("."), self.error.local(lang))
        }
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.error.local(Locale::En))
        } else {
            write!(
                f,
                "{}: {}",
                self.path.join("."),
                self.error.local(Locale::En)
            )
        }
    }
}
