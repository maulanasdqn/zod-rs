use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta};

#[proc_macro_derive(ZodTs, attributes(zod))]
pub fn derive_zod_ts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();

    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_schemas: Vec<String> = fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        let field_type = &field.ty;
                        let attrs = parse_zod_attributes(&field.attrs);
                        let is_optional = is_option_type(field_type);

                        let base_type = if is_optional {
                            get_option_inner_type_str(field_type)
                        } else {
                            type_to_string(field_type)
                        };

                        let zod_type = rust_type_to_zod(&base_type, &attrs);
                        let final_type = if is_optional {
                            format!("{}.optional()", zod_type)
                        } else {
                            zod_type
                        };

                        format!("  {}: {}", field_name, final_type)
                    })
                    .collect();

                let fields_str = field_schemas.join(",\n");
                let schema_name = format!("{}Schema", name_str);

                let ts_code = format!(
                    r#"import {{ z }} from 'zod';

export const {} = z.object({{
{}
}});

export type {} = z.infer<typeof {}>;"#,
                    schema_name, fields_str, name_str, schema_name
                );

                let expanded = quote! {
                    impl #name {
                        pub fn zod_ts() -> String {
                            #ts_code.to_string()
                        }
                    }
                };

                TokenStream::from(expanded)
            }
            _ => {
                let error = syn::Error::new_spanned(
                    &input,
                    "ZodTs can only be derived for structs with named fields",
                );
                TokenStream::from(error.to_compile_error())
            }
        },
        Data::Enum(data_enum) => {
            let variant_schemas: Vec<String> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = variant.ident.to_string();
                    generate_variant_ts(&variant_name, &variant.fields)
                })
                .collect();

            let variants_str = variant_schemas.join(",\n  ");
            let schema_name = format!("{}Schema", name_str);

            let ts_code = format!(
                r#"import {{ z }} from 'zod';

export const {} = z.union([
  {}
]);

export type {} = z.infer<typeof {}>;"#,
                schema_name, variants_str, name_str, schema_name
            );

            let expanded = quote! {
                impl #name {
                    pub fn zod_ts() -> String {
                        #ts_code.to_string()
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Data::Union(_) => {
            let error =
                syn::Error::new_spanned(&input, "ZodTs cannot be derived for Rust unions");
            TokenStream::from(error.to_compile_error())
        }
    }
}

fn generate_variant_ts(variant_name: &str, fields: &Fields) -> String {
    match fields {
        Fields::Unit => {
            format!("z.object({{ {}: z.null() }})", variant_name)
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_count = fields_unnamed.unnamed.len();
            if field_count == 1 {
                let field = fields_unnamed.unnamed.first().unwrap();
                let field_type = type_to_string(&field.ty);
                let attrs = parse_zod_attributes(&field.attrs);
                let zod_type = rust_type_to_zod(&field_type, &attrs);
                format!("z.object({{ {}: {} }})", variant_name, zod_type)
            } else {
                let element_types: Vec<String> = fields_unnamed
                    .unnamed
                    .iter()
                    .map(|field| {
                        let field_type = type_to_string(&field.ty);
                        let attrs = parse_zod_attributes(&field.attrs);
                        rust_type_to_zod(&field_type, &attrs)
                    })
                    .collect();
                let tuple_str = element_types.join(", ");
                format!("z.object({{ {}: z.tuple([{}]) }})", variant_name, tuple_str)
            }
        }
        Fields::Named(fields_named) => {
            let field_schemas: Vec<String> = fields_named
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_type = type_to_string(&field.ty);
                    let attrs = parse_zod_attributes(&field.attrs);
                    let is_optional = is_option_type(&field.ty);

                    let base_type = if is_optional {
                        get_option_inner_type_str(&field.ty)
                    } else {
                        field_type
                    };

                    let zod_type = rust_type_to_zod(&base_type, &attrs);
                    let final_type = if is_optional {
                        format!("{}.optional()", zod_type)
                    } else {
                        zod_type
                    };

                    format!("{}: {}", field_name, final_type)
                })
                .collect();
            let fields_str = field_schemas.join(", ");
            format!(
                "z.object({{ {}: z.object({{ {} }}) }})",
                variant_name, fields_str
            )
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
                                i += 1;
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
                        "," => {}
                        _ => {}
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

fn strip_quotes(value: &str) -> String {
    if let Some(inner) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    if let Some(inner) = value.strip_prefix("r\"").and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    value.to_string()
}

fn rust_type_to_zod(rust_type: &str, attrs: &ZodAttributes) -> String {
    let base = match rust_type {
        "String" | "&str" | "str" => {
            let mut chain = String::from("z.string()");

            if let Some(len) = attrs.length {
                chain.push_str(&format!(".length({})", len));
            }
            if let Some(min) = attrs.min_length {
                chain.push_str(&format!(".min({})", min));
            }
            if let Some(max) = attrs.max_length {
                chain.push_str(&format!(".max({})", max));
            }
            if attrs.email {
                chain.push_str(".email()");
            }
            if attrs.url {
                chain.push_str(".url()");
            }
            if let Some(ref pattern) = attrs.regex {
                chain.push_str(&format!(".regex(/{}/)", pattern));
            }
            if let Some(ref prefix) = attrs.starts_with {
                chain.push_str(&format!(".startsWith(\"{}\")", prefix));
            }
            if let Some(ref suffix) = attrs.ends_with {
                chain.push_str(&format!(".endsWith(\"{}\")", suffix));
            }
            if let Some(ref substr) = attrs.includes {
                chain.push_str(&format!(".includes(\"{}\")", substr));
            }

            chain
        }
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => {
            let mut chain = String::from("z.number().int()");
            append_number_validators(&mut chain, attrs);
            chain
        }
        "f32" | "f64" => {
            let mut chain = String::from("z.number()");
            if attrs.int {
                chain.push_str(".int()");
            }
            append_number_validators(&mut chain, attrs);
            chain
        }
        "bool" => String::from("z.boolean()"),
        other => {
            if other.starts_with("Vec<") {
                let inner = other
                    .strip_prefix("Vec<")
                    .and_then(|s| s.strip_suffix('>'))
                    .unwrap_or("unknown");
                let inner_zod = rust_type_to_zod(inner, &ZodAttributes::default());
                let mut chain = format!("z.array({})", inner_zod);

                if let Some(len) = attrs.length {
                    chain.push_str(&format!(".length({})", len));
                }
                if let Some(min) = attrs.min_length {
                    chain.push_str(&format!(".min({})", min));
                }
                if let Some(max) = attrs.max_length {
                    chain.push_str(&format!(".max({})", max));
                }

                chain
            } else {
                format!("{}Schema", other)
            }
        }
    };

    base
}

fn append_number_validators(chain: &mut String, attrs: &ZodAttributes) {
    if let Some(min) = attrs.min {
        chain.push_str(&format!(".min({})", min));
    }
    if let Some(max) = attrs.max {
        chain.push_str(&format!(".max({})", max));
    }
    if attrs.positive {
        chain.push_str(".positive()");
    }
    if attrs.negative {
        chain.push_str(".negative()");
    }
    if attrs.nonnegative {
        chain.push_str(".nonnegative()");
    }
    if attrs.nonpositive {
        chain.push_str(".nonpositive()");
    }
    if attrs.finite {
        chain.push_str(".finite()");
    }
}

fn type_to_string(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        let segments: Vec<String> = type_path
            .path
            .segments
            .iter()
            .map(|seg| {
                let ident = seg.ident.to_string();
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    let args_str: Vec<String> = args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let syn::GenericArgument::Type(t) = arg {
                                Some(type_to_string(t))
                            } else {
                                None
                            }
                        })
                        .collect();
                    if args_str.is_empty() {
                        ident
                    } else {
                        format!("{}<{}>", ident, args_str.join(", "))
                    }
                } else {
                    ident
                }
            })
            .collect();
        segments.join("::")
    } else {
        "unknown".to_string()
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

fn get_option_inner_type_str(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return type_to_string(inner_type);
                    }
                }
            }
        }
    }
    "unknown".to_string()
}
