use clap::{Parser, Subcommand};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "zod-rs-ts")]
#[command(about = "Generate TypeScript Zod schemas from Rust structs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate TypeScript Zod schemas from Rust source files
    Generate {
        /// Input directory containing Rust source files
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory or file for TypeScript schemas
        #[arg(short, long)]
        output: PathBuf,

        /// Generate all schemas in a single file
        #[arg(long)]
        single_file: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            output,
            single_file,
        } => {
            if let Err(e) = generate_schemas(&input, &output, single_file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn generate_schemas(
    input: &PathBuf,
    output: &PathBuf,
    single_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_schemas = Vec::new();

    // Find all .rs files
    for entry in WalkDir::new(input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let content = fs::read_to_string(entry.path())?;
        let schemas = extract_schemas(&content);

        if !schemas.is_empty() {
            if single_file {
                all_schemas.extend(schemas);
            } else {
                // Write individual files
                for (name, schema) in schemas {
                    let file_name = format!("{}.ts", to_snake_case(&name));
                    let file_path = output.join(&file_name);
                    fs::create_dir_all(output)?;
                    fs::write(&file_path, schema)?;
                    println!("Generated: {}", file_path.display());
                }
            }
        }
    }

    if single_file && !all_schemas.is_empty() {
        fs::create_dir_all(output.parent().unwrap_or(output))?;

        let combined = format!(
            "import {{ z }} from 'zod';\n\n{}",
            all_schemas
                .iter()
                .map(|(_, schema)| {
                    // Remove the import line from individual schemas
                    schema
                        .lines()
                        .filter(|line| !line.starts_with("import"))
                        .collect::<Vec<_>>()
                        .join("\n")
                        .trim()
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        );

        fs::write(output, combined)?;
        println!("Generated: {}", output.display());
    }

    Ok(())
}

fn extract_schemas(content: &str) -> Vec<(String, String)> {
    let mut schemas = Vec::new();

    // Regex to find structs/enums with ZodTs derive
    let derive_re = Regex::new(r#"#\[derive\([^)]*ZodTs[^)]*\)\]"#).unwrap();
    let struct_re = Regex::new(r#"struct\s+(\w+)"#).unwrap();
    let enum_re = Regex::new(r#"enum\s+(\w+)"#).unwrap();

    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if derive_re.is_match(line) {
            // Look ahead for struct or enum definition
            for j in i + 1..std::cmp::min(i + 5, lines.len()) {
                if let Some(caps) = struct_re.captures(lines[j]) {
                    let name = caps.get(1).unwrap().as_str().to_string();
                    if let Some(schema) = generate_struct_schema(&lines, j, &name) {
                        schemas.push((name, schema));
                    }
                    break;
                } else if let Some(caps) = enum_re.captures(lines[j]) {
                    let name = caps.get(1).unwrap().as_str().to_string();
                    if let Some(schema) = generate_enum_schema(&lines, j, &name) {
                        schemas.push((name, schema));
                    }
                    break;
                }
            }
        }
    }

    schemas
}

fn generate_struct_schema(lines: &[&str], start: usize, name: &str) -> Option<String> {
    let mut fields = Vec::new();
    let mut in_struct = false;
    let mut brace_count = 0;
    let mut current_attrs: Vec<String> = Vec::new();

    for line in lines.iter().skip(start) {
        let trimmed = line.trim();

        if trimmed.contains('{') {
            in_struct = true;
            brace_count += trimmed.matches('{').count();
            brace_count -= trimmed.matches('}').count();
            continue;
        }

        if in_struct {
            brace_count += trimmed.matches('{').count();
            brace_count -= trimmed.matches('}').count();

            if brace_count == 0 {
                break;
            }

            if trimmed.starts_with("#[zod(") {
                current_attrs.push(trimmed.to_string());
            } else if trimmed.contains(':') && !trimmed.starts_with("//") {
                // Parse field
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() >= 2 {
                    let field_name = parts[0].trim().trim_start_matches("pub ");
                    let field_type = parts[1]
                        .trim()
                        .trim_end_matches(',')
                        .trim();

                    let zod_type = rust_type_to_zod_simple(field_type, &current_attrs);
                    fields.push(format!("  {}: {}", field_name, zod_type));
                }
                current_attrs.clear();
            }
        }
    }

    if fields.is_empty() {
        return None;
    }

    let schema_name = format!("{}Schema", name);
    Some(format!(
        r#"import {{ z }} from 'zod';

export const {} = z.object({{
{}
}});

export type {} = z.infer<typeof {}>;"#,
        schema_name,
        fields.join(",\n"),
        name,
        schema_name
    ))
}

fn generate_enum_schema(lines: &[&str], start: usize, name: &str) -> Option<String> {
    let mut variants = Vec::new();
    let mut in_enum = false;
    let mut brace_count = 0;

    for line in lines.iter().skip(start) {
        let trimmed = line.trim();

        if trimmed.contains('{') && !in_enum {
            in_enum = true;
            brace_count += trimmed.matches('{').count();
            brace_count -= trimmed.matches('}').count();
            continue;
        }

        if in_enum {
            brace_count += trimmed.matches('{').count();
            brace_count -= trimmed.matches('}').count();

            if brace_count == 0 {
                break;
            }

            if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
                // Parse variant
                let variant_name = trimmed
                    .split(|c| c == '(' || c == '{' || c == ',')
                    .next()
                    .unwrap_or("")
                    .trim();

                if !variant_name.is_empty() {
                    if trimmed.contains('(') {
                        // Tuple variant
                        if let Some(inner) = trimmed
                            .split('(')
                            .nth(1)
                            .and_then(|s| s.split(')').next())
                        {
                            let types: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
                            if types.len() == 1 {
                                let zod_type =
                                    rust_type_to_zod_simple(types[0], &[]);
                                variants.push(format!(
                                    "z.object({{ {}: {} }})",
                                    variant_name, zod_type
                                ));
                            } else {
                                let tuple_types: Vec<String> = types
                                    .iter()
                                    .map(|t| rust_type_to_zod_simple(t, &[]))
                                    .collect();
                                variants.push(format!(
                                    "z.object({{ {}: z.tuple([{}]) }})",
                                    variant_name,
                                    tuple_types.join(", ")
                                ));
                            }
                        }
                    } else if trimmed.contains('{') {
                        // Struct variant - simplified handling
                        variants.push(format!(
                            "z.object({{ {}: z.object({{  }}) }})",
                            variant_name
                        ));
                    } else {
                        // Unit variant
                        variants.push(format!("z.object({{ {}: z.null() }})", variant_name));
                    }
                }
            }
        }
    }

    if variants.is_empty() {
        return None;
    }

    let schema_name = format!("{}Schema", name);
    Some(format!(
        r#"import {{ z }} from 'zod';

export const {} = z.union([
  {}
]);

export type {} = z.infer<typeof {}>;"#,
        schema_name,
        variants.join(",\n  "),
        name,
        schema_name
    ))
}

fn rust_type_to_zod_simple(rust_type: &str, attrs: &[String]) -> String {
    let rust_type = rust_type.trim();
    let is_optional = rust_type.starts_with("Option<");

    let inner_type = if is_optional {
        rust_type
            .strip_prefix("Option<")
            .and_then(|s| s.strip_suffix('>'))
            .unwrap_or(rust_type)
    } else {
        rust_type
    };

    let mut base = match inner_type {
        "String" | "&str" | "str" => "z.string()".to_string(),
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => "z.number().int()".to_string(),
        "f32" | "f64" => "z.number()".to_string(),
        "bool" => "z.boolean()".to_string(),
        other => {
            if other.starts_with("Vec<") {
                let element_type = other
                    .strip_prefix("Vec<")
                    .and_then(|s| s.strip_suffix('>'))
                    .unwrap_or("unknown");
                let inner_zod = rust_type_to_zod_simple(element_type, &[]);
                format!("z.array({})", inner_zod)
            } else {
                format!("{}Schema", other)
            }
        }
    };

    // Apply attributes
    for attr in attrs {
        if attr.contains("email") {
            base.push_str(".email()");
        }
        if attr.contains("url") {
            base.push_str(".url()");
        }
        if let Some(min) = extract_attr_value(attr, "min_length") {
            base.push_str(&format!(".min({})", min));
        }
        if let Some(max) = extract_attr_value(attr, "max_length") {
            base.push_str(&format!(".max({})", max));
        }
        if let Some(min) = extract_attr_value(attr, "min") {
            base.push_str(&format!(".min({})", min));
        }
        if let Some(max) = extract_attr_value(attr, "max") {
            base.push_str(&format!(".max({})", max));
        }
        if attr.contains("positive") {
            base.push_str(".positive()");
        }
        if attr.contains("negative") {
            base.push_str(".negative()");
        }
        if attr.contains("nonnegative") {
            base.push_str(".nonnegative()");
        }
        if attr.contains("nonpositive") {
            base.push_str(".nonpositive()");
        }
        if attr.contains("int") && !base.contains(".int()") {
            base.push_str(".int()");
        }
    }

    if is_optional {
        format!("{}.optional()", base)
    } else {
        base
    }
}

fn extract_attr_value(attr: &str, name: &str) -> Option<String> {
    let pattern = format!(r"{}[\s]*\(([^)]+)\)", name);
    let re = Regex::new(&pattern).ok()?;
    re.captures(attr)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
