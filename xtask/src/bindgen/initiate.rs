use crate::bindgen::config::{CRATES, SysCrateConfig};
use crate::bindgen::fs_utils::{copy_headers_to_crate, rewrite_inlines_includes};
use crate::bindgen::target::parse_target_triple;
use crate::bindgen::{callbacks, constants};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn begin(targets: &[&str], read_sdkroot_env: bool) {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set; run via `cargo`");
    let root_dir = PathBuf::from(manifest_dir)
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .to_path_buf();

    for config in CRATES {
        if let Err(e) = generate_crate_bindings(targets, read_sdkroot_env, config, &root_dir) {
            eprintln!(
                "Error: Failed to generate bindings for {}: {}",
                config.lib_name, e
            );
            std::process::exit(1);
        }
    }
}

fn generate_crate_bindings(
    targets: &[&str],
    read_sdkroot_env: bool,
    config: &SysCrateConfig,
    root_dir: &Path,
) -> Result<(), String> {
    println!("Generating bindings for {}...", config.lib_name);

    let out_path = root_dir.join(config.out_dir);
    if let Err(e) = fs::create_dir_all(&out_path) {
        return Err(format!("Failed to create {}: {}", out_path.display(), e));
    }

    let mut mod_rs_content = String::new();
    mod_rs_content.push_str("pub mod constants;\n\n");

    // Prepare portable headers in the sys-crate
    copy_headers_to_crate(config, root_dir, &out_path);

    for &target in targets {
        println!("-> Target: {}", target);

        let safe_target = target.replace("-", "_");
        let bindings_rs = out_path.join(format!("bindings_{}.rs", safe_target));
        let inlines_c = out_path.join(format!("inlines_{}.c", safe_target));

        let mut builder = bindgen::Builder::default()
            .use_core()
            .ctypes_prefix("core::ffi")
            .default_enum_style(bindgen::EnumVariation::NewType {
                is_bitfield: false,
                is_global: false,
            })
            .wrap_static_fns(true)
            .wrap_static_fns_path(&inlines_c)
            .derive_debug(true)
            .derive_default(true)
            .derive_copy(true)
            .derive_hash(true)
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(callbacks::SdlParseCallback))
            .clang_arg(format!("--target={}", target));

        // Emscripten-specific setup
        if target == "wasm32-unknown-emscripten" {
            if let Ok(emsdk) = std::env::var("EMSDK") {
                let sysroot = format!("{}/upstream/emscripten/cache/sysroot", emsdk);
                builder = builder.clang_arg(format!("--sysroot={}", sysroot));
                builder = builder.clang_arg(format!("-I{}/include", sysroot));
            } else {
                eprintln!("WARNING: EMSDK environment variable not set");
            }
        }

        // Add headers from config
        for header_file in config.headers {
            let header_path = root_dir.join(header_file);
            builder = builder.header(header_path.to_str().expect("Non-UTF-8 header path"));
        }

        // Add include directories from config
        for inc in config.include_dirs {
            builder = builder.clang_arg(format!("-I{}", root_dir.join(inc).display()));
        }

        // Apply allowlist if specified
        if let Some(allowlist) = config.allowlist_file {
            builder = builder.allowlist_file(allowlist);
            builder = builder.raw_line("use sdl_sys_bindgen::*;");
        }

        // Generate bindings
        if read_sdkroot_env {
            if target.contains("darwin") {
                unsafe {
                    std::env::set_var(
                        "SDKROOT",
                        std::env::var("OSX_SDKROOT")
                            .map_err(|e| format!("Failed to get OSX_SDKROOT: {e}"))?,
                    )
                };
            } else if target.contains("ios") {
                unsafe {
                    std::env::set_var(
                        "SDKROOT",
                        std::env::var("IOS_SDKROOT")
                            .map_err(|e| format!("Failed to get IOS_SDKROOT: {e}"))?,
                    )
                };
            }
        }

        let bindings = builder
            .generate()
            .unwrap_or_else(|e| panic!("Failed to generate bindings for {}: {e}", config.lib_name));

        // Post-process inline wrapper headers
        rewrite_inlines_includes(&inlines_c, config, root_dir, &out_path);

        // Write bindings to file
        bindings
            .write_to_file(&bindings_rs)
            .unwrap_or_else(|e| panic!("Failed to write {}: {e}", bindings_rs.display()));

        // Extract macros for this specific target
        let primary_include = root_dir.join(
            config
                .include_dirs
                .last()
                .ok_or_else(|| "include_dirs must not be empty".to_string())?,
        );
        constants::append_macro_constants(&primary_include, &bindings_rs, config.lib_name);

        // Add conditional compilation flags to mod.rs
        let (cfg_os, cfg_arch) = parse_target_triple(target);
        mod_rs_content.push_str(&format!(
            "#[cfg(all(target_os = \"{}\", target_arch = \"{}\"))]\nmod bindings_{};\n",
            cfg_os, cfg_arch, safe_target
        ));
        mod_rs_content.push_str(&format!(
            "#[cfg(all(target_os = \"{}\", target_arch = \"{}\"))]\npub use bindings_{}::*;\n\n",
            cfg_os, cfg_arch, safe_target
        ));
    }

    // Add compile_error! fallback for unsupported targets
    mod_rs_content.push_str("#[cfg(not(any(");
    for (i, &target) in targets.iter().enumerate() {
        let (cfg_os, cfg_arch) = parse_target_triple(target);
        if i > 0 {
            mod_rs_content.push_str(", ");
        }
        mod_rs_content.push_str(&format!(
            "all(target_os = \"{}\", target_arch = \"{}\")",
            cfg_os, cfg_arch
        ));
    }
    mod_rs_content.push_str(")))]\ncompile_error!(\"Unsupported target: run sdl-bindgen to generate bindings for this platform\");\n");

    // Extract target-agnostic constants (string/doc based)
    let primary_include = root_dir.join(
        config
            .include_dirs
            .last()
            .ok_or_else(|| "include_dirs must not be empty".to_string())?,
    );
    constants::extract_and_generate(
        &primary_include,
        &out_path.join("constants.rs"),
        config.lib_name,
        config.hint_prefix,
        config.prop_prefix,
    );

    // Finalize mod.rs
    fs::write(out_path.join("mod.rs"), mod_rs_content)
        .map_err(|e| format!("Failed to write mod.rs: {e}"))?;

    Ok(())
}
