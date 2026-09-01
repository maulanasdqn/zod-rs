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

                        pub fn validate_and_parse(value: &serde_json::Value) -> Result<Self, ::zod_rs::__private::ValidationResult> {
                            match Self::schema().validate(value) {
                                Ok(_) => {
                                    serde_json::from_value(value.clone())
                                        .map_err(|e| ::zod_rs::__private::ValidationError::custom(format!("Deserialization failed: {}", e)).into())
                                }
                                Err(validation_result) => Err(validation_result)
                            }
                        }

                        pub fn from_json(json_str: &str) -> Result<Self, ::zod_rs::__private::ParseError> {
                            let value: serde_json::Value = serde_json::from_str(json_str)?;
                            Ok(Self::validate_and_parse(&value)?)
                        }

                        pub fn validate_json(json_str: &str) -> Result<serde_json::Value, ::zod_rs::__private::ParseError> {
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
        Data::Enum(data_enum) => generate_enum_schema(name, &input.attrs, data_enum),
        Data::Union(_) => {
            let error = syn::Error::new_spanned(&input, "ZodSchema cannot be derived for unions");
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

fn generate_enum_schema(
    name: &syn::Ident,
    attrs: &[Attribute],
    data_enum: &syn::DataEnum,
) -> TokenStream {
    let rename_all = parse_serde_name(attrs, "rename_all");

    let variant_schemas = data_enum.variants.iter().map(|variant| {
        let variant_name_str =
            effective_variant_name(&variant.ident, &variant.attrs, rename_all.as_deref());

        generate_variant_schema(&variant_name_str, &variant.fields)
    });

    let expanded = quote! {
        impl #name {
            pub fn schema() -> impl zod_rs::Schema<serde_json::Value> {
                zod_rs::union()
                    #(#variant_schemas)*
            }

            pub fn validate_and_parse(value: &serde_json::Value) -> Result<Self, ::zod_rs::__private::ValidationResult> {
                match Self::schema().validate(value) {
                    Ok(_) => {
                        serde_json::from_value(value.clone())
                            .map_err(|e| ::zod_rs::__private::ValidationError::custom(format!("Deserialization failed: {}", e)).into())
                    }
                    Err(validation_result) => Err(validation_result)
                }
            }

            pub fn from_json(json_str: &str) -> Result<Self, ::zod_rs::__private::ParseError> {
                let value: serde_json::Value = serde_json::from_str(json_str)?;
                Ok(Self::validate_and_parse(&value)?)
            }

            pub fn validate_json(json_str: &str) -> Result<serde_json::Value, ::zod_rs::__private::ParseError> {
                let value: serde_json::Value = serde_json::from_str(json_str)?;
                Self::schema().validate(&value)?;
                Ok(value)
            }
        }
    };

    TokenStream::from(expanded)
}

fn effective_variant_name(
    ident: &syn::Ident,
    attrs: &[Attribute],
    rename_all: Option<&str>,
) -> String {
    if let Some(renamed) = parse_serde_name(attrs, "rename") {
        return renamed;
    }
    let name = ident.to_string();
    match rename_all {
        Some(rule) => apply_rename_rule(rule, &name),
        None => name,
    }
}

fn parse_serde_name(attrs: &[Attribute], key: &str) -> Option<String> {
    let mut result = None;

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(key) {
                if meta.input.peek(syn::Token![=]) {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    result = Some(lit.value());
                } else if meta.input.peek(syn::token::Paren) {
                    let mut serialize = None;
                    let mut deserialize = None;
                    meta.parse_nested_meta(|inner| {
                        let lit: syn::LitStr = inner.value()?.parse()?;
                        if inner.path.is_ident("deserialize") {
                            deserialize = Some(lit.value());
                        } else if inner.path.is_ident("serialize") {
                            serialize = Some(lit.value());
                        }
                        Ok(())
                    })?;
                    if let Some(name) = deserialize.or(serialize) {
                        result = Some(name);
                    }
                }
            } else {
                skip_serde_meta_value(&meta)?;
            }
            Ok(())
        });
    }

    result
}

fn skip_serde_meta_value(meta: &syn::meta::ParseNestedMeta) -> syn::Result<()> {
    if meta.input.peek(syn::Token![=]) {
        let _: syn::Expr = meta.value()?.parse()?;
    } else if meta.input.peek(syn::token::Paren) {
        meta.parse_nested_meta(|inner| skip_serde_meta_value(&inner))?;
    }
    Ok(())
}

fn apply_rename_rule(rule: &str, variant: &str) -> String {
    match rule {
        "lowercase" => variant.to_ascii_lowercase(),
        "UPPERCASE" => variant.to_ascii_uppercase(),
        "camelCase" => {
            let mut chars = variant.chars();
            match chars.next() {
                Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        }
        "snake_case" => {
            let mut snake = String::new();
            for (i, ch) in variant.char_indices() {
                if i > 0 && ch.is_uppercase() {
                    snake.push('_');
                }
                snake.push(ch.to_ascii_lowercase());
            }
            snake
        }
        "SCREAMING_SNAKE_CASE" => apply_rename_rule("snake_case", variant).to_ascii_uppercase(),
        "kebab-case" => apply_rename_rule("snake_case", variant).replace('_', "-"),
        "SCREAMING-KEBAB-CASE" => {
            apply_rename_rule("SCREAMING_SNAKE_CASE", variant).replace('_', "-")
        }
        _ => variant.to_string(),
    }
}

fn generate_variant_schema(variant_name: &str, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => {
            quote! {
                .variant(zod_rs::literal_value(#variant_name))
                .variant(
                    zod_rs::object()
                        .field(#variant_name, zod_rs::null())
                )
            }
        }

        // Tuple variant (unnamed fields)
        Fields::Unnamed(fields_unnamed) => {
            generate_tuple_variant_schema(variant_name, fields_unnamed)
        }

        // Struct variant (named fields): {"VariantName": {"field1": ..., "field2": ...}}
        Fields::Named(fields_named) => generate_struct_variant_schema(variant_name, fields_named),
    }
}

fn generate_tuple_variant_schema(
    variant_name: &str,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let field_count = fields.unnamed.len();

    if field_count == 1 {
        // Single element: {"VariantName": value}
        let field = fields.unnamed.first().unwrap();
        let field_type = &field.ty;
        let field_attrs = &field.attrs;
        let inner_validation =
            generate_base_validation_with_attrs(field_type, &parse_zod_attributes(field_attrs));

        quote! {
            .variant(
                zod_rs::object()
                    .field(#variant_name, #inner_validation)
            )
        }
    } else {
        // Multiple elements: {"VariantName": [value1, value2, ...]}
        let element_validations = fields.unnamed.iter().map(|field| {
            let field_type = &field.ty;
            let field_attrs = &field.attrs;
            generate_base_validation_with_attrs(field_type, &parse_zod_attributes(field_attrs))
        });

        quote! {
            .variant(
                zod_rs::object()
                    .field(#variant_name, zod_rs::tuple()
                        #(.element(#element_validations))*
                    )
            )
        }
    }
}

fn generate_struct_variant_schema(
    variant_name: &str,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let field_validations = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        let field_type = &field.ty;
        let field_attrs = &field.attrs;

        generate_field_validation_with_attrs(&field_name_str, field_type, field_attrs)
    });

    quote! {
        .variant(
            zod_rs::object()
                .field(#variant_name, zod_rs::object()
                    #(#field_validations)*
                )
        )
    }
}

#[proc_macro]
pub fn infer_struct(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        compile_error!("infer_struct macro is not yet implemented. Use #[derive(ZodSchema)] instead.");
    };

    TokenStream::from(expanded)
}
