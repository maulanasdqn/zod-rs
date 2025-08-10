pub mod ar;
pub mod en;

use crate::{
    locales::{ar::Ar, en::En},
    ValidationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Ar,
}

pub trait Localizer {
    fn localize(&self, error: &ValidationError) -> String;
}

pub fn localizer(locale: Locale) -> Box<dyn Localizer> {
    match locale {
        Locale::En => Box::new(En),
        Locale::Ar => Box::new(Ar),
    }
}

#[derive(Debug, Clone)]
struct Sizable {
    unit: &'static str,
    verb: &'static str,
}

impl Sizable {
    fn new(unit: &'static str, verb: &'static str) -> Self {
        Self { unit, verb }
    }
}
