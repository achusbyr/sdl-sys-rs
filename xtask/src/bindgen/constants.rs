use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Collect and sort all `.h` header file paths from a directory.
fn sorted_header_paths(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read header directory {}: {e}", dir.display()));

    let mut paths: Vec<_> = entries
        .map(|r| {
            r.unwrap_or_else(|e| panic!("Failed to read directory entry in {}: {e}", dir.display()))
                .path()
        })
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("h"))
        .collect();
    paths.sort();
    paths
}

/// Scans SDL headers for `#define` macros that represent "Hints" or "Properties".
/// These are typically string-based constants used with `SDL_SetHint` or property bags.
/// Since these are just strings, we treat them as target-agnostic and extract documentation
/// from the headers.
pub fn extract_and_generate(
    include_dir: &Path,
    out_file: &Path,
    lib_name: &str,
    hint_prefix: &str,
    prop_prefix: &str,
) {
    let subsystem_dir = include_dir.join(lib_name);

    println!("Extracting constants from {:?}...", subsystem_dir);

    let mut hints = Vec::new();
    let mut props = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let re_hint = Regex::new(&format!(
        r#"^#define\s+({}[A-Za-z0-9_]+)\s+"([^"]+)"(?:\s*/\*.*\*/|\s*//.*)?"#,
        hint_prefix
    ))
    .unwrap();
    let re_prop = Regex::new(&format!(
        r#"^#define\s+({}[A-Za-z0-9_]+)\s+"([^"]+)"(?:\s*/\*.*\*/|\s*//.*)?"#,
        prop_prefix
    ))
    .unwrap();

    let paths = sorted_header_paths(&subsystem_dir);

    for path in paths {
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
        let mut current_doc = String::new();
        let mut in_doc = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("/**") {
                in_doc = true;
                current_doc.clear();
            }

            if in_doc {
                current_doc.push_str(trimmed);
                current_doc.push('\n');
                if trimmed.ends_with("*/") {
                    in_doc = false;
                }
            } else if let Some(caps) = re_hint.captures(trimmed) {
                let name = caps.get(1).unwrap().as_str().to_string();
                if seen.insert(name.clone()) {
                    hints.push((
                        name,
                        caps.get(2).unwrap().as_str().to_string(),
                        current_doc.clone(),
                    ));
                }
                current_doc.clear();
            } else if let Some(caps) = re_prop.captures(trimmed) {
                let name = caps.get(1).unwrap().as_str().to_string();
                if seen.insert(name.clone()) {
                    props.push((
                        name,
                        caps.get(2).unwrap().as_str().to_string(),
                        current_doc.clone(),
                    ));
                }
                current_doc.clear();
            } else if !trimmed.is_empty() {
                current_doc.clear();
            }
        }
    }

    let mut out = fs::File::create(out_file)
        .unwrap_or_else(|e| panic!("Failed to create {}: {e}", out_file.display()));
    writeln!(out, "//! Generated constants\n").unwrap();
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").unwrap();
    writeln!(
        out,
        "pub enum PropertyType {{ Pointer, String, Number, Float, Boolean }}"
    )
    .unwrap();
    writeln!(out, "\n#[derive(Debug, Clone, Copy)]").unwrap();
    writeln!(out, "pub struct Hint {{\n    pub name: &'static str,\n    pub value: &'static str,\n    pub doc: &'static str,\n}}").unwrap();
    writeln!(out, "\n#[derive(Debug, Clone, Copy)]").unwrap();
    writeln!(out, "pub struct Property {{\n    pub name: &'static str,\n    pub value: &'static str,\n    pub ty: PropertyType,\n    pub doc: &'static str,\n}}\n").unwrap();

    for (n, v, d) in &hints {
        writeln!(
            out,
            "{d}pub const {n}: Hint = Hint {{ name: \"{n}\", value: \"{v}\", doc: {d:?} }};"
        )
        .unwrap();
    }

    // Infer the property type based on the suffix of the constant name.
    for (n, v, d) in &props {
        let ty = if n.ends_with("_POINTER") {
            "PropertyType::Pointer"
        } else if n.ends_with("_STRING") {
            "PropertyType::String"
        } else if n.ends_with("_NUMBER") {
            "PropertyType::Number"
        } else if n.ends_with("_FLOAT") {
            "PropertyType::Float"
        } else if n.ends_with("_BOOLEAN") {
            "PropertyType::Boolean"
        } else {
            "PropertyType::Pointer"
        };
        writeln!(
            out,
            "{d}pub const {n}: Property = Property {{ name: \"{n}\", value: \"{v}\", ty: {ty}, doc: {d:?} }};"
        )
        .unwrap();
    }
}

pub fn append_macro_constants(include_dir: &Path, out_file: &Path, lib_name: &str) {
    let subsystem_dir = include_dir.join(lib_name);
    let paths = sorted_header_paths(&subsystem_dir);

    // Regex to match #define NAME VALUE
    // We target names starting with SDL_ and values that don't look like function macros.
    let re_define = Regex::new(r#"^#define\s+(SDL_[A-Z][A-Za-z0-9_]+)\s+(.+)$"#).unwrap();

    // Specific patterns within the value
    let re_sdl_c_macro = Regex::new(r#"SDL_(U?)INT(64|32|16|8)_C\(([^)]+)\)"#).unwrap();
    let re_cast = Regex::new(r#"\(([A-Za-z0-9_]+)\)\s*(.+)$"#).unwrap();
    let re_bit_shift = Regex::new(r#"\(\s*(1[uU]?)\s*<<\s*([0-9]+)\s*\)"#).unwrap();
    let re_numeric = Regex::new(r#"^-?([0-9]+|0x[0-9A-Fa-f]+)[uU]?[lL]{0,2}$"#).unwrap();

    let mut constants = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for path in paths {
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        if file_name == "SDL_begin_code.h"
            || file_name == "SDL_close_code.h"
            || file_name == "SDL_assert.h"
            || file_name == "SDL_oldnames.h"
        {
            continue;
        }

        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));

        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("#define") {
                continue;
            }

            // Remove trailing comments from the line
            let line_without_comment = trimmed
                .split("//")
                .next()
                .unwrap()
                .split("/*")
                .next()
                .unwrap()
                .trim();

            if let Some(caps) = re_define.captures(line_without_comment) {
                let name = caps.get(1).unwrap().as_str();
                let mut raw_val = caps.get(2).unwrap().as_str().trim();

                if seen_names.contains(name) {
                    continue;
                }

                // Skip if it looks like a function macro
                if raw_val.contains('\\') || (raw_val.starts_with('(') && !raw_val.ends_with(')')) {
                    continue;
                }

                // Strip balanced outer parentheses
                while raw_val.starts_with('(') && raw_val.ends_with(')') {
                    let inner = &raw_val[1..raw_val.len() - 1].trim();
                    let mut depth = 0;
                    let mut balanced = true;
                    for c in inner.chars() {
                        if c == '(' {
                            depth += 1;
                        } else if c == ')' {
                            depth -= 1;
                            if depth < 0 {
                                balanced = false;
                                break;
                            }
                        }
                    }
                    if balanced && depth == 0 {
                        raw_val = inner;
                    } else {
                        break;
                    }
                }

                let mut extracted = None;

                // Try to match various patterns
                if let Some(c) = re_sdl_c_macro.captures(raw_val) {
                    let is_unsigned = c.get(1).unwrap().as_str() == "U";
                    let bits = c.get(2).unwrap().as_str();
                    let val = c
                        .get(3)
                        .unwrap()
                        .as_str()
                        .trim()
                        .trim_end_matches(['U', 'u', 'L', 'l']);
                    let ty = format!("{}{}", if is_unsigned { "u" } else { "i" }, bits);
                    extracted = Some((ty, val.to_string()));
                } else if let Some(c) = re_cast.captures(raw_val) {
                    let c_type = c.get(1).unwrap().as_str();
                    let mut inner_val = c
                        .get(2)
                        .unwrap()
                        .as_str()
                        .trim()
                        .trim_end_matches(['U', 'u', 'L', 'l'])
                        .to_string();

                    // Final strip and translation
                    if inner_val.starts_with('(') && inner_val.ends_with(')') {
                        inner_val = inner_val[1..inner_val.len() - 1].trim().to_string();
                    }
                    inner_val = inner_val.replace('~', "!");

                    let ty = map_c_type_to_rust(c_type);
                    if inner_val.starts_with('-') && ty.starts_with('u') {
                        // Use parentheses and i32 suffix to ensure negative literal is valid before casting.
                        extracted = Some((ty.to_string(), format!("({}i32) as {}", inner_val, ty)));
                    } else {
                        extracted = Some((ty.to_string(), inner_val));
                    }
                } else if let Some(c) = re_bit_shift.captures(raw_val) {
                    let val = c.get(1).unwrap().as_str().trim_end_matches(['U', 'u']);
                    let shift = c.get(2).unwrap().as_str();
                    extracted = Some(("u32".to_string(), format!("{} << {}", val, shift)));
                } else if re_numeric.is_match(raw_val) {
                    let val = raw_val.trim_end_matches(['U', 'u', 'L', 'l']);
                    let ty = if val.starts_with("0x") && val.len() > 10 {
                        "u64"
                    } else {
                        "i32"
                    };
                    if val.starts_with('-') {
                        extracted = Some(("i32".to_string(), val.to_string()));
                    } else {
                        extracted = Some((ty.to_string(), val.to_string()));
                    }
                }

                if let Some((ty, mut val)) = extracted {
                    val = val.trim().to_string();
                    // Final sanity check: if it contains weird things like '(', ')', '[', ']', '{', '}', ';' it's probably not a simple constant we want
                    if val.is_empty() || val.contains(['[', ']', '{', '}', ';', ',']) {
                        continue;
                    }
                    constants.push(format!("pub const {}: {} = {};", name, ty, val));
                    seen_names.insert(name.to_string());
                }
            }
        }
    }

    // Extract constants that weren't already seen/extracted
    let mut final_constants = Vec::new();
    if out_file.exists() {
        if let Ok(content) = fs::read_to_string(out_file) {
            for c_line in constants {
                // Extract the name from "pub const NAME: ..."
                if let Some(name) = c_line.split_whitespace().nth(2) {
                    let name_trimmed = name.trim_end_matches(':');
                    if !content.contains(&format!("pub const {}", name_trimmed)) {
                        final_constants.push(c_line);
                    }
                }
            }
        } else {
            final_constants = constants;
        }
    } else {
        final_constants = constants;
    }

    if !final_constants.is_empty() {
        let mut out = fs::OpenOptions::new()
            .append(true)
            .open(out_file)
            .unwrap_or_else(|e| panic!("Failed to open {}: {e}", out_file.display()));
        writeln!(out, "\n// Extracted macro constants").unwrap();
        for c in final_constants {
            writeln!(out, "{}", c).unwrap();
        }
    }
}

fn map_c_type_to_rust(c_type: &str) -> &str {
    match c_type {
        "Uint8" => "u8",
        "Sint8" => "i8",
        "Uint16" => "u16",
        "Sint16" => "i16",
        "Uint32" => "u32",
        "Sint32" => "i32",
        "Uint64" => "u64",
        "Sint64" => "i64",
        "SDL_AudioDeviceID" => "u32",
        "SDL_WindowFlags" => "u64",
        "size_t" => "usize",
        _ => "u32", // Default for many SDL ID types
    }
}
