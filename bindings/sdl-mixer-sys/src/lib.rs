//! Raw FFI bindings for SDL3_mixer.
//!
//! # Feature flags
//!
//! The linkage features (`build-from-source`, `link-static`, `use-pkg-config`,
//! `use-vcpkg`) apply to **SDL3_mixer only**. They are deliberately *not* forwarded to
//! `sdl-sys-bindgen`, so you can, for example, build SDL3_mixer from source against a
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
        MIX_Version, SDL_MIXER_MAJOR_VERSION, SDL_MIXER_MICRO_VERSION, SDL_MIXER_MINOR_VERSION,
    };

    #[test]
    fn native_library_initializes_and_quits() {
        // SAFETY: Initialization and shutdown are paired and require no audio device.
        unsafe {
            assert!(super::MIX_Init());
            super::MIX_Quit();
        }
    }

    #[test]
    fn native_library_version_matches_bindings() {
        // SAFETY: The version query takes no arguments and requires no initialization.
        let version = unsafe { MIX_Version() };
        let expected = SDL_MIXER_MAJOR_VERSION * 1_000_000
            + SDL_MIXER_MINOR_VERSION * 1_000
            + SDL_MIXER_MICRO_VERSION;
        assert_eq!(version as u32, expected);
    }
}
