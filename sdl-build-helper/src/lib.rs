//! # SDL Build Helper
//!
//! A utility crate for managing the build process of SDL3 and its satellite libraries.
//!
//! This crate provides `SdlBuilder`, which uses the Typestate pattern to ensure
//! correct configuration before attempting a build. It supports building from source
//! via CMake, using `pkg-config`, `vcpkg`, or standard system linkage.

use std::{
    env, fmt,
    path::{Path, PathBuf},
};

/// Errors that can occur during the SDL build process.
#[derive(Debug)]
pub enum BuildError {
    /// A mandatory environment variable was missing.
    EnvVarMissing(String),
    /// A path expected to exist was not found.
    PathNotFound(PathBuf),
    /// An error occurred during a Git operation.
    GitError(String),
    /// An error occurred during a CMake build.
    CMakeError(String),
    /// An error occurred during C compilation of inlines.
    CcError(String),
    /// Pkg-config failed to find the library.
    PkgConfigError(String),
    /// Vcpkg failed to find the library.
    VcpkgError(String),
    /// General I/O error.
    Io(std::io::Error),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvVarMissing(var) => write!(f, "Missing environment variable: {}", var),
            Self::PathNotFound(path) => write!(f, "Path not found: {}", path.display()),
            Self::GitError(err) => write!(f, "Git error: {}", err),
            Self::CMakeError(err) => write!(f, "CMake build error: {}", err),
            Self::CcError(err) => write!(f, "Inline compilation error: {}", err),
            Self::PkgConfigError(err) => write!(f, "Pkg-config error: {}", err),
            Self::VcpkgError(err) => write!(f, "Vcpkg error: {}", err),
            Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// Marker trait for SdlBuilder states.
pub trait BuilderState {}

/// Initial state: mandatory fields are being set.
pub struct Init;
impl BuilderState for Init {}

/// Ready state: configuration is complete, and `.build()` can be called.
pub struct Ready;
impl BuilderState for Ready {}

/// A builder for SDL3 and satellite libraries using the Typestate pattern.
///
/// # Example
/// ```no_run
/// use sdl_build_helper::SdlBuilder;
///
/// SdlBuilder::new("SDL3", "sdl3", "../submodules/SDL")
///     .with_cmake_option("SDL_TESTS", "OFF")
///     .build()
///     .expect("Failed to build SDL3");
/// ```
pub struct SdlBuilder<S: BuilderState> {
    link_name: String,
    lib_name: String,
    #[cfg(feature = "build-from-source")]
    source_dir: PathBuf,
    repo_url: Option<String>,
    include_dirs: Vec<PathBuf>,
    cmake_options: Vec<(String, String)>,
    cflags: Vec<String>,
    requires_base_sdl: bool,
    _state: std::marker::PhantomData<S>,
}

impl SdlBuilder<Init> {
    /// Creates a new builder for a specific library.
    ///
    /// * `link_name`: The name used for linking (e.g., "SDL3", "SDL3_image").
    /// * `lib_name`: The internal library name (e.g., "sdl3", "sdl3_image").
    /// * `source_dir`: Path to the source code (absolute or relative to manifest).
    pub fn new(
        link_name: impl Into<String>,
        lib_name: impl Into<String>,
        #[allow(unused_variables)] source_dir: impl AsRef<Path>,
    ) -> SdlBuilder<Ready> {
        SdlBuilder {
            link_name: link_name.into(),
            lib_name: lib_name.into(),
            #[cfg(feature = "build-from-source")]
            source_dir: source_dir.as_ref().to_path_buf(),
            repo_url: None,
            include_dirs: vec![PathBuf::from("src/generated/include")],
            cmake_options: vec![],
            cflags: vec![],
            requires_base_sdl: false,
            _state: std::marker::PhantomData,
        }
    }
}

impl SdlBuilder<Ready> {
    /// Sets the repository URL for "build-from-source" checkout.
    pub fn with_repo_url(mut self, url: impl Into<String>) -> Self {
        self.repo_url = Some(url.into());
        self
    }

    /// Adds an include directory to be used during inline compilation.
    pub fn include_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.include_dirs.push(dir.as_ref().to_path_buf());
        self
    }

    /// Adds a CMake definition (e.g., `SDL_TESTS=OFF`).
    pub fn with_cmake_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.cmake_options.push((key.into(), value.into()));
        self
    }

    /// Adds a C compiler flag for the inline wrapper compilation.
    pub fn with_cflag(mut self, flag: impl Into<String>) -> Self {
        self.cflags.push(flag.into());
        self
    }

    /// Specifies if this library requires the base SDL3 CMake directory.
    pub fn requires_base_sdl(mut self, req: bool) -> Self {
        self.requires_base_sdl = req;
        self
    }

    /// Executes the build process based on enabled features and environment.
    ///
    /// 1. Compiles generated inline wrappers.
    /// 2. Determines linkage strategy (Source, Pkg-Config, Vcpkg, or System).
    pub fn build(self) -> Result<(), BuildError> {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR")
                .map_err(|_| BuildError::EnvVarMissing("CARGO_MANIFEST_DIR".to_string()))?,
        );
        let target =
            env::var("TARGET").map_err(|_| BuildError::EnvVarMissing("TARGET".to_string()))?;
        let safe_target = target.replace("-", "_");

        let inlines_c = manifest_dir
            .join("src")
            .join("generated")
            .join(format!("inlines_{}.c", safe_target));

        // 1. Compile Inlines
        if inlines_c.exists() {
            let include_paths: Vec<PathBuf> = self
                .include_dirs
                .iter()
                .map(|d| manifest_dir.join(d))
                .collect();
            let include_refs: Vec<&Path> = include_paths.iter().map(|p| p.as_path()).collect();
            self.compile_inlines(&include_refs, &inlines_c)?;
        }

        // 2. Determine Link Strategy
        if env::var("CARGO_FEATURE_BUILD_FROM_SOURCE").is_ok() {
            self.build_cmake(&manifest_dir)?;
        } else if env::var("CARGO_FEATURE_USE_PKG_CONFIG").is_ok() {
            self.probe_pkg_config()?;
        } else if env::var("CARGO_FEATURE_USE_VCPKG").is_ok() {
            self.probe_vcpkg()?;
        } else {
            self.link_system();
        }

        Ok(())
    }

    fn compile_inlines(&self, include_dirs: &[&Path], inlines_c: &Path) -> Result<(), BuildError> {
        println!("cargo:rerun-if-changed={}", inlines_c.display());

        let mut build = cc::Build::new();
        build.file(inlines_c);

        for dir in include_dirs {
            println!("cargo:rerun-if-changed={}", dir.display());
            build.include(dir);
        }

        for flag in &self.cflags {
            build.flag(flag);
        }

        build.warnings(false);
        build
            .try_compile(&format!("{}_inlines", self.lib_name))
            .map_err(|e| BuildError::CcError(e.to_string()))?;
        Ok(())
    }

    fn build_cmake(&self, manifest_dir: &Path) -> Result<(), BuildError> {
        #[cfg(feature = "build-from-source")]
        {
            let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
            let link_name_upper = self.link_name.to_uppercase().replace("-", "_");
            let source_override_var = format!("{}_SOURCE_OVERRIDE", link_name_upper);

            let mut source_path = if let Ok(override_path) = env::var(&source_override_var) {
                let path = PathBuf::from(override_path);
                path.canonicalize().unwrap_or(path)
            } else if self.source_dir.is_absolute() {
                self.source_dir.clone()
            } else {
                manifest_dir.join(&self.source_dir)
            };

            if !source_path.exists() {
                let out_dir = env::var_os("OUT_DIR")
                    .ok_or_else(|| BuildError::EnvVarMissing("OUT_DIR".to_string()))?;
                source_path = PathBuf::from(out_dir).join("source");
                if !source_path.exists() {
                    self.checkout_repo(&source_path)?;
                }
            }

            let mut cfg = cmake::Config::new(source_path);
            cfg.define("BUILD_SHARED_LIBS", if link_static { "OFF" } else { "ON" });

            for (k, v) in &self.cmake_options {
                cfg.define(k, v);
            }

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
            }

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
            if link_static && env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
                println!("cargo:rustc-link-lib={}{}-static", kind, self.link_name);
            } else {
                println!("cargo:rustc-link-lib={}{}", kind, self.link_name);
            }

            let cmake_dir = self.find_cmake_dir(&dst);
            println!("cargo:cmake_dir={}", cmake_dir.display());
            Ok(())
        }
        #[cfg(not(feature = "build-from-source"))]
        {
            let _ = manifest_dir;
            Err(BuildError::CMakeError(
                "Feature 'build-from-source' not enabled but required by build environment"
                    .to_string(),
            ))
        }
    }

    #[cfg(feature = "build-from-source")]
    fn checkout_repo(&self, dest: &Path) -> Result<(), BuildError> {
        let link_name_upper = self.link_name.to_uppercase();
        let repo_override_var = format!("{}_REPOSITORY_OVERRIDE", link_name_upper);
        let branch_override_var = format!("{}_BRANCH_OVERRIDE", link_name_upper);

        let url = env::var(&repo_override_var)
            .ok()
            .or_else(|| self.repo_url.clone())
            .ok_or_else(|| BuildError::EnvVarMissing("Repository URL".to_string()))?;

        println!("cargo:info=Cloning {} into {}", url, dest.display());

        let repo = git2::Repository::clone(&url, dest)
            .map_err(|e| BuildError::GitError(format!("Failed to clone repository: {}", e)))?;

        let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
        let default_commit = version.split('-').next_back().unwrap_or("main");
        let commit = env::var(&branch_override_var).unwrap_or_else(|_| default_commit.to_string());

        println!("cargo:info=Checking out commit/branch: {}", commit);

        let (object, reference) = repo.revparse_ext(&commit).map_err(|e| {
            BuildError::GitError(format!("Failed to find commit/branch '{}': {}", commit, e))
        })?;

        repo.checkout_tree(&object, None).map_err(|e| {
            BuildError::GitError(format!("Failed to checkout tree for '{}': {}", commit, e))
        })?;

        match reference {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(object.id()),
        }
        .map_err(|e| BuildError::GitError(format!("Failed to set HEAD to '{}': {}", commit, e)))?;

        Ok(())
    }

    fn probe_pkg_config(&self) -> Result<(), BuildError> {
        #[cfg(feature = "use-pkg-config")]
        {
            let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
            pkg_config::Config::new()
                .statik(link_static)
                .probe(&self.link_name)
                .map_err(|e| BuildError::PkgConfigError(e.to_string()))?;
            Ok(())
        }
        #[cfg(not(feature = "use-pkg-config"))]
        {
            Err(BuildError::PkgConfigError(
                "Feature 'use-pkg-config' not enabled".to_string(),
            ))
        }
    }

    fn probe_vcpkg(&self) -> Result<(), BuildError> {
        #[cfg(feature = "use-vcpkg")]
        {
            vcpkg::Config::new()
                .find_package(&self.link_name)
                .map_err(|e| BuildError::VcpkgError(e.to_string()))?;
            Ok(())
        }
        #[cfg(not(feature = "use-vcpkg"))]
        {
            Err(BuildError::VcpkgError(
                "Feature 'use-vcpkg' not enabled".to_string(),
            ))
        }
    }

    fn link_system(&self) {
        let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
        let kind = if link_static { "static=" } else { "" };
        println!("cargo:rustc-link-lib={}{}", kind, self.link_name);
    }

    #[cfg(feature = "build-from-source")]
    fn find_cmake_dir(&self, dst: &Path) -> PathBuf {
        let paths = [
            dst.join("lib").join("cmake").join(&self.link_name),
            dst.join("lib64").join("cmake").join(&self.link_name),
            dst.join("cmake"),
        ];
        for p in paths {
            if p.exists() {
                return p;
            }
        }
        dst.to_path_buf()
    }
}
