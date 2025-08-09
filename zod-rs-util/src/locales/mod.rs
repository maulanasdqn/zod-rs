pub mod ar;
pub mod en;
pub mod fr;

use crate::{
    error::ValidationOrigin,
    locales::{ar::Ar, en::En, fr::Fr},
    ValidationError,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Fr,
    Ar,
}

// Might not need all this
impl Locale {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "fr" => Locale::Fr,
            "ar" => Locale::Ar,
            _ => Locale::En, // default to English, maybe?
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Fr => "fr",
            Locale::Ar => "ar",
        }
    }
}

pub trait Localizer {
    fn sizable(&self) -> HashMap<ValidationOrigin, Sizable>;
    fn nouns(&self) -> HashMap<&'static str, &'static str>;
    fn localize(&self, error: &ValidationError) -> String;
}

pub fn localizer(locale: Locale) -> Box<dyn Localizer> {
    match locale {
        Locale::En => Box::new(En),
        Locale::Ar => Box::new(Ar),
        Locale::Fr => Box::new(Fr),
    }
}

#[derive(Debug, Clone)]
pub struct Sizable {
    unit: &'static str,
    verb: &'static str,
}

impl Sizable {
    fn new(unit: &'static str, verb: &'static str) -> Self {
        Self { unit, verb }
    }
}
