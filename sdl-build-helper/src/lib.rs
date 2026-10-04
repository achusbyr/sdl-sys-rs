//! # SDL Build Helper
//!
//! A utility crate for managing the build process of SDL3 and its satellite libraries.
//!
//! This crate provides `SdlBuilder`, a builder that compiles the generated inline
//! wrappers and links the library. It supports building from source via CMake,
//! using `pkg-config`, `vcpkg`, or standard system linkage.

#[cfg(feature = "build-from-source")]
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    env, fmt, fs,
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
    /// The build target is not supported.
    UnsupportedTarget(String),
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
            Self::UnsupportedTarget(err) => write!(f, "Unsupported target: {}", err),
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

/// A builder for SDL3 and satellite libraries.
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
pub struct SdlBuilder {
    link_name: String,
    lib_name: String,
    #[cfg(feature = "build-from-source")]
    source_dir: PathBuf,
    repo_url: Option<String>,
    include_dirs: Vec<PathBuf>,
    cmake_options: Vec<(String, String)>,
    cflags: Vec<String>,
    requires_base_sdl: bool,
}

impl SdlBuilder {
    /// Creates a new builder for a specific library.
    ///
    /// * `link_name`: The name used for linking (e.g., "SDL3", "SDL3_image").
    /// * `lib_name`: The internal library name (e.g., "sdl3", "sdl3_image").
    /// * `source_dir`: Path to the source code (absolute or relative to manifest).
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
            cflags: vec![],
            requires_base_sdl: false,
        }
    }

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

    /// Name of an override variable, e.g. `SDL3_SOURCE_OVERRIDE`.
    fn override_var(&self, suffix: &str) -> String {
        format!(
            "{}_{suffix}_OVERRIDE",
            self.link_name.to_uppercase().replace('-', "_")
        )
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
        validate_target(
            &env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default(),
            &env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default(),
        )?;

        println!(
            "cargo:rerun-if-changed={}",
            manifest_dir.join("build.rs").display()
        );
        for suffix in ["SOURCE", "REPOSITORY", "BRANCH", "CMAKE"] {
            println!("cargo:rerun-if-env-changed={}", self.override_var(suffix));
        }

        let generated_dir = manifest_dir.join("src").join("generated");

        // 1. Compile Inlines
        match find_inlines(&generated_dir, &target) {
            Some(inlines_c) => {
                let include_paths: Vec<PathBuf> = self
                    .include_dirs
                    .iter()
                    .map(|d| manifest_dir.join(d))
                    .collect();
                let include_refs: Vec<&Path> = include_paths.iter().map(|p| p.as_path()).collect();
                self.compile_inlines(&include_refs, &inlines_c)?;
            }
            // Crates without header-only wrappers have no inline files at all.
            None if !has_inline_files(&generated_dir) => {}
            // A crate with wrappers but no file for this target would produce
            // undefined `__extern` symbols at link time; fail early instead.
            None => {
                return Err(BuildError::CcError(format!(
                    "No inline wrappers found for target '{target}' in {}. Run `cargo xtask` to regenerate bindings for this target.",
                    generated_dir.display()
                )));
            }
        }

        // 2. Determine Link Strategy (compile-time feature selection, in
        // priority order: build-from-source > use-pkg-config > use-vcpkg >
        // system linkage). The crate's own features gate the strategies
        // directly, so the runtime CARGO_FEATURE_* probes they replaced were
        // always redundant with these cfgs.
        #[cfg(feature = "build-from-source")]
        {
            self.build_cmake(&manifest_dir)?;
        }
        #[cfg(all(not(feature = "build-from-source"), feature = "use-pkg-config"))]
        {
            self.probe_pkg_config()?;
        }
        #[cfg(all(
            not(feature = "build-from-source"),
            not(feature = "use-pkg-config"),
            feature = "use-vcpkg"
        ))]
        {
            self.probe_vcpkg()?;
        }
        #[cfg(not(any(
            feature = "build-from-source",
            feature = "use-pkg-config",
            feature = "use-vcpkg"
        )))]
        {
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

    #[cfg(feature = "build-from-source")]
    fn build_cmake(&self, manifest_dir: &Path) -> Result<(), BuildError> {
        let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();

        let source_path = if let Ok(override_path) = env::var(self.override_var("SOURCE")) {
            let source = validated_source_path(&manifest_dir.join(override_path))?;
            println!("cargo:rerun-if-changed={}", source.display());
            source
        } else {
            let configured_source = if self.source_dir.is_absolute() {
                self.source_dir.clone()
            } else {
                manifest_dir.join(&self.source_dir)
            };
            if configured_source.join("CMakeLists.txt").is_file() {
                println!("cargo:rerun-if-changed={}", configured_source.display());
                configured_source
            } else {
                let out_dir = env::var_os("OUT_DIR")
                    .ok_or_else(|| BuildError::EnvVarMissing("OUT_DIR".to_string()))?;
                self.checkout_repo(&PathBuf::from(out_dir).join("source"))?
            }
        };

        let mut cfg = cmake::Config::new(source_path);
        cfg.define("BUILD_SHARED_LIBS", if link_static { "OFF" } else { "ON" });

        for (k, v) in &self.cmake_options {
            cfg.define(k, v);
        }

        let cmake_override_var = self.override_var("CMAKE");
        if let Ok(overrides) = env::var(&cmake_override_var) {
            for part in overrides.split_whitespace() {
                let part = part.strip_prefix("-D").unwrap_or(part);
                if let Some((k, v)) = part.split_once('=') {
                    cfg.define(k, v);
                } else {
                    println!(
                        "cargo::warning=Ignoring '{part}' in {cmake_override_var}: expected KEY=VALUE or -DKEY=VALUE"
                    );
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

        if link_static {
            self.link_source_static(&dst)?;
        } else {
            self.link_system();
        }

        let cmake_dir = self.find_cmake_dir(&dst);
        println!("cargo:cmake_dir={}", cmake_dir.display());
        Ok(())
    }

    #[cfg(feature = "build-from-source")]
    fn link_source_static(&self, dst: &Path) -> Result<(), BuildError> {
        let package = package_name(&self.link_name);
        let pc_file = ["lib", "lib64", "share"]
            .into_iter()
            .map(|dir| {
                dst.join(dir)
                    .join("pkgconfig")
                    .join(format!("{package}.pc"))
            })
            .find(|path| path.is_file())
            .ok_or_else(|| {
                BuildError::PathNotFound(dst.join("lib/pkgconfig").join(format!("{package}.pc")))
            })?;
        println!("cargo:rerun-if-changed={}", pc_file.display());
        let mut config = pkg_config::Config::new();
        config.statik(true).cargo_metadata(false);
        // pkgconf's per-command search path avoids mutating the build script's
        // environment. Include the source-built core before any system SDL3.
        let mut prefixes = vec![dst.to_path_buf()];
        if self.requires_base_sdl
            && let Ok(cmake_dir) = env::var("DEP_SDL3_CMAKE_DIR")
            && let Some(prefix) = install_prefix(Path::new(&cmake_dir))
        {
            prefixes.push(prefix);
        }
        for prefix in &prefixes {
            for dir in ["lib/pkgconfig", "lib64/pkgconfig", "share/pkgconfig"] {
                config.arg(format!("--with-path={}", prefix.join(dir).display()));
            }
        }
        let library = config
            .probe(&package)
            .map_err(|error| BuildError::PkgConfigError(format!("Static source-build dependency discovery requires pkgconf with --with-path support: {error}")))?;
        // Validate before emitting any cargo metadata so a failure leaves no partial output.
        if !library.link_files.is_empty() {
            return Err(BuildError::PkgConfigError(
                "Explicit library file paths in static source-build metadata are not supported"
                    .to_string(),
            ));
        }
        for path in &library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for path in &library.framework_paths {
            println!("cargo:rustc-link-search=framework={}", path.display());
        }
        for name in &library.libs {
            let kind = if prefixes.iter().any(|prefix| {
                ["lib", "lib64"]
                    .into_iter()
                    .any(|dir| prefix.join(dir).join(format!("lib{name}.a")).is_file())
            }) {
                "static="
            } else {
                ""
            };
            println!("cargo:rustc-link-lib={kind}{name}");
        }
        for name in &library.frameworks {
            println!("cargo:rustc-link-lib=framework={name}");
        }
        for args in &library.ld_args {
            println!("cargo:rustc-link-arg=-Wl,{}", args.join(","));
        }
        Ok(())
    }

    #[cfg(feature = "build-from-source")]
    fn checkout_repo(&self, dest: &Path) -> Result<PathBuf, BuildError> {
        let url = env::var(self.override_var("REPOSITORY"))
            .ok()
            .or_else(|| self.repo_url.clone())
            .ok_or_else(|| BuildError::EnvVarMissing("Repository URL".to_string()))?;
        let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
        let default_revision = version
            .rsplit_once('-')
            .map(|(_, revision)| revision)
            .filter(|revision| !revision.is_empty())
            .unwrap_or("main");
        let revision =
            env::var(self.override_var("BRANCH")).unwrap_or_else(|_| default_revision.to_string());

        println!("cargo::warning=Preparing {url} at {revision}");
        ensure_checkout(dest, &url, &revision)
    }

    #[cfg(all(feature = "use-pkg-config", not(feature = "build-from-source")))]
    fn probe_pkg_config(&self) -> Result<(), BuildError> {
        let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
        pkg_config::Config::new()
            .statik(link_static)
            .probe(&package_name(&self.link_name))
            .map_err(|e| BuildError::PkgConfigError(e.to_string()))?;
        Ok(())
    }

    #[cfg(all(
        feature = "use-vcpkg",
        not(feature = "build-from-source"),
        not(feature = "use-pkg-config")
    ))]
    fn probe_vcpkg(&self) -> Result<(), BuildError> {
        vcpkg::Config::new()
            .find_package(&package_name(&self.link_name))
            .map_err(|e| BuildError::VcpkgError(e.to_string()))?;
        Ok(())
    }

    // Reachable from the default (no-strategy-feature) branch and from
    // build_cmake's shared-linkage path; unused when only a probe feature is on.
    #[cfg(any(
        feature = "build-from-source",
        not(any(feature = "use-pkg-config", feature = "use-vcpkg"))
    ))]
    fn link_system(&self) {
        let link_static = env::var("CARGO_FEATURE_LINK_STATIC").is_ok();
        let kind = if link_static { "static=" } else { "" };
        println!("cargo:rustc-link-lib={kind}{}", self.link_name);
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

#[cfg(feature = "build-from-source")]
struct CacheLock {
    _file: fs::File,
}

#[cfg(feature = "build-from-source")]
impl CacheLock {
    fn acquire(path: PathBuf) -> Result<Self, BuildError> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        file.lock()?;
        Ok(Self { _file: file })
    }
}

#[cfg(feature = "build-from-source")]
struct CacheStage {
    path: PathBuf,
}

#[cfg(feature = "build-from-source")]
impl Drop for CacheStage {
    fn drop(&mut self) {
        remove_path(&self.path);
    }
}

#[cfg(feature = "build-from-source")]
fn remove_path(path: &Path) {
    let result = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    if let Err(error) = result
        && error.kind() != std::io::ErrorKind::NotFound
    {
        eprintln!(
            "Failed to clean temporary source path {}: {error}",
            path.display()
        );
    }
}

#[cfg(feature = "build-from-source")]
fn cleanup_cache_temps(parent: &Path, name: &str) {
    let prefix = format!("{name}.staging-");
    let entries = match fs::read_dir(parent) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!(
                "Failed to inspect source cache directory {}: {error}",
                parent.display()
            );
            return;
        }
    };
    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let file_name = entry.file_name();
        if file_name.to_string_lossy().starts_with(&prefix) {
            remove_path(&entry.path());
        }
    }
}

#[cfg(feature = "build-from-source")]
fn cleanup_cache_backups(parent: &Path, name: &str) {
    let prefix = format!("{name}.backup-");
    let Ok(entries) = fs::read_dir(parent) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().starts_with(&prefix) {
            remove_path(&entry.path());
        }
    }
}

#[cfg(feature = "build-from-source")]
fn restore_cached_backup(
    parent: &Path,
    dest: &Path,
    name: &str,
    url: &str,
    revision: &str,
) -> Result<(), BuildError> {
    let prefix = format!("{name}.backup-");
    let entries = fs::read_dir(parent)?;
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().starts_with(&prefix)
            && cache_is_valid(&entry.path(), url, revision)
        {
            fs::rename(entry.path(), dest)?;
            return Ok(());
        }
    }
    Ok(())
}

#[cfg(feature = "build-from-source")]
fn unique_sibling(path: &Path, suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    path.with_file_name(format!("{name}.{suffix}-{}-{nanos}", std::process::id()))
}

#[cfg(feature = "build-from-source")]
fn cache_metadata(url: &str, revision: &str, commit_id: &str) -> Vec<u8> {
    format!(
        "{}:{url}{}:{revision}{commit_id}",
        url.len(),
        revision.len()
    )
    .into_bytes()
}

#[cfg(feature = "build-from-source")]
fn resolve_commit<'repo>(
    repo: &'repo git2::Repository,
    revision: &str,
) -> Result<git2::Commit<'repo>, git2::Error> {
    repo.revparse_single(revision)
        .or_else(|_| repo.revparse_single(&format!("refs/remotes/origin/{revision}")))?
        .peel_to_commit()
}

#[cfg(feature = "build-from-source")]
fn cache_is_valid(dest: &Path, url: &str, revision: &str) -> bool {
    let Ok(repo) = git2::Repository::open(dest) else {
        return false;
    };
    let Ok(head) = repo.head() else {
        return false;
    };
    if !repo.head_detached().unwrap_or(false) {
        return false;
    }
    let Some(commit_id) = head.target() else {
        return false;
    };
    let expected = cache_metadata(url, revision, &commit_id.to_string());
    if fs::read(repo.path().join("sdl-sys-rs-cache"))
        .ok()
        .as_deref()
        != Some(expected.as_slice())
    {
        return false;
    }
    let Ok(origin) = repo.find_remote("origin") else {
        return false;
    };
    if origin.url() != Ok(url) {
        return false;
    }
    if resolve_commit(&repo, revision).map(|commit| commit.id()) != Ok(commit_id) {
        return false;
    }
    if is_repository_dirty(&repo) {
        return false;
    }
    dest.join("CMakeLists.txt").is_file() && submodules_are_ready(&repo, 0)
}

#[cfg(feature = "build-from-source")]
fn is_repository_dirty(repo: &git2::Repository) -> bool {
    let mut options = git2::StatusOptions::new();
    options
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        .exclude_submodules(true);
    repo.statuses(Some(&mut options))
        .map(|statuses| !statuses.is_empty())
        .unwrap_or(true)
}

#[cfg(feature = "build-from-source")]
fn submodules_are_ready(repo: &git2::Repository, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    let Ok(submodules) = repo.submodules() else {
        return false;
    };
    for submodule in submodules {
        let Some(expected_id) = submodule.head_id() else {
            return false;
        };
        if submodule.workdir_id() != Some(expected_id) {
            return false;
        }
        let Ok(child_repo) = submodule.open() else {
            return false;
        };
        if is_repository_dirty(&child_repo) || !submodules_are_ready(&child_repo, depth + 1) {
            return false;
        }
    }
    true
}

#[cfg(feature = "build-from-source")]
fn update_submodules(repo: &git2::Repository, depth: usize) -> Result<(), BuildError> {
    if depth > 32 {
        return Err(BuildError::GitError(
            "Submodule nesting exceeds the supported depth of 32".to_string(),
        ));
    }
    let mut submodules = repo
        .submodules()
        .map_err(|error| BuildError::GitError(format!("Failed to read submodules: {error}")))?;
    for submodule in &mut submodules {
        let path = submodule.path().display().to_string();
        submodule.update(true, None).map_err(|error| {
            BuildError::GitError(format!("Failed to initialize submodule {path}: {error}"))
        })?;
        let child_repo = submodule.open().map_err(|error| {
            BuildError::GitError(format!("Failed to open submodule {path}: {error}"))
        })?;
        update_submodules(&child_repo, depth + 1)?;
    }
    Ok(())
}

#[cfg(feature = "build-from-source")]
fn ensure_checkout(dest: &Path, url: &str, revision: &str) -> Result<PathBuf, BuildError> {
    let parent = dest
        .parent()
        .ok_or_else(|| BuildError::PathNotFound(dest.to_path_buf()))?;
    fs::create_dir_all(parent)?;
    let name = dest.file_name().unwrap_or_default().to_string_lossy();
    let lock_path = dest.with_file_name(format!("{name}.lock"));
    let _cache_lock = CacheLock::acquire(lock_path)?;
    cleanup_cache_temps(parent, &name);

    if !dest.exists() {
        restore_cached_backup(parent, dest, &name, url, revision)?;
    }
    if cache_is_valid(dest, url, revision) {
        cleanup_cache_backups(parent, &name);
        return Ok(dest.to_path_buf());
    }

    let stage_path = unique_sibling(dest, "staging");
    let stage = CacheStage {
        path: stage_path.clone(),
    };
    println!("cargo::warning=Cloning {url} into {}", stage_path.display());
    let repo = git2::Repository::clone(url, &stage_path)
        .map_err(|error| BuildError::GitError(format!("Failed to clone {url}: {error}")))?;
    let commit = resolve_commit(&repo, revision).map_err(|error| {
        BuildError::GitError(format!(
            "Failed to resolve commit revision '{revision}' in {url}: {error}"
        ))
    })?;
    let mut checkout = git2::build::CheckoutBuilder::new();
    checkout.force();
    repo.checkout_tree(commit.as_object(), Some(&mut checkout))
        .map_err(|error| {
            BuildError::GitError(format!("Failed to check out '{revision}': {error}"))
        })?;
    repo.set_head_detached(commit.id()).map_err(|error| {
        BuildError::GitError(format!("Failed to detach at '{revision}': {error}"))
    })?;
    if !stage_path.join("CMakeLists.txt").is_file() {
        return Err(BuildError::PathNotFound(stage_path.join("CMakeLists.txt")));
    }
    update_submodules(&repo, 0)?;
    fs::write(
        repo.path().join("sdl-sys-rs-cache"),
        cache_metadata(url, revision, &commit.id().to_string()),
    )?;

    let backup_path = unique_sibling(dest, "backup");
    let had_previous = dest.exists();
    if had_previous {
        fs::rename(dest, &backup_path)?;
    }
    if let Err(error) = fs::rename(&stage_path, dest) {
        if had_previous && let Err(restore_error) = fs::rename(&backup_path, dest) {
            return Err(BuildError::Io(std::io::Error::new(
                restore_error.kind(),
                format!(
                    "Failed to restore old source cache after promotion error ({error}): {restore_error}"
                ),
            )));
        }
        return Err(BuildError::Io(error));
    }
    if had_previous {
        remove_path(&backup_path);
    }
    cleanup_cache_backups(parent, &name);
    drop(stage);
    Ok(dest.to_path_buf())
}

#[cfg(any(
    feature = "build-from-source",
    feature = "use-pkg-config",
    feature = "use-vcpkg",
    test
))]
fn package_name(link_name: &str) -> String {
    link_name.to_lowercase().replace('_', "-")
}

fn validate_target(target_family: &str, target_env: &str) -> Result<(), BuildError> {
    if target_family == "windows" && !matches!(target_env, "gnu" | "gnullvm") {
        return Err(BuildError::UnsupportedTarget(format!(
            "Unsupported Windows environment '{target_env}': use a GNU-environment target such as x86_64-pc-windows-gnu or x86_64-pc-windows-gnullvm; MSVC is not supported"
        )));
    }
    Ok(())
}

/// The stem of the inline wrapper file a target should use.
///
/// Windows GNU-environment targets (gnu and gnullvm) share one wrapper set.
fn inline_file_stem(target: &str) -> String {
    if let Some((prefix, _env)) = target.rsplit_once('-')
        && prefix.ends_with("-windows")
    {
        return format!("{}_gnu", prefix.replace('-', "_"));
    }
    target.replace('-', "_")
}

/// Finds the inline wrapper file for a target, preferring an exact match and
/// falling back to the shared family file.
fn find_inlines(generated_dir: &Path, target: &str) -> Option<PathBuf> {
    let exact = generated_dir.join(format!("inlines_{}.c", target.replace('-', "_")));
    if exact.is_file() {
        return Some(exact);
    }
    let family = generated_dir.join(format!("inlines_{}.c", inline_file_stem(target)));
    family.is_file().then_some(family)
}

/// Whether a crate ships any inline wrapper files at all.
fn has_inline_files(generated_dir: &Path) -> bool {
    fs::read_dir(generated_dir)
        .map(|entries| {
            entries.flatten().any(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                name.starts_with("inlines_") && name.ends_with(".c")
            })
        })
        .unwrap_or(false)
}

/// Derives the install prefix from a CMake package directory.
///
/// Handles `<prefix>/lib/cmake/<Name>`, `<prefix>/lib64/cmake/<Name>` and
/// `<prefix>/cmake`.
#[cfg(feature = "build-from-source")]
fn install_prefix(cmake_dir: &Path) -> Option<PathBuf> {
    let cmake_root = cmake_dir
        .ancestors()
        .find(|path| path.file_name().is_some_and(|name| name == "cmake"))?;
    let parent = cmake_root.parent()?;
    let in_libdir = parent
        .file_name()
        .is_some_and(|name| name == "lib" || name == "lib64");
    Some(if in_libdir { parent.parent()? } else { parent }.to_path_buf())
}

#[cfg(feature = "build-from-source")]
fn validated_source_path(path: &Path) -> Result<PathBuf, BuildError> {
    if !path.is_dir() {
        return Err(BuildError::PathNotFound(path.to_path_buf()));
    }
    if !path.join("CMakeLists.txt").is_file() {
        return Err(BuildError::PathNotFound(path.join("CMakeLists.txt")));
    }
    Ok(path.canonicalize()?)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "build-from-source")]
    use super::{
        BuildError, cache_is_valid, ensure_checkout, install_prefix, validated_source_path,
    };
    use super::{find_inlines, has_inline_files, inline_file_stem, package_name, validate_target};
    #[cfg(feature = "build-from-source")]
    use git2::{Repository, Signature};
    use std::fs;
    #[cfg(feature = "build-from-source")]
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[cfg(feature = "build-from-source")]
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    #[cfg(feature = "build-from-source")]
    struct TestDir(PathBuf);

    #[cfg(feature = "build-from-source")]
    impl TestDir {
        fn new() -> Self {
            static NEXT_ID: AtomicU64 = AtomicU64::new(0);
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "sdl-build-helper-{}-{timestamp}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    #[cfg(feature = "build-from-source")]
    impl Drop for TestDir {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.0)
                && error.kind() != std::io::ErrorKind::NotFound
            {
                eprintln!(
                    "Failed to remove test directory {}: {error}",
                    self.0.display()
                );
            }
        }
    }

    #[cfg(feature = "build-from-source")]
    fn commit_all(repo: &Repository, message: &str) -> git2::Oid {
        let mut index = repo.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = Signature::now("Test", "test@example.invalid").unwrap();
        let parent = repo.head().ok().and_then(|head| head.peel_to_commit().ok());
        let parents: Vec<_> = parent.iter().collect();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )
        .unwrap()
    }

    #[cfg(feature = "build-from-source")]
    fn create_source_repo(path: &Path) -> (Repository, git2::Oid) {
        let repo = Repository::init(path).unwrap();
        fs::write(path.join("CMakeLists.txt"), "project(test)\n").unwrap();
        let first = commit_all(&repo, "first");
        (repo, first)
    }

    #[cfg(feature = "build-from-source")]
    fn add_submodule(repo: &Repository, url: &str, path: &str) {
        let mut submodule = repo.submodule(url, Path::new(path), true).unwrap();
        submodule.clone(None).unwrap();
        submodule.add_to_index(true).unwrap();
        submodule.add_finalize().unwrap();
    }

    #[test]
    fn discovery_names_match_upstream_packages() {
        for (link, package) in [
            ("SDL3", "sdl3"),
            ("SDL3_image", "sdl3-image"),
            ("SDL3_mixer", "sdl3-mixer"),
            ("SDL3_ttf", "sdl3-ttf"),
        ] {
            assert_eq!(package_name(link), package);
        }
    }

    #[test]
    fn windows_requires_a_gnu_environment() {
        assert!(validate_target("windows", "gnu").is_ok());
        assert!(validate_target("windows", "gnullvm").is_ok());
        assert!(validate_target("unix", "gnu").is_ok());
        assert!(validate_target("unix", "").is_ok());
        assert!(validate_target("windows", "msvc").is_err());
        assert!(validate_target("windows", "").is_err());
    }

    #[test]
    fn windows_gnu_targets_share_one_inline_wrapper() {
        assert_eq!(
            inline_file_stem("x86_64-pc-windows-gnu"),
            "x86_64_pc_windows_gnu"
        );
        assert_eq!(
            inline_file_stem("x86_64-pc-windows-gnullvm"),
            "x86_64_pc_windows_gnu"
        );
        assert_eq!(
            inline_file_stem("x86_64-unknown-linux-gnu"),
            "x86_64_unknown_linux_gnu"
        );
    }

    #[test]
    fn inline_lookup_prefers_exact_then_family_and_detects_presence() {
        let dir = std::env::temp_dir().join(format!(
            "sdl-build-helper-inlines-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();

        assert!(!has_inline_files(&dir));
        assert!(find_inlines(&dir, "x86_64-pc-windows-gnullvm").is_none());

        let gnu = dir.join("inlines_x86_64_pc_windows_gnu.c");
        fs::write(&gnu, "// wrapper\n").unwrap();
        assert!(has_inline_files(&dir));
        // gnullvm resolves to the shared GNU wrapper.
        assert_eq!(
            find_inlines(&dir, "x86_64-pc-windows-gnullvm"),
            Some(gnu.clone())
        );
        // An exact match still wins when present.
        let exact = dir.join("inlines_x86_64_unknown_linux_gnu.c");
        fs::write(&exact, "// wrapper\n").unwrap();
        assert_eq!(find_inlines(&dir, "x86_64-unknown-linux-gnu"), Some(exact));
        // A crate with wrappers but no match for this target returns None.
        assert!(find_inlines(&dir, "aarch64-apple-darwin").is_none());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn checkout_cache_tracks_remote_revision_and_repairs_partial_directories() {
        let temp = TestDir::new();
        let source_path = temp.path().join("upstream");
        let (source, first) = create_source_repo(&source_path);
        fs::write(source_path.join("change.txt"), "second").unwrap();
        let second = commit_all(&source, "second");
        let url = source_path.to_string_lossy().to_string();
        let cache = temp.path().join("cache/source");

        ensure_checkout(&cache, &url, &first.to_string()).unwrap();
        assert_eq!(
            Repository::open(&cache).unwrap().head().unwrap().target(),
            Some(first)
        );
        let interrupted_backup = cache.with_file_name("source.backup-interrupted");
        fs::rename(&cache, &interrupted_backup).unwrap();
        ensure_checkout(&cache, &url, &first.to_string()).unwrap();
        assert_eq!(
            Repository::open(&cache).unwrap().head().unwrap().target(),
            Some(first)
        );
        ensure_checkout(&cache, &url, &first.to_string()).unwrap();
        assert_eq!(
            Repository::open(&cache).unwrap().head().unwrap().target(),
            Some(first)
        );
        ensure_checkout(&cache, &url, &second.to_string()).unwrap();
        assert_eq!(
            Repository::open(&cache).unwrap().head().unwrap().target(),
            Some(second)
        );

        fs::remove_file(cache.join("CMakeLists.txt")).unwrap();
        ensure_checkout(&cache, &url, &first.to_string()).unwrap();
        assert_eq!(
            Repository::open(&cache).unwrap().head().unwrap().target(),
            Some(first)
        );
        assert!(cache.join("CMakeLists.txt").is_file());
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn checkout_initializes_nested_submodules_at_pinned_commits() {
        let temp = TestDir::new();
        let leaf_path = temp.path().join("leaf");
        let (_, leaf_commit) = create_source_repo(&leaf_path);
        let leaf_url = leaf_path.to_string_lossy().to_string();

        let middle_path = temp.path().join("middle");
        let (middle, _) = create_source_repo(&middle_path);
        add_submodule(&middle, &leaf_url, "external/leaf");
        let middle_commit = commit_all(&middle, "add leaf");
        let middle_url = middle_path.to_string_lossy().to_string();

        let root_path = temp.path().join("root");
        let (root, _) = create_source_repo(&root_path);
        add_submodule(&root, &middle_url, "external/middle");
        let root_commit = commit_all(&root, "add middle");
        let root_url = root_path.to_string_lossy().to_string();
        let cache = temp.path().join("cache/source");

        ensure_checkout(&cache, &root_url, &root_commit.to_string()).unwrap();
        let root_cache = Repository::open(&cache).unwrap();
        let middle_submodules = root_cache.submodules().unwrap();
        let middle_cache = middle_submodules[0].open().unwrap();
        assert_eq!(middle_cache.head().unwrap().target(), Some(middle_commit));
        let leaf_submodules = middle_cache.submodules().unwrap();
        let leaf_cache = leaf_submodules[0].open().unwrap();
        assert_eq!(leaf_cache.head().unwrap().target(), Some(leaf_commit));
        drop(leaf_cache);
        drop(leaf_submodules);
        drop(middle_cache);
        drop(middle_submodules);
        drop(root_cache);

        let dirty_leaf = cache.join("external/middle/external/leaf/local-edit.txt");
        fs::write(&dirty_leaf, "dirty").unwrap();
        ensure_checkout(&cache, &root_url, &root_commit.to_string()).unwrap();
        assert!(!dirty_leaf.exists());
        assert!(cache_is_valid(&cache, &root_url, &root_commit.to_string()));
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    #[ignore = "clones SDL_ttf and its submodules from GitHub, requires network access"]
    fn checkout_real_remote_initializes_submodules_and_reuses_cache() {
        let temp = TestDir::new();
        let cache = temp.path().join("cache/source");
        let url = "https://github.com/libsdl-org/SDL_ttf";
        let revision = "a1ce3670aec736ecbf0936c43f2f0cc53aa61e5b";

        ensure_checkout(&cache, url, revision).expect("real remote checkout failed");
        let repo = Repository::open(&cache).unwrap();
        assert!(repo.head_detached().unwrap());
        assert_eq!(
            repo.head().unwrap().target(),
            Some(git2::Oid::from_str(revision).unwrap())
        );
        assert!(!repo.submodules().unwrap().is_empty());
        assert!(cache_is_valid(&cache, url, revision));
        let reuse_probe = repo.path().join("sdl-network-reuse-probe");
        fs::write(&reuse_probe, "preserved only when the checkout is reused").unwrap();
        drop(repo);

        ensure_checkout(&cache, url, revision).expect("cached remote checkout failed");
        assert!(
            reuse_probe.is_file(),
            "valid checkout was unnecessarily replaced"
        );
        assert!(cache_is_valid(&cache, url, revision));
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn static_source_metadata_resolves_private_dependencies_in_local_prefix() {
        let temp = TestDir::new();
        let prefix = temp.path().join("install with spaces");
        let pc_dir = prefix.join("lib/pkgconfig");
        fs::create_dir_all(&pc_dir).unwrap();
        fs::write(
            pc_dir.join("sdl3-fixture.pc"),
            "Name: SDL fixture\nDescription: Static source metadata fixture\nVersion: 1.0\nLibs: -lSDL3_fixture\nRequires.private: sdl-test-private-dependency\n",
        ).unwrap();
        let builder = super::SdlBuilder::new("SDL3_fixture", "fixture", "unused");
        assert!(matches!(
            builder.link_source_static(&prefix),
            Err(BuildError::PkgConfigError(_))
        ));
        fs::write(
            pc_dir.join("sdl-test-private-dependency.pc"),
            "Name: Private dependency\nDescription: Local private dependency fixture\nVersion: 1.0\nLibs: -lsdl_test_private_dependency\n",
        ).unwrap();
        builder.link_source_static(&prefix).unwrap();
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn checkout_uses_advisory_lock_file() {
        let temp = TestDir::new();
        let path = temp.path().join("cache.lock");
        let first = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .unwrap();
        first.lock().unwrap();
        let second = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        assert!(second.try_lock().is_err());
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn install_prefix_handles_each_cmake_layout() {
        for (dir, prefix) in [
            ("/p/lib/cmake/SDL3", "/p"),
            ("/p/lib64/cmake/SDL3", "/p"),
            ("/p/cmake", "/p"),
        ] {
            assert_eq!(install_prefix(Path::new(dir)), Some(PathBuf::from(prefix)));
        }
        assert_eq!(install_prefix(Path::new("/p/other")), None);
    }

    #[test]
    fn unsupported_targets_use_their_own_error() {
        assert!(matches!(
            validate_target("windows", "msvc"),
            Err(super::BuildError::UnsupportedTarget(_))
        ));
    }

    #[cfg(feature = "build-from-source")]
    #[test]
    fn invalid_source_overrides_are_rejected() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        for path in [
            manifest.to_path_buf(),
            manifest.join("Cargo.toml"),
            manifest.join("missing-source"),
        ] {
            assert!(matches!(
                validated_source_path(&path),
                Err(BuildError::PathNotFound(_))
            ));
        }
    }
}
