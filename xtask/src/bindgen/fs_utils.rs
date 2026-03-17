use crate::bindgen::config::SysCrateConfig;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

/// Internal helper to copy all headers from configured include directories into the sys-crate's `src/generated/include` folder.
pub fn copy_headers_to_crate(config: &SysCrateConfig, root_dir: &Path, out_path: &Path) {
    let crate_include_path = out_path.join("include");

    if crate_include_path.exists() {
        fs::remove_dir_all(&crate_include_path).unwrap_or_else(|e| {
            panic!(
                "Failed to clean old include directory at {:?}: {e}",
                crate_include_path
            )
        });
    }
    fs::create_dir_all(&crate_include_path).unwrap_or_else(|e| {
        panic!(
            "Failed to create include directory at {:?}: {e}",
            crate_include_path
        )
    });

    for inc in config.include_dirs {
        let src_inc = root_dir.join(inc);
        if src_inc.exists() {
            copy_dir_all(&src_inc, &crate_include_path).unwrap_or_else(|e| {
                panic!(
                    "Failed to copy headers from {:?} to {:?}: {e}",
                    src_inc, crate_include_path
                )
            });
        } else {
            println!("WARNING: Include directory not found: {:?}", src_inc);
        }
    }
}

/// Rewrites `#include` directives in the generated inline-wrapper C file to use paths relative
/// to the crate's internal `src/include` directory.
pub fn rewrite_inlines_includes(
    inlines_c: &Path,
    config: &SysCrateConfig,
    root_dir: &Path,
    out_path: &Path,
) {
    if !inlines_c.exists() {
        return;
    }

    let content = fs::read_to_string(inlines_c)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", inlines_c.display()));
    let mut new_content = String::with_capacity(content.len());

    for line in content.lines() {
        if line.starts_with("#include") {
            let rewritten = rewrite_include_line(line, config, root_dir, out_path);
            writeln!(new_content, "#include \"{}\"", rewritten.display()).unwrap();
        } else {
            writeln!(new_content, "{}", line).unwrap();
        }
    }

    fs::write(inlines_c, new_content)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", inlines_c.display()));
}

/// Logic for mapping a submodule-based include path to a crate-internal relative path.
///
/// This function attempts to find the shortest relative path from the header's original
/// location in the submodules to the `include` directory in the generated output.
fn rewrite_include_line(
    line: &str,
    config: &SysCrateConfig,
    root_dir: &Path,
    out_path: &Path,
) -> PathBuf {
    let raw = line
        .strip_prefix("#include ")
        .expect("called on non-#include line");
    let cleaned = raw.trim().trim_matches(['<', '>', '"']);
    let path = Path::new(cleaned);
    let components: Vec<_> = path.components().collect();

    // If the path contains "submodules", it's likely a reference to an external dependency header.
    if let Some(pos) = components
        .iter()
        .position(|c| c.as_os_str() == "submodules")
    {
        let mut final_path = PathBuf::from("include");
        let full_header_path = root_dir.join(path);

        let mut found = false;
        for inc in config.include_dirs {
            let inc_path = root_dir.join(inc);
            if let Ok(rel) = full_header_path.strip_prefix(&inc_path) {
                final_path.push(rel);
                found = true;
                break;
            }
        }

        if !found {
            // Fallback: calculate a relative path based on the output directory depth.
            // This assumes a standard workspace structure where sys-crates are one level below root.
            let relative: PathBuf = components[pos..].iter().collect();
            let depth = out_path
                .strip_prefix(root_dir)
                .map(|p| p.components().count())
                .unwrap_or(3);
            let prefix = "../".repeat(depth);
            PathBuf::from(format!("{}{}", prefix, relative.display()))
        } else {
            final_path
        }
    } else {
        path.to_path_buf()
    }
}

/// Recursive directory copy that follows symlinks when they point to files or directories.
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&path, dest_path)?;
        } else if ty.is_file() {
            fs::copy(&path, dest_path)?;
        } else if ty.is_symlink() {
            // Follow the symlink for headers
            let target = fs::read_link(&path)?;
            let actual_path = if target.is_absolute() {
                target
            } else {
                path.parent().unwrap().join(target)
            };

            if actual_path.is_dir() {
                copy_dir_all(&actual_path, dest_path)?;
            } else if actual_path.is_file() {
                fs::copy(&actual_path, dest_path)?;
            }
        }
    }
    Ok(())
}
