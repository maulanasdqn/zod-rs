use crate::{
    error::{ValidationError, ValidationFormat, ValidationOrigin},
    locales::{Localizer, Sizable},
};
use std::collections::HashMap;

pub struct Fr;

impl Localizer for Fr {
    fn sizable(&self) -> HashMap<ValidationOrigin, Sizable> {
        HashMap::from([
            (
                ValidationOrigin::String,
                Sizable::new("caractères", "avoir"),
            ),
            (ValidationOrigin::File, Sizable::new("octets", "avoir")),
            (ValidationOrigin::Array, Sizable::new("éléments", "avoir")),
            (ValidationOrigin::Set, Sizable::new("éléments", "avoir")),
        ])
    }

    fn nouns(&self) -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("regex", "entrée"),
            ("email", "adresse e-mail"),
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
            ("datetime", "date et heure ISO"),
            ("date", "date ISO"),
            ("time", "heure ISO"),
            ("duration", "durée ISO"),
            ("ipv4", "adresse IPv4"),
            ("ipv6", "adresse IPv6"),
            ("cidrv4", "plage IPv4"),
            ("cidrv6", "plage IPv6"),
            ("base64", "chaîne encodée en base64"),
            ("base64url", "chaîne encodée en base64url"),
            ("json_string", "chaîne JSON"),
            ("e164", "numéro E.164"),
            ("jwt", "JWT"),
            ("template_literal", "entrée"),
        ])
    }

    fn localize(&self, error: &ValidationError) -> String {
        match error {
            ValidationError::InvalidType { expected, input } => {
                format!("Entrée invalide: {expected} attendu, {input} reçu")
            }
            ValidationError::InvalidValue { value } => {
                format!("Entrée invalide: {value} attendu")
            }
            ValidationError::InvalidValues { values } => {
                format!(
                    "Option invalide : une valeur parmi {} attendue",
                    values.join(" | ")
                )
            }
            ValidationError::TooBig {
                origin,
                maximum,
                inclusive,
            } => {
                let adj = if *inclusive { "<=" } else { "<" };

                if let Some(sizing) = self.sizable().get(origin) {
                    return format!(
                        "Trop grand: {} doit {} {}{} {}",
                        origin, sizing.verb, adj, maximum, sizing.unit
                    );
                }

                format!("Trop grand: {origin} doit être {adj}{maximum}")
            }
            ValidationError::TooSmall {
                origin,
                minimum,
                inclusive,
            } => {
                let adj = if *inclusive { ">=" } else { ">" };
                if let Some(sizing) = self.sizable().get(origin) {
                    return format!(
                        "Trop petit: {} doit {} {}{} {}",
                        origin, sizing.verb, adj, minimum, sizing.unit
                    );
                }
                format!("Trop petit: {origin} doit être {adj}{minimum}")
            }
            ValidationError::InvalidFormat { format, detail } => match format {
                ValidationFormat::StartsWith => format!(
                    "Chaîne invalide: doit commencer par \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::EndsWith => format!(
                    "Chaîne invalide: doit se terminer par \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Includes => format!(
                    "Chaîne invalide: doit inclure \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Regex => format!(
                    "Chaîne invalide: doit correspondre au modèle {}",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Custom(format) => {
                    let format = self
                        .nouns()
                        .get(format.as_str())
                        .map_or(format.as_str(), |v| v);

                    format!("{format} invalide")
                }
            },
            ValidationError::NotMultipleOf { divisor } => {
                format!("Nombre invalide: doit être un multiple de {divisor}")
            }
            ValidationError::UnrecognizedKeys { keys } => {
                format!(
                    "Clé{} non reconnue${}: {}",
                    if keys.len() > 1 { "s" } else { "" },
                    if keys.len() > 1 { "s" } else { "" },
                    keys.join(", ")
                )
            }
            ValidationError::InvalidKey { origin } => format!("Clé invalide dans {origin}"),
            ValidationError::InvalidUnion { .. } => "Entrée invalide".into(),
            ValidationError::InvalidElement { origin } => {
                format!("Valeur invalide dans {origin}")
            }
            ValidationError::Required => "La valeur est requise mais n’a pas été fournie".into(),
            ValidationError::Custom { message } => message.into(),
        }
    }
}
