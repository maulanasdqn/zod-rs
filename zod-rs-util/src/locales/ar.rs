use std::{collections::HashMap, sync::LazyLock};

use crate::{
    error::{NumberConstraint, StringFormat, ValidationError, ValidationOrigin},
    locales::{Localizer, Sizable},
};

static NOUNS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
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
});

fn get_noun(key: &str) -> &str {
    NOUNS.get(key).copied().unwrap_or(key)
}

static SIZABLES: LazyLock<HashMap<ValidationOrigin, Sizable>> = LazyLock::new(|| {
    HashMap::from([
        (ValidationOrigin::String, Sizable::new("حرف", "أن يحوي")),
        (ValidationOrigin::Array, Sizable::new("عنصر", "أن يحوي")),
    ])
});

fn get_sizable(key: &ValidationOrigin) -> Option<&'static Sizable> {
    SIZABLES.get(key)
}

#[derive(Debug, Default, Clone)]
pub struct Ar;

impl Localizer for Ar {
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

                if let Some(sizing) = get_sizable(origin) {
                    return format!(
                        " أكبر من اللازم: يفترض {} {} {} {} {}",
                        origin, sizing.verb, adj, maximum, sizing.unit
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

                if let Some(sizing) = get_sizable(origin) {
                    return format!(
                        "أصغر من اللازم: يفترض لـ {} {} {} {} {}",
                        origin, sizing.verb, adj, minimum, sizing.unit
                    );
                }

                format!("أصغر من اللازم: يفترض لـ {origin} أن يكون {adj} {minimum}")
            }
            ValidationError::InvalidFormat { format, detail } => match format {
                StringFormat::StartsWith => format!(
                    "نَص غير مقبول: يجب أن يبدأ بـ \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::EndsWith => format!(
                    "نَص غير مقبول: يجب أن ينتهي بـ \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Includes => format!(
                    "نَص غير مقبول: يجب أن يتضمَّن \"{}\"",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Regex => format!(
                    "نَص غير مقبول: يجب أن يطابق النمط {}",
                    detail.clone().unwrap_or_default()
                ),
                StringFormat::Custom(format) => {
                    let format = get_noun(format);

                    format!("{format} غير مقبول")
                }
            },
            ValidationError::InvalidNumber { constraint } => match constraint {
                NumberConstraint::Finite => "رقم غير صالح: يجب أن يكون محدودًا".into(),
                NumberConstraint::Positive => "رقم غير صالح: يجب أن يكون موجبًا".into(),
                NumberConstraint::Negative => "رقم غير صالح: يجب أن يكون سالبًا".into(),
                NumberConstraint::NonNegative => "رقم غير صالح: يجب ألا يكون سالبًا".into(),
                NumberConstraint::NonPositive => "رقم غير صالح: يجب ألا يكون موجبًا".into(),
            },
            ValidationError::UnrecognizedKeys { keys } => {
                format!(
                    "معرف{} غريب{}: {}",
                    if keys.len() > 1 { "ات" } else { "" },
                    if keys.len() > 1 { "ة" } else { "" },
                    keys.join("، ")
                )
            }
            ValidationError::InvalidUnion { .. } => "مدخل غير مقبول".into(),
            ValidationError::Required => "القيمة مطلوبة ولكن لم يتم تقديمها".into(),
            ValidationError::Custom { message } => message.into(),
        }
    }
}
