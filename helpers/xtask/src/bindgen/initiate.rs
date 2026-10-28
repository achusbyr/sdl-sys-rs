use crate::bindgen::{
    callbacks,
    config::{CRATES, SysCrateConfig},
    constants,
    fs_utils::{copy_headers_to_crate, remove_crate_headers, rewrite_inlines_includes},
    target::binding_cfg,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn begin(targets: &[&str], osx_sdk: Option<PathBuf>, ios_sdk: Option<PathBuf>) {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set; run via `cargo`");
    let root_dir = PathBuf::from(manifest_dir)
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .to_path_buf();

    for config in CRATES {
        if let Err(e) = generate_crate_bindings(
            targets,
            osx_sdk.as_ref(),
            ios_sdk.as_ref(),
            config,
            &root_dir,
        ) {
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
    osx_sdk: Option<&PathBuf>,
    ios_sdk: Option<&PathBuf>,
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

    let primary_include = root_dir.join(
        config
            .include_dirs
            .last()
            .ok_or_else(|| "include_dirs must not be empty".to_string())?,
    );

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
            builder = builder.blocklist_file(".*SDL3[^_].*");
            builder = builder.raw_line("use sdl_sys_bindgen::*;");
        } else if config.lib_name == "SDL3" {
            builder = builder.allowlist_file(r".*/SDL3/.*");
        }

        // Prepare environment for cross-compilation to Apple platforms. SDKROOT is
        // restored after generation so one target's SDK never leaks into the next.
        let previous_sdkroot = env::var_os("SDKROOT");
        if target.contains("apple-darwin")
            && let Some(path) = osx_sdk
        {
            let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
            builder = builder.clang_arg(format!("-isysroot{}", abs_path.display()));
            // SAFETY: xtask is single-threaded, so no other thread reads the environment.
            unsafe { env::set_var("SDKROOT", &abs_path) };
        } else if target.contains("apple-ios")
            && let Some(path) = ios_sdk
        {
            let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
            builder = builder.clang_arg(format!("-isysroot{}", abs_path.display()));
            // SAFETY: xtask is single-threaded, so no other thread reads the environment.
            unsafe { env::set_var("SDKROOT", &abs_path) };
        }

        // Generate bindings
        let bindings = builder.generate().map_err(|e| {
            format!(
                "Failed to generate bindings for {} ({target}): {e}",
                config.lib_name
            )
        })?;

        match previous_sdkroot {
            // SAFETY: xtask is single-threaded, so no other thread reads the environment.
            Some(value) => unsafe { env::set_var("SDKROOT", value) },
            None => unsafe { env::remove_var("SDKROOT") },
        }

        // Post-process inline wrapper headers
        rewrite_inlines_includes(&inlines_c, config, root_dir, &out_path)?;

        // Write bindings to file
        bindings
            .write_to_file(&bindings_rs)
            .map_err(|e| format!("Failed to write {}: {e}", bindings_rs.display()))?;

        // Extract macros for this specific target
        constants::append_macro_constants(&primary_include, &bindings_rs, config.lib_name)?;

        // Add conditional compilation flags to mod.rs
        let cfg = binding_cfg(target);
        mod_rs_content.push_str(&format!("#[cfg({cfg})]\nmod bindings_{safe_target};\n"));
        mod_rs_content.push_str(&format!(
            "#[cfg({cfg})]\npub use bindings_{safe_target}::*;\n\n"
        ));
    }

    // The bundled headers only exist so the build script can compile the inline
    // wrappers; crates without any wrappers must not ship them.
    let has_inline_wrappers = targets.iter().any(|target| {
        out_path
            .join(format!("inlines_{}.c", target.replace('-', "_")))
            .is_file()
    });
    if has_inline_wrappers {
        copy_headers_to_crate(config, root_dir, &out_path)?;
    } else {
        remove_crate_headers(&out_path)?;
    }

    // Add compile_error! fallback for unsupported targets
    mod_rs_content.push_str("#[cfg(not(any(");
    for (i, &target) in targets.iter().enumerate() {
        let cfg = binding_cfg(target);
        if i > 0 {
            mod_rs_content.push_str(", ");
        }
        mod_rs_content.push_str(&cfg);
    }
    mod_rs_content.push_str(")))]\ncompile_error!(\"Unsupported target: Windows requires a GNU-environment target such as x86_64-pc-windows-gnu or x86_64-pc-windows-gnullvm; for other platforms, generate bindings with cargo xtask\");\n");

    // Extract target-agnostic constants (string/doc based)
    constants::extract_and_generate(
        &primary_include,
        &out_path.join("constants.rs"),
        config.lib_name,
        config.hint_prefix,
        config.prop_prefix,
    )?;

    // Finalize mod.rs
    fs::write(out_path.join("mod.rs"), mod_rs_content)
        .map_err(|e| format!("Failed to write mod.rs: {e}"))?;

    Ok(())
}
