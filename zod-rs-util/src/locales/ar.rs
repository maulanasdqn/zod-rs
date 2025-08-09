use std::collections::HashMap;

use crate::{
    error::{ValidationError, ValidationFormat, ValidationOrigin},
    locales::{Localizer, Sizable},
};

pub struct Ar;

impl Localizer for Ar {
    fn sizable(&self) -> HashMap<ValidationOrigin, Sizable> {
        HashMap::from([
            (ValidationOrigin::String, Sizable::new("حرف", "أن يحوي")),
            (ValidationOrigin::File, Sizable::new("بايت", "أن يحوي")),
            (ValidationOrigin::Array, Sizable::new("عنصر", "أن يحوي")),
            (ValidationOrigin::Set, Sizable::new("عنصر", "أن يحوي")),
        ])
    }

    fn nouns(&self) -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("regex", "مدخل"),
            ("email", "بريد إلكتروني"),
            ("url", "رابط"),
            ("emoji", "إيموجي"),
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
            ("datetime", "تاريخ ووقت بمعيار ISO"),
            ("date", "تاريخ بمعيار ISO"),
            ("time", "وقت بمعيار ISO"),
            ("duration", "مدة بمعيار ISO"),
            ("ipv4", "عنوان IPv4"),
            ("ipv6", "عنوان IPv6"),
            ("cidrv4", "مدى عناوين بصيغة IPv4"),
            ("cidrv6", "مدى عناوين بصيغة IPv6"),
            ("base64", "نَص بترميز base64-encoded"),
            ("base64url", "نَص بترميز base64url-encoded"),
            ("json_string", "نَص على هيئة JSON"),
            ("e164", "رقم هاتف بمعيار E.164"),
            ("jwt", "JWT"),
            ("template_literal", "مدخل"),
        ])
    }

    fn localize(&self, error: &ValidationError) -> String {
        match error {
            ValidationError::InvalidType { expected, input } => {
                format!("مدخلات غير مقبولة: يفترض إدخال {expected}، ولكن تم إدخال {input}")
            }
            ValidationError::InvalidValue { value } => {
                format!("مدخلات غير مقبولة: يفترض إدخال {value}")
            }
            ValidationError::InvalidValues { values } => {
                format!(
                    "اختيار غير مقبول: يتوقع انتقاء أحد هذه الخيارات: {}",
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
                        " أكبر من اللازم: يفترض أن تكون {} {} {} {}",
                        origin, adj, maximum, sizing.unit
                    );
                }

                format!("أكبر من اللازم: يفترض أن تكون {origin} {adj} {maximum}")
            }
            ValidationError::TooSmall {
                origin,
                minimum,
                inclusive,
            } => {
                let adj = if *inclusive { ">=" } else { ">" };

                if let Some(sizing) = self.sizable().get(origin) {
                    return format!(
                        "أصغر من اللازم: يفترض لـ {} أن يكون {} {} {}",
                        origin, adj, minimum, sizing.unit
                    );
                }

                format!("أصغر من اللازم: يفترض لـ {origin} أن يكون {adj} {minimum}")
            }
            ValidationError::InvalidFormat { format, detail } => match format {
                ValidationFormat::StartsWith => format!(
                    "نَص غير مقبول: يجب أن يبدأ بـ \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::EndsWith => format!(
                    "نَص غير مقبول: يجب أن ينتهي بـ \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Includes => format!(
                    "نَص غير مقبول: يجب أن يتضمَّن \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Regex => format!(
                    "نَص غير مقبول: يجب أن يطابق النمط {}",
                    detail.clone().unwrap_or_default()
                ),
                ValidationFormat::Custom(format) => {
                    let format = self
                        .nouns()
                        .get(format.as_str())
                        .map_or(format.as_str(), |v| v);

                    format!("{format} غير مقبول")
                }
            },
            ValidationError::NotMultipleOf { divisor } => {
                format!("رقم غير مقبول: يجب أن يكون من مضاعفات {divisor}")
            }
            ValidationError::UnrecognizedKeys { keys } => {
                format!(
                    "معرف{} غريب{}: {}",
                    if keys.len() > 1 { "ات" } else { "" },
                    if keys.len() > 1 { "ة" } else { "" },
                    keys.join("، ")
                )
            }
            ValidationError::InvalidKey { origin } => format!("`معرف غير مقبول في {origin}"),
            ValidationError::InvalidUnion { .. } => "مدخل غير مقبول".into(),
            ValidationError::InvalidElement { origin } => format!("مدخل غير مقبول في {origin}"),
            ValidationError::Required => "القيمة مطلوبة ولكن لم يتم تقديمها".into(),
            ValidationError::Custom { message } => message.into(),
        }
    }
}
