//! Raw FFI bindings for SDL3_image.
//!
//! # Feature flags
//!
//! The linkage features (`build-from-source`, `link-static`, `use-pkg-config`,
//! `use-vcpkg`) apply to **SDL3_image only**. They are deliberately *not* forwarded to
//! `sdl-sys-bindgen`, so you can, for example, build SDL3_image from source against a
//! system-installed SDL3. To also build or link SDL3 itself in a particular way,
//! enable the same feature on `sdl-sys-bindgen` explicitly.

#![no_std]

// The lint allowances are scoped to the bindgen output; hand-written code and
// tests in this crate are linted normally.
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports,
    clippy::approx_constant,
    clippy::redundant_static_lifetimes,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::missing_safety_doc
)]
mod generated;
pub use generated::*;

#[cfg(all(test, feature = "build-from-source"))]
mod tests {
    use super::{
        IMG_Version, SDL_IMAGE_MAJOR_VERSION, SDL_IMAGE_MICRO_VERSION, SDL_IMAGE_MINOR_VERSION,
    };

    #[test]
    fn native_image_loader_rejects_a_missing_file() {
        // SAFETY: The path is a valid C string. No surface is returned for this missing file.
        let surface = unsafe {
            super::IMG_Load(c"sdl-build-smoke-missing-directory/nonexistent-image.png".as_ptr())
        };
        if !surface.is_null() {
            // SAFETY: An unexpected successful load still returns an owned surface.
            unsafe { sdl_sys_bindgen::SDL_DestroySurface(surface) };
            panic!("unexpected image at the smoke-test path");
        }
    }

    #[test]
    fn native_library_version_matches_bindings() {
        // SAFETY: The version query takes no arguments and requires no initialization.
        let version = unsafe { IMG_Version() };
        let expected = SDL_IMAGE_MAJOR_VERSION * 1_000_000
            + SDL_IMAGE_MINOR_VERSION * 1_000
            + SDL_IMAGE_MICRO_VERSION;
        assert_eq!(version as u32, expected);
    }
}
