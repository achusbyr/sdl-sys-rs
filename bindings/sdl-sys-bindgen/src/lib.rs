//! Raw FFI bindings for SDL3.
//!
//! # Feature flags
//!
//! The linkage features (`build-from-source`, `link-static`, `use-pkg-config`,
//! `use-vcpkg`) apply to SDL3 only. The satellite crates (`sdl-image-sys`,
//! `sdl-mixer-sys`, `sdl-ttf-sys`) have their own, independent features; enabling
//! one of them does not change how SDL3 is built or linked.

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

#[cfg(test)]
mod tests {
    use super::{SDL_MOUSE_TOUCHID, SDL_PEN_TOUCHID, SDL_TouchID};

    #[cfg(feature = "build-from-source")]
    #[test]
    fn native_library_version_matches_bindings() {
        // SAFETY: The version query takes no arguments and requires no initialization.
        let version = unsafe { super::SDL_GetVersion() };
        let expected = super::SDL_MAJOR_VERSION * 1_000_000
            + super::SDL_MINOR_VERSION * 1_000
            + super::SDL_MICRO_VERSION;
        assert_eq!(version as u32, expected);
    }

    #[test]
    fn touch_sentinels_match_the_full_width_touch_id() {
        let mouse: SDL_TouchID = SDL_MOUSE_TOUCHID;
        let pen: SDL_TouchID = SDL_PEN_TOUCHID;
        assert_eq!(mouse, u64::MAX);
        assert_eq!(pen, u64::MAX - 1);
    }

    #[test]
    fn signed_64_bit_limits_are_extracted() {
        assert_eq!(super::SDL_MAX_SINT64, i64::MAX);
        assert_eq!(super::SDL_MIN_SINT64, i64::MIN);
    }
}
