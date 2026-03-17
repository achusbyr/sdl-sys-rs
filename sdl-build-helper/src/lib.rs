use std::env;
use std::path::{Path, PathBuf};

pub struct SdlBuilder {
    link_name: String,
    lib_name: String,
    #[cfg(feature = "build-from-source")]
    source_dir: PathBuf,
    repo_url: Option<String>,
    include_dirs: Vec<PathBuf>,
    cmake_options: Vec<(String, String)>,
    requires_base_sdl: bool,
}

impl SdlBuilder {
    pub fn new(
        link_name: impl Into<String>,
        lib_name: impl Into<String>,
        #[allow(unused_variables)] source_dir: impl AsRef<Path>,
    ) -> Self {
        Self {
            link_name: link_name.into(),
            lib_name: lib_name.into(),
            #[cfg(feature = "build-from-source")]
            source_dir: source_dir.as_ref().to_path_buf(),
            repo_url: None,
            include_dirs: vec![PathBuf::from("src/generated/include")],
            cmake_options: vec![],
            requires_base_sdl: false,
        }
    }

    pub fn with_repo_url(mut self, url: impl Into<String>) -> Self {
        self.repo_url = Some(url.into());
        self
    }

    pub fn include_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.include_dirs.push(dir.as_ref().to_path_buf());
        self
    }

    pub fn with_cmake_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.cmake_options.push((key.into(), value.into()));
        self
    }

    pub fn requires_base_sdl(mut self, req: bool) -> Self {
        self.requires_base_sdl = req;
        self
    }

    pub fn build(self) {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR")
                .map_err(|_| "CARGO_MANIFEST_DIR not set")
                .unwrap(),
        );
        let target = env::var("TARGET").map_err(|_| "TARGET not set").unwrap();
        let safe_target = target.replace("-", "_");

        let inlines_c = manifest_dir
            .join("src")
            .join("generated")
            .join(format!("inlines_{}.c", safe_target));
        let include_paths: Vec<PathBuf> = self
            .include_dirs
            .iter()
            .map(|d| manifest_dir.join(d))
            .collect();
        let include_refs: Vec<&Path> = include_paths.iter().map(|p| p.as_path()).collect();

        // 1. Compile Inlines
        if inlines_c.exists() {
            compile_inlines(&self.lib_name, &include_refs, &inlines_c);
        }

        // 2. Determine Link Strategy
        let build_from_source = env::var("CARGO_FEATURE_BUILD_FROM_SOURCE").is_ok();
        let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
        let use_pkg_config = env::var("CARGO_FEATURE_USE_PKG_CONFIG").is_ok();
        let use_vcpkg = env::var("CARGO_FEATURE_USE_VCPKG").is_ok();

        if build_from_source {
            #[cfg(feature = "build-from-source")]
            self.build_cmake(&manifest_dir, link_static);
            #[cfg(not(feature = "build-from-source"))]
            panic!("Feature 'build-from-source' is not enabled but required by environment.");
        } else if use_pkg_config {
            #[cfg(feature = "use-pkg-config")]
            pkg_config::Config::new()
                .statik(link_static)
                .probe(&self.link_name)
                .unwrap_or_else(|e| {
                    panic!("Failed to find {} using pkg-config: {}", self.link_name, e)
                });
            #[cfg(not(feature = "use-pkg-config"))]
            panic!("Feature 'use-pkg-config' is not enabled but required by environment.");
        } else if use_vcpkg {
            #[cfg(feature = "use-vcpkg")]
            vcpkg::Config::new()
                .find_package(&self.link_name)
                .unwrap_or_else(|e| panic!("Failed to find {} using vcpkg: {}", self.link_name, e));
            #[cfg(not(feature = "use-vcpkg"))]
            panic!("Feature 'use-vcpkg' is not enabled but required by environment.");
        } else {
            // Default system link
            let kind = if link_static { "static=" } else { "" };
            println!("cargo:rustc-link-lib={}{}", kind, self.link_name);
        }
    }

    #[cfg(feature = "build-from-source")]
    fn build_cmake(&self, manifest_dir: &Path, link_static: bool) {
        let mut source_path = if self.source_dir.is_absolute() {
            self.source_dir.clone()
        } else {
            manifest_dir.join(&self.source_dir)
        };

        if !source_path.exists() {
            source_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("source");
            if !source_path.exists() {
                checkout_repo(self, &source_path);
            }
        }

        let mut cfg = cmake::Config::new(source_path);

        cfg.define("BUILD_SHARED_LIBS", if link_static { "OFF" } else { "ON" });

        for (k, v) in &self.cmake_options {
            cfg.define(k, v);
        }

        let link_name_upper = self.link_name.to_uppercase().replace("-", "_");
        let cmake_override_var = format!("{}_CMAKE_OVERRIDE", link_name_upper);
        if let Ok(overrides) = env::var(&cmake_override_var) {
            for part in overrides.split_whitespace() {
                let part = part.strip_prefix("-D").unwrap_or(part);
                if let Some((k, v)) = part.split_once('=') {
                    cfg.define(k, v);
                }
            }
        }

        if self.requires_base_sdl
            && let Ok(sdl_cmake_dir) = env::var("DEP_SDL3_CMAKE_DIR")
        {
            cfg.define("SDL3_DIR", sdl_cmake_dir);
        } // Otherwise, the user is not building SDL from source, and will let CMake try to find a system installation

        let dst = cfg.build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib64").display()
        );

        let kind = if link_static { "static=" } else { "" };

        // For MSVC static builds, CMake sometimes outputs SDL3-static.lib
        if link_static && env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
            println!("cargo:rustc-link-lib={}{}-static", kind, self.link_name);
        } else {
            println!("cargo:rustc-link-lib={}{}", kind, self.link_name);
        }

        // Export CMAKE_DIR for downstream crates (like SDL_image)
        let cmake_dir = find_cmake_dir(&dst, &self.link_name);
        println!("cargo:cmake_dir={}", cmake_dir.display()); // Cargo turns this into DEP_<LIB>_CMAKE_DIR
    }
}

#[cfg(feature = "build-from-source")]
fn checkout_repo(builder: &SdlBuilder, dest: &Path) {
    let link_name_upper = builder.link_name.to_uppercase();
    let repo_override_var = format!("{}_REPOSITORY_OVERRIDE", link_name_upper);
    let branch_override_var = format!("{}_BRANCH_OVERRIDE", link_name_upper);

    let url = env::var(&repo_override_var)
        .ok()
        .or_else(|| builder.repo_url.clone())
        .expect("No repository URL provided and no override found");

    let repo = git2::Repository::clone(&url, dest).expect("Failed to clone repository");
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();

    // Attempt to extract a meaningful commit or tag from the version.
    // Versions like "0.1.0-abc1234" will use "abc1234".
    let default_commit = version.split('-').next_back().unwrap_or("main");

    let commit = env::var(&branch_override_var).unwrap_or_else(|_| default_commit.to_string());

    println!("cargo:info=Checking out commit/branch: {}", commit);

    let (object, reference) = repo.revparse_ext(&commit).unwrap_or_else(|_| {
        if env::var(&branch_override_var).is_ok() {
            panic!("Specified branch/commit override '{}' not found", commit);
        }
        // Fallback to main/master if the specific commit is not found and no override was specified
        repo.revparse_ext("main")
            .or_else(|_| repo.revparse_ext("master"))
            .expect("Target commit, 'main', or 'master' branch not found")
    });
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout tree");

    match reference {
        Some(gref) => repo.set_head(gref.name().unwrap()),
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");
}

/// Helper to locate the CMake config package output
#[cfg(feature = "build-from-source")]
fn find_cmake_dir(dst: &Path, link_name: &str) -> PathBuf {
    let paths = [
        dst.join("lib").join("cmake").join(link_name),
        dst.join("lib64").join("cmake").join(link_name),
        dst.join("cmake"),
    ];
    for p in paths {
        if p.exists() {
            return p;
        }
    }
    dst.to_path_buf()
}

/// Compile the generated inline-function wrapper C source for the current target.
pub fn compile_inlines(lib_name: &str, include_dirs: &[&Path], inlines_c: &Path) {
    println!("cargo:rerun-if-changed={}", inlines_c.display());

    let mut build = cc::Build::new();
    build.file(inlines_c);

    for dir in include_dirs {
        println!("cargo:rerun-if-changed={}", dir.display());
        build.include(dir);
    }

    build
        .warnings(false)
        .compile(&format!("{}_inlines", lib_name));
}
