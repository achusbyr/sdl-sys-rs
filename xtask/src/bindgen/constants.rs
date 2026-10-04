use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

type Result<T> = std::result::Result<T, String>;

static RE_DEFINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#define\s+(SDL_[A-Z][A-Za-z0-9_]+)\s+(.+)$").unwrap());
static RE_SDL_C_MACRO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(~?)SDL_([US])INT(8|16|32|64)_C\(([^)]+)\)$").unwrap());
static RE_CAST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\(([A-Za-z_][A-Za-z0-9_]*)\)\s*(.+)$").unwrap());
static RE_BIT_SHIFT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\(?\s*1[uU]?\s*<<\s*([0-9]+)\s*\)?$").unwrap());
static RE_NUMERIC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(-?)(0[xX][0-9A-Fa-f]+|[0-9]+)[uU]?[lL]{0,2}$").unwrap());
static RE_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:0[xX][0-9A-Fa-f]+|[0-9]+)$").unwrap());
static RE_CONST_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"pub const ([A-Za-z0-9_]+)\s*:").unwrap());
static RE_TYPE_ALIAS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"pub type ([A-Za-z0-9_]+)\s*=").unwrap());

/// A string `#define` (hint or property) together with its preceding doc comment.
struct Entry {
    name: String,
    value: String,
    doc: String,
}

fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))
}

/// Collect and sort all `.h` header file paths from a directory.
fn sorted_header_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read header directory {}: {e}", dir.display()))?;

    let mut paths = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| format!("Failed to read directory entry in {}: {e}", dir.display()))?
            .path();
        if path.extension().and_then(|e| e.to_str()) == Some("h") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn define_regex(prefix: &str) -> Result<Regex> {
    Regex::new(&format!(
        r#"^#define\s+({}[A-Za-z0-9_]+)\s+"([^"]+)"(?:\s*/\*.*\*/|\s*//.*)?"#,
        regex::escape(prefix)
    ))
    .map_err(|e| format!("Invalid prefix '{prefix}': {e}"))
}

fn capture_define(re: &Regex, line: &str) -> Option<(String, String)> {
    re.captures(line)
        .map(|caps| (caps[1].to_string(), caps[2].to_string()))
}

/// Infer the property type based on the suffix of the constant name.
fn property_type(name: &str) -> &'static str {
    if name.ends_with("_STRING") {
        "PropertyType::String"
    } else if name.ends_with("_NUMBER") {
        "PropertyType::Number"
    } else if name.ends_with("_FLOAT") {
        "PropertyType::Float"
    } else if name.ends_with("_BOOLEAN") {
        "PropertyType::Boolean"
    } else {
        // `_POINTER` and anything unrecognised.
        "PropertyType::Pointer"
    }
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
) -> Result<()> {
    let subsystem_dir = include_dir.join(lib_name);

    println!("Extracting constants from {:?}...", subsystem_dir);

    let mut hints = Vec::new();
    let mut props = Vec::new();
    let mut seen = HashSet::new();

    let re_hint = define_regex(hint_prefix)?;
    let re_prop = define_regex(prop_prefix)?;

    for path in sorted_header_paths(&subsystem_dir)? {
        let content = read_file(&path)?;
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
            } else if let Some(((name, value), list)) =
                [(&re_hint, &mut hints), (&re_prop, &mut props)]
                    .into_iter()
                    .find_map(|(re, list)| capture_define(re, trimmed).map(|found| (found, list)))
            {
                let doc = std::mem::take(&mut current_doc);
                if seen.insert(name.clone()) {
                    list.push(Entry { name, value, doc });
                }
            } else if !trimmed.is_empty() {
                current_doc.clear();
            }
        }
    }

    let mut out = String::from(
        "//! Generated constants\n\
         \n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum PropertyType { Pointer, String, Number, Float, Boolean }\n\
         \n\
         #[derive(Debug, Clone, Copy)]\n\
         pub struct Hint {\n    pub name: &'static str,\n    pub value: &'static str,\n    pub doc: &'static str,\n}\n\
         \n\
         #[derive(Debug, Clone, Copy)]\n\
         pub struct Property {\n    pub name: &'static str,\n    pub value: &'static str,\n    pub ty: PropertyType,\n    pub doc: &'static str,\n}\n\n",
    );

    for Entry { name, value, doc } in &hints {
        out.push_str(&format!(
            "{doc}pub const {name}: Hint = Hint {{ name: {name:?}, value: {value:?}, doc: {doc:?} }};\n"
        ));
    }
    for Entry { name, value, doc } in &props {
        let ty = property_type(name);
        out.push_str(&format!(
            "{doc}pub const {name}: Property = Property {{ name: {name:?}, value: {value:?}, ty: {ty}, doc: {doc:?} }};\n"
        ));
    }

    fs::write(out_file, out).map_err(|e| format!("Failed to write {}: {e}", out_file.display()))
}

/// Appends numeric `#define` constants that bindgen did not emit to the bindings file.
pub fn append_macro_constants(include_dir: &Path, out_file: &Path, lib_name: &str) -> Result<()> {
    let subsystem_dir = include_dir.join(lib_name);
    let bindings = read_file(out_file)?;
    let mut known: HashSet<String> = RE_CONST_NAME
        .captures_iter(&bindings)
        .map(|caps| caps[1].to_string())
        .collect();
    let aliases: HashSet<String> = RE_TYPE_ALIAS
        .captures_iter(&bindings)
        .map(|caps| caps[1].to_string())
        .collect();

    let mut constants = Vec::new();

    for path in sorted_header_paths(&subsystem_dir)? {
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        if matches!(
            file_name.as_ref(),
            "SDL_begin_code.h" | "SDL_close_code.h" | "SDL_assert.h" | "SDL_oldnames.h"
        ) {
            continue;
        }

        for line in read_file(&path)?.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("#define") {
                continue;
            }

            // Remove trailing comments from the line
            let without_comment = trimmed
                .split("//")
                .next()
                .unwrap_or_default()
                .split("/*")
                .next()
                .unwrap_or_default()
                .trim();

            let Some(caps) = RE_DEFINE.captures(without_comment) else {
                continue;
            };
            let name = &caps[1];
            if known.contains(name) {
                continue;
            }
            if let Some((ty, value)) = convert_value(&caps[2], &aliases) {
                constants.push(format!("pub const {name}: {ty} = {value};"));
                known.insert(name.to_string());
            }
        }
    }

    if constants.is_empty() {
        return Ok(());
    }

    let mut out = bindings;
    out.push_str("\n// Extracted macro constants\n");
    for constant in constants {
        out.push_str(&constant);
        out.push('\n');
    }
    fs::write(out_file, out).map_err(|e| format!("Failed to write {}: {e}", out_file.display()))
}

/// Strips balanced parentheses wrapping the whole expression.
fn strip_outer_parens(mut value: &str) -> &str {
    while value.starts_with('(') && value.ends_with(')') {
        let inner = value[1..value.len() - 1].trim();
        let mut depth = 0i32;
        for c in inner.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            if depth < 0 {
                return value;
            }
        }
        if depth != 0 {
            return value;
        }
        value = inner;
    }
    value
}

/// Validates a C integer literal and strips its `u`/`l` suffixes.
fn parse_literal(literal: &str) -> Option<&str> {
    let literal = literal.trim().trim_end_matches(['U', 'u', 'L', 'l']);
    RE_LITERAL.is_match(literal).then_some(literal)
}

fn literal_value(literal: &str) -> Option<u128> {
    match literal
        .strip_prefix("0x")
        .or_else(|| literal.strip_prefix("0X"))
    {
        Some(hex) => u128::from_str_radix(hex, 16).ok(),
        None => literal.parse().ok(),
    }
}

fn primitive_type(c_type: &str) -> Option<&'static str> {
    Some(match c_type {
        "Uint8" => "u8",
        "Sint8" => "i8",
        "Uint16" => "u16",
        "Sint16" => "i16",
        "Uint32" => "u32",
        "Sint32" => "i32",
        "Uint64" => "u64",
        "Sint64" => "i64",
        "size_t" => "usize",
        _ => return None,
    })
}

/// Converts the value of a numeric C macro into a Rust `(type, expression)` pair.
///
/// Returns `None` for anything that is not a plain integer constant. Casts to
/// types other than the fixed-width SDL integers are only accepted when the type
/// is an integer alias in the generated bindings (`aliases`), so the compiler
/// enforces the real width instead of a guessed one.
fn convert_value(raw: &str, aliases: &HashSet<String>) -> Option<(String, String)> {
    let raw = strip_outer_parens(raw.trim());

    if let Some(caps) = RE_SDL_C_MACRO.captures(raw) {
        let ty = format!("{}{}", if &caps[2] == "U" { 'u' } else { 'i' }, &caps[3]);
        let literal = parse_literal(&caps[4])?;
        return Some((ty, format!("{}{literal}", caps[1].replace('~', "!"))));
    }

    if let Some(caps) = RE_CAST.captures(raw) {
        let c_type = &caps[1];
        let rest = strip_outer_parens(caps[2].trim());
        let (prefix, literal) = if let Some(rest) = rest.strip_prefix('~') {
            ("!", rest)
        } else if let Some(rest) = rest.strip_prefix('-') {
            ("-", rest)
        } else {
            ("", rest)
        };
        let literal = parse_literal(literal)?;
        let primitive = primitive_type(c_type);
        let ty = match primitive {
            Some(ty) => ty,
            None if aliases.contains(c_type) => c_type,
            None => return None,
        };
        let signed_primitive = primitive.is_some_and(|ty| ty.starts_with('i'));
        let value = if prefix == "-" && !signed_primitive {
            // A negative literal cannot be written in an unsigned (or alias) type.
            format!("(-{literal}i64) as {ty}")
        } else {
            format!("{prefix}{literal}")
        };
        return Some((ty.to_string(), value));
    }

    if let Some(caps) = RE_BIT_SHIFT.captures(raw) {
        let shift: u32 = caps[1].parse().ok()?;
        return (shift < 32).then(|| ("u32".to_string(), format!("1 << {shift}")));
    }

    let caps = RE_NUMERIC.captures(raw)?;
    let negative = !caps[1].is_empty();
    let literal = &caps[2];
    let value = literal_value(literal)?;
    let ty = if negative {
        match value {
            0..=0x7FFF_FFFF => "i32",
            0x8000_0000..=0x7FFF_FFFF_FFFF_FFFF => "i64",
            _ => return None,
        }
    } else {
        match value {
            0..=0x7FFF_FFFF => "i32",
            0x8000_0000..=0xFFFF_FFFF => "u32",
            0x1_0000_0000..=0xFFFF_FFFF_FFFF_FFFF => "u64",
            _ => return None,
        }
    };
    Some((
        ty.to_string(),
        format!("{}{literal}", if negative { "-" } else { "" }),
    ))
}

#[cfg(test)]
mod tests {
    use super::{RE_CONST_NAME, convert_value, property_type};
    use std::collections::HashSet;

    fn convert(raw: &str) -> Option<(String, String)> {
        let aliases = HashSet::from(["SDL_TouchID".to_string(), "SDL_MouseID".to_string()]);
        convert_value(raw, &aliases)
    }

    fn pair(ty: &str, value: &str) -> Option<(String, String)> {
        Some((ty.to_string(), value.to_string()))
    }

    #[test]
    fn signed_64_bit_limits_are_extracted_with_correct_values() {
        assert_eq!(
            convert("SDL_SINT64_C(0x7FFFFFFFFFFFFFFF)"),
            pair("i64", "0x7FFFFFFFFFFFFFFF")
        );
        // `!0x7FFF…` is i64::MIN; dropping the `~` would yield i64::MAX.
        assert_eq!(
            convert("~SDL_SINT64_C(0x7FFFFFFFFFFFFFFF)"),
            pair("i64", "!0x7FFFFFFFFFFFFFFF")
        );
    }

    #[test]
    fn unsigned_64_bit_limits_are_extracted() {
        assert_eq!(
            convert("SDL_UINT64_C(0xFFFFFFFFFFFFFFFF)"),
            pair("u64", "0xFFFFFFFFFFFFFFFF")
        );
        assert_eq!(
            convert("SDL_UINT64_C(0x0000000000000000)"),
            pair("u64", "0x0000000000000000")
        );
    }

    #[test]
    fn complemented_fixed_width_casts_keep_the_complement() {
        assert_eq!(convert("((Sint8)(~0x7F))"), pair("i8", "!0x7F"));
        assert_eq!(
            convert("((Sint32)(~0x7FFFFFFF))"),
            pair("i32", "!0x7FFFFFFF")
        );
    }

    #[test]
    fn alias_casts_use_the_alias_type_and_cast_negatives() {
        assert_eq!(
            convert("((SDL_TouchID)-1)"),
            pair("SDL_TouchID", "(-1i64) as SDL_TouchID")
        );
        assert_eq!(
            convert("((SDL_MouseID)0xFFFFFFFFu)"),
            pair("SDL_MouseID", "0xFFFFFFFF")
        );
        assert_eq!(convert("((size_t)-1)"), pair("usize", "(-1i64) as usize"));
    }

    #[test]
    fn unknown_cast_types_are_skipped_instead_of_guessed() {
        assert_eq!(convert("((SDL_Keymod)3)"), None);
    }

    #[test]
    fn shifts_are_only_accepted_as_a_whole_expression() {
        assert_eq!(convert("(1u << 3)"), pair("u32", "1 << 3"));
        assert_eq!(convert("(1u << 40)"), None);
        assert_eq!(convert("(1u << 3) | (1u << 4)"), None);
    }

    #[test]
    fn plain_literals_are_typed_by_range() {
        assert_eq!(convert("42"), pair("i32", "42"));
        assert_eq!(convert("-5"), pair("i32", "-5"));
        assert_eq!(convert("0x80000000"), pair("u32", "0x80000000"));
        assert_eq!(convert("0xFFFFFFFFu"), pair("u32", "0xFFFFFFFF"));
        assert_eq!(convert("0x100000000"), pair("u64", "0x100000000"));
        assert_eq!(convert("SDL_FOO + 1"), None);
    }

    #[test]
    fn existing_constants_are_matched_by_exact_name() {
        let bindings = "pub const SDL_FOO_BAR: u32 = 1;\npub const SDL_BAZ : i32 = 2;";
        let names: HashSet<_> = RE_CONST_NAME
            .captures_iter(bindings)
            .map(|caps| caps[1].to_string())
            .collect();
        assert!(names.contains("SDL_FOO_BAR"));
        assert!(names.contains("SDL_BAZ"));
        assert!(!names.contains("SDL_FOO"));
    }

    #[test]
    fn property_types_follow_the_name_suffix() {
        assert_eq!(property_type("SDL_PROP_X_STRING"), "PropertyType::String");
        assert_eq!(property_type("SDL_PROP_X_BOOLEAN"), "PropertyType::Boolean");
        assert_eq!(property_type("SDL_PROP_X_POINTER"), "PropertyType::Pointer");
    }
}
