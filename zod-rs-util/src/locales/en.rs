use crate::{
    error::{NumberConstraint, StringFormat, ValidationError, ValidationOrigin},
    locales::{Localizer, Sizable},
};
use std::{collections::HashMap, sync::LazyLock};

static NOUNS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("regex", "input"),
        ("email", "email address"),
        ("url", "URL"),
        ("emoji", "emoji"),
        ("uuid", "UUID"),
        ("uuidv4", "UUIDv4"),
        ("uuidv6", "UUIDv6"),
        ("nanoid", "nanoid"),
        ("guid", "GUID"),
        ("cuid", "cuid"),
        ("cuid2", "cuid2"),
        ("ulid", "ULID"),
        ("xid", "XID"),
        ("ksuid", "KSUID"),
        ("datetime", "ISO datetime"),
        ("date", "ISO date"),
        ("time", "ISO time"),
        ("duration", "ISO duration"),
        ("ipv4", "IPv4 address"),
        ("ipv6", "IPv6 address"),
        ("cidrv4", "IPv4 range"),
        ("cidrv6", "IPv6 range"),
        ("base64", "base64-encoded string"),
        ("base64url", "base64url-encoded string"),
        ("json_string", "JSON string"),
        ("e164", "E.164 number"),
        ("jwt", "JWT"),
        ("template_literal", "input"),
    ])
});

fn get_noun(key: &str) -> &str {
    NOUNS.get(key).copied().unwrap_or(key)
}

static SIZABLES: LazyLock<HashMap<ValidationOrigin, Sizable>> = LazyLock::new(|| {
    HashMap::from([
        (
            ValidationOrigin::String,
            Sizable::new("characters", "to have"),
        ),
        (ValidationOrigin::Array, Sizable::new("items", "to have")),
    ])
});

fn get_sizable(key: &ValidationOrigin) -> Option<&'static Sizable> {
    SIZABLES.get(key)
}

#[derive(Debug, Default, Clone)]
pub struct En;

impl Localizer for En {
    fn localize(&self, error: &ValidationError) -> String {
        match error {
            ValidationError::InvalidType { expected, input } => {
                format!("Invalid input: expected {expected}, received {input}")
            }
            ValidationError::InvalidValue { value } => {
                format!("Invalid input: expected {value}")
            }
            ValidationError::InvalidValues { values } => {
                format!("Invalid option: expected one of {}", values.join(" | "))
            }
            ValidationError::TooBig {
                origin,
                maximum,
                inclusive,
            } => {
                let adj = if *inclusive { "<=" } else { "<" };

                if let Some(sizing) = get_sizable(origin) {
                    return format!(
                        "Too big: expected {} {} {} {} {}",
                        origin, sizing.verb, adj, maximum, sizing.unit
                    );
                }

                format!("Too big: expected {origin} to have {adj} {maximum}")
            }
            ValidationError::TooSmall {
                origin,
                minimum,
                inclusive,
            } => {
                let adj = if *inclusive { ">=" } else { ">" };

                if let Some(sizing) = get_sizable(origin) {
                    return format!(
                        "Too small: expected {} {} {} {} {}",
                        origin, sizing.verb, adj, minimum, sizing.unit
                    );
                }

                format!("Too small: expected {origin} to have {adj} {minimum}")
            }
            ValidationError::InvalidFormat { format, detail } => match format {
                StringFormat::StartsWith => format!(
                    "Invalid value: must start with \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::EndsWith => format!(
                    "Invalid value: must end with \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Includes => format!(
                    "Invalid value: must include \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Regex => format!(
                    "Invalid value: must match pattern {}",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Custom(format) => {
                    let format = get_noun(format);

                    format!("Invalid {format}")
                }
            },
            ValidationError::InvalidNumber { constraint } => match constraint {
                NumberConstraint::Finite => "Invalid number: must be finite".into(),
                NumberConstraint::Positive => "Invalid number: must be positive".into(),
                NumberConstraint::Negative => "Invalid number: must be negative".into(),
                NumberConstraint::NonNegative => "Invalid number: must be non-negative".into(),
                NumberConstraint::NonPositive => "Invalid number: must be non-positive".into(),
            },
            ValidationError::UnrecognizedKeys { keys } => {
                format!(
                    "Unrecognized key{}: {}",
                    if keys.len() > 1 { "s" } else { "" },
                    keys.join(", ")
                )
            }
            ValidationError::InvalidUnion { .. } => "Invalid input".into(),
            ValidationError::Required => "Value is required but was not provided".into(),
            ValidationError::Custom { message } => message.into(),
        }
    }
}
