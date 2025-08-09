use crate::{
    error::{ValidationError, ValidationFormat, ValidationOrigin},
    locales::{Localizer, Sizable},
};
use std::collections::HashMap;

pub struct En;

impl Localizer for En {
    fn sizable(&self) -> HashMap<ValidationOrigin, Sizable> {
        HashMap::from([
            (
                ValidationOrigin::String,
                Sizable::new("characters", "to have"),
            ),
            (ValidationOrigin::File, Sizable::new("bytes", "to have")),
            (ValidationOrigin::Array, Sizable::new("items", "to have")),
            (ValidationOrigin::Set, Sizable::new("items", "to have")),
        ])
    }

    fn nouns(&self) -> HashMap<&'static str, &'static str> {
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
    }

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

                if let Some(sizing) = self.sizable().get(origin) {
                    return format!(
                        "Too big: expected {} to have {} {} {}",
                        origin, adj, maximum, sizing.unit
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

                if let Some(sizing) = self.sizable().get(origin) {
                    return format!(
                        "Too big: expected {} to have {} {} {}",
                        origin, adj, minimum, sizing.unit
                    );
                }

                format!("Too small: expected {origin} to have {adj} {minimum}")
            }
            ValidationError::InvalidFormat { format, detail } => match format {
                ValidationFormat::StartsWith => format!(
                    "Invalid string: must start with \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::EndsWith => format!(
                    "Invalid string: must end with \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Includes => format!(
                    "Invalid string: must include \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Regex => format!(
                    "Invalid string: must match pattern {}",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Custom(format) => {
                    let format = self
                        .nouns()
                        .get(format.as_str())
                        .map_or(format.as_str(), |v| v);

                    format!("Invalid {format}")
                }
            },
            ValidationError::NotMultipleOf { divisor } => {
                format!("Invalid number: must be a multiple of {divisor}")
            }
            ValidationError::UnrecognizedKeys { keys } => {
                format!(
                    "Unrecognized key{}: {}",
                    if keys.len() > 1 { "s" } else { "" },
                    keys.join(", ")
                )
            }
            ValidationError::InvalidKey { origin } => format!("Invalid key in {origin}"),
            ValidationError::InvalidUnion { .. } => "Invalid input".into(),
            ValidationError::InvalidElement { origin } => format!("Invalid value in {origin}"),
            ValidationError::Required => "Value is required but was not provided".into(),
            ValidationError::Custom { message } => message.into(),
        }
    }
}
