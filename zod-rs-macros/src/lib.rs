use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta};

#[proc_macro_derive(ZodSchema, attributes(zod))]
pub fn derive_zod_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_validations = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_name_str = field_name.as_ref().unwrap().to_string();
                    let field_type = &field.ty;
                    let field_attrs = &field.attrs;

                    generate_field_validation_with_attrs(&field_name_str, field_type, field_attrs)
                });

                let expanded = quote! {
                    impl #name {
                        pub fn schema() -> impl zod_rs::Schema<serde_json::Value> {
                            zod_rs::object()
                                #(#field_validations)*
                        }

                        pub fn validate_and_parse(value: &serde_json::Value) -> Result<Self, zod_rs_util::ValidationResult> {
                            match Self::schema().validate(value) {
                                Ok(_) => {
                                    serde_json::from_value(value.clone())
                                        .map_err(|e| zod_rs_util::ValidationError::custom(format!("Deserialization failed: {}", e)).into())
                                }
                                Err(validation_result) => Err(validation_result)
                            }
                        }

                        pub fn from_json(json_str: &str) -> Result<Self, zod_rs_util::ParseError> {
                            let value: serde_json::Value = serde_json::from_str(json_str)?;
                            Ok(Self::validate_and_parse(&value)?)
                        }

                        pub fn validate_json(json_str: &str) -> Result<serde_json::Value, zod_rs_util::ParseError> {
                            let value: serde_json::Value = serde_json::from_str(json_str)?;
                            Self::schema().validate(&value)?;
                            Ok(value)
                        }
                    }
                };

                TokenStream::from(expanded)
            }
            Fields::Unnamed(_) => {
                let error = syn::Error::new_spanned(
                    &input,
                    "ZodSchema can only be derived for structs with named fields, not tuple structs",
                );
                TokenStream::from(error.to_compile_error())
            }
            Fields::Unit => {
                let error = syn::Error::new_spanned(
                    &input,
                    "ZodSchema can only be derived for structs with named fields, not unit structs",
                );
                TokenStream::from(error.to_compile_error())
            }
        },
        Data::Enum(_) => {
            let error = syn::Error::new_spanned(
                &input,
                "ZodSchema cannot be derived for enums. Consider using UnionSchema instead.",
            );
            TokenStream::from(error.to_compile_error())
        }
        Data::Union(_) => {
            let error = syn::Error::new_spanned(
                &input,
                "ZodSchema cannot be derived for unions",
            );
            TokenStream::from(error.to_compile_error())
        }
    }
}

#[derive(Default)]
struct ZodAttributes {
    min: Option<f64>,
    max: Option<f64>,
    length: Option<usize>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    starts_with: Option<String>,
    ends_with: Option<String>,
    includes: Option<String>,
    email: bool,
    url: bool,
    regex: Option<String>,
    positive: bool,
    negative: bool,
    nonnegative: bool,
    nonpositive: bool,
    int: bool,
    finite: bool,
}

fn parse_zod_attributes(attrs: &[Attribute]) -> ZodAttributes {
    let mut zod_attrs = ZodAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("zod") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens: Vec<_> = meta_list.tokens.clone().into_iter().collect();
                let mut i = 0;

                while i < tokens.len() {
                    let token_str = tokens[i].to_string();

                    match token_str.as_str() {
                        "min_length" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_number_from_parens(&value_token) {
                                    zod_attrs.min_length = Some(value);
                                }
                                i += 1; // Skip the value token
                            }
                        }
                        "max_length" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_number_from_parens(&value_token) {
                                    zod_attrs.max_length = Some(value);
                                }
                                i += 1;
                            }
                        }
                        "length" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_number_from_parens(&value_token) {
                                    zod_attrs.length = Some(value);
                                }
                                i += 1;
                            }
                        }
                        "min" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value_str) = extract_string_from_parens(&value_token) {
                                    if let Ok(value) = value_str.parse::<f64>() {
                                        zod_attrs.min = Some(value);
                                    }
                                }
                                i += 1;
                            }
                        }
                        "max" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value_str) = extract_string_from_parens(&value_token) {
                                    if let Ok(value) = value_str.parse::<f64>() {
                                        zod_attrs.max = Some(value);
                                    }
                                }
                                i += 1;
                            }
                        }
                        "starts_with" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_string_from_parens(&value_token) {
                                    zod_attrs.starts_with = Some(strip_quotes(&value));
                                }
                                i += 1;
                            }
                        }
                        "ends_with" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_string_from_parens(&value_token) {
                                    zod_attrs.ends_with = Some(strip_quotes(&value));
                                }
                                i += 1;
                            }
                        }
                        "includes" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_string_from_parens(&value_token) {
                                    zod_attrs.includes = Some(strip_quotes(&value));
                                }
                                i += 1;
                            }
                        }
                        "regex" => {
                            if i + 1 < tokens.len() {
                                let value_token = tokens[i + 1].to_string();
                                if let Some(value) = extract_string_from_parens(&value_token) {
                                    zod_attrs.regex = Some(strip_quotes(&value));
                                }
                                i += 1;
                            }
                        }
                        "email" => {
                            zod_attrs.email = true;
                        }
                        "url" => {
                            zod_attrs.url = true;
                        }
                        "positive" => {
                            zod_attrs.positive = true;
                        }
                        "negative" => {
                            zod_attrs.negative = true;
                        }
                        "nonnegative" => {
                            zod_attrs.nonnegative = true;
                        }
                        "nonpositive" => {
                            zod_attrs.nonpositive = true;
                        }
                        "int" => {
                            zod_attrs.int = true;
                        }
                        "finite" => {
                            zod_attrs.finite = true;
                        }
                        "," => {
                            // Skip commas
                        }
                        _ => {
                            // Skip unknown tokens
                        }
                    }

                    i += 1;
                }
            }
        }
    }

    zod_attrs
}

fn extract_number_from_parens(token: &str) -> Option<usize> {
    token
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .and_then(|inner| inner.parse::<usize>().ok())
}

fn extract_string_from_parens(token: &str) -> Option<String> {
    token
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .map(|s| s.to_string())
}

/// Safely removes surrounding quotes from a string value
fn strip_quotes(value: &str) -> String {
    // Try to strip regular quotes first
    if let Some(inner) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    // Try to strip raw string literal (r"...")
    if let Some(inner) = value.strip_prefix("r\"").and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    // Return as-is if no quotes
    value.to_string()
}

fn generate_field_validation_with_attrs(
    field_name: &str,
    field_type: &syn::Type,
    attrs: &[Attribute],
) -> proc_macro2::TokenStream {
    let zod_attrs = parse_zod_attributes(attrs);
    let is_optional = is_option_type(field_type);

    if is_optional {
        let inner_type = get_option_inner_type(field_type);
        let base_validation = generate_base_validation_with_attrs(&inner_type, &zod_attrs);
        quote! { .optional_field(#field_name, #base_validation) }
    } else {
        let base_validation = generate_base_validation_with_attrs(field_type, &zod_attrs);
        quote! { .field(#field_name, #base_validation) }
    }
}

fn generate_base_validation_with_attrs(
    field_type: &syn::Type,
    zod_attrs: &ZodAttributes,
) -> proc_macro2::TokenStream {
    if let syn::Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();

            match type_name.as_str() {
                "String" => {
                    let mut validation = quote! { zod_rs::string() };

                    if let Some(min) = zod_attrs.min_length {
                        validation = quote! { #validation.min(#min) };
                    }
                    if let Some(max) = zod_attrs.max_length {
                        validation = quote! { #validation.max(#max) };
                    }
                    if let Some(length) = zod_attrs.length {
                        validation = quote! { #validation.length(#length) };
                    }
                    if zod_attrs.email {
                        validation = quote! { #validation.email() };
                    }
                    if zod_attrs.url {
                        validation = quote! { #validation.url() };
                    }
                    if let Some(regex) = &zod_attrs.regex {
                        validation = quote! { #validation.regex(#regex) };
                    }
                    if let Some(starts_with) = &zod_attrs.starts_with {
                        validation = quote! { #validation.starts_with(#starts_with) };
                    }
                    if let Some(ends_with) = &zod_attrs.ends_with {
                        validation = quote! { #validation.ends_with(#ends_with) };
                    }
                    if let Some(includes) = &zod_attrs.includes {
                        validation = quote! { #validation.includes(#includes) };
                    }

                    validation
                }
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize"
                | "f32" | "f64" => {
                    let mut validation = quote! { zod_rs::number() };

                    if zod_attrs.int
                        || matches!(
                            type_name.as_str(),
                            "i8" | "i16"
                                | "i32"
                                | "i64"
                                | "u8"
                                | "u16"
                                | "u32"
                                | "u64"
                                | "isize"
                                | "usize"
                        )
                    {
                        validation = quote! { #validation.int() };
                    }
                    if let Some(min) = zod_attrs.min {
                        validation = quote! { #validation.min(#min) };
                    }
                    if let Some(max) = zod_attrs.max {
                        validation = quote! { #validation.max(#max) };
                    }
                    if zod_attrs.positive {
                        validation = quote! { #validation.positive() };
                    }
                    if zod_attrs.negative {
                        validation = quote! { #validation.negative() };
                    }
                    if zod_attrs.nonnegative {
                        validation = quote! { #validation.nonnegative() };
                    }
                    if zod_attrs.nonpositive {
                        validation = quote! { #validation.nonpositive() };
                    }
                    if zod_attrs.finite {
                        validation = quote! { #validation.finite() };
                    }

                    validation
                }
                "bool" => {
                    quote! { zod_rs::boolean() }
                }
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                            let inner_validation = generate_element_validation(inner_type);
                            let mut validation = quote! { zod_rs::array(#inner_validation) };

                            if let Some(min) = zod_attrs.min_length {
                                validation = quote! { #validation.min(#min) };
                            }
                            if let Some(max) = zod_attrs.max_length {
                                validation = quote! { #validation.max(#max) };
                            }
                            if let Some(length) = zod_attrs.length {
                                validation = quote! { #validation.length(#length) };
                            }

                            validation
                        } else {
                            quote! { zod_rs::array(zod_rs::string()) }
                        }
                    } else {
                        quote! { zod_rs::array(zod_rs::string()) }
                    }
                }
                _ => {
                    let type_ident = &segment.ident;
                    quote! { #type_ident::schema() }
                }
            }
        } else {
            quote! { zod_rs::string() }
        }
    } else {
        quote! { zod_rs::string() }
    }
}

fn generate_element_validation(field_type: &syn::Type) -> proc_macro2::TokenStream {
    if let syn::Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();

            match type_name.as_str() {
                "String" => quote! { zod_rs::string() },
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                    quote! { zod_rs::number().int() }
                }
                "f32" | "f64" => quote! { zod_rs::number() },
                "bool" => quote! { zod_rs::boolean() },
                _ => {
                    let type_ident = &segment.ident;
                    quote! { #type_ident::schema() }
                }
            }
        } else {
            quote! { zod_rs::string() }
        }
    } else {
        quote! { zod_rs::string() }
    }
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn get_option_inner_type(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return inner_type.clone();
                    }
                }
            }
        }
    }
    syn::parse_quote! { String }
}

#[proc_macro]
pub fn infer_struct(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        compile_error!("infer_struct macro is not yet implemented. Use #[derive(ZodSchema)] instead.");
    };

    TokenStream::from(expanded)
}
