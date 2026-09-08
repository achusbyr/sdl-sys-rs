# sdl-sys-rs

[![License: Zlib](https://img.shields.io/badge/License-Zlib-blue.svg)](https://opensource.org/licenses/Zlib)
[![Rust: 1.80+](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org)

Low-level Rust FFI bindings for **SDL3** and its satellite libraries, featuring a robust build system and an idiomatic application wrapper.

## Features

- **Pre-generated Bindings:** LLVM/Clang not required during a standard build. Bindings for common targets (Linux, Windows, macOS, iOS, Emscripten) are bundled.
- **Typestate Build System:** Compile-time checked builder for handling C library linkage and source builds.
- **Main Loop Wrapper:** `no_std`-friendly trait-based wrapper for SDL3's callback-based application architecture.
- **Cross-Platform:** Built-in support for cross-compiling to WASM (Emscripten) and mobile platforms.

## Workspace Crates

| Crate | Purpose |
|---|---|
| [`sdl-sys-bindgen`](./sdl-sys-bindgen) | Core FFI bindings for SDL3. |
| [`sdl-image-sys`](./sdl-image-sys) | Bindings for SDL3_image. |
| [`sdl-mixer-sys`](./sdl-mixer-sys) | Bindings for SDL3_mixer. |
| [`sdl-ttf-sys`](./sdl-ttf-sys) | Bindings for SDL3_ttf. |
| [`sdl-main-wrapper`](./sdl-main-wrapper) | High-level `SdlApp` trait and `run_app` infrastructure. |
| `sdl-build-helper` | Internal build logic (shared by `-sys` crates). |
| `xtask` | Developer tool for regenerating bindings. |

## Quick Start

### 1. Using Application Callbacks

The `sdl-main-wrapper` crate allows you to write SDL3 applications using a callback-based trait.

```rust
use sdl_main_wrapper::*;
use sdl_sys_bindgen::*;

struct MyApp;

impl SdlApp for MyApp {
    type Error = ();

    fn init(_args: &[core::ffi::CString]) -> Result<Self, Self::Error> {
        Ok(MyApp)
    }

    fn iterate(&mut self) -> SDL_AppResult {
        SDL_AppResult::SDL_APP_CONTINUE
    }

    fn event(&mut self, _event: &SDL_Event) -> SDL_AppResult {
        SDL_AppResult::SDL_APP_CONTINUE
    }

    fn quit(&mut self, _result: SDL_AppResult) {
        // Cleanup happens here
    }
}

fn main() -> i32 {
    run_app::<MyApp>()
}
```

### 2. Linking Strategies

The `-sys` crates provide several features to control linkage:

- **`build-from-source`**: Automatically clones and builds SDL3 via CMake. Requires `cmake`.
- **`link-static`**: Forces static linkage.
- **`use-pkg-config`**: Uses `pkg-config` to find system libraries.
- **`use-vcpkg`**: Uses `vcpkg` to find system libraries.

If no features are selected, the build script defaults to dynamic linkage against system-installed libraries.

## Configuration Overrides

When using `build-from-source`, you can fine-tune the build via environment variables:

| Variable | Description |
|---|---|
| `(LIB)_SOURCE_OVERRIDE` | Path to a local SDL3 source directory (skips Git clone). |
| `(LIB)_REPOSITORY_OVERRIDE` | Custom Git URL for the SDL3 repository. |
| `(LIB)_BRANCH_OVERRIDE` | Custom branch, tag, or commit hash to checkout. |
| `(LIB)_CMAKE_OVERRIDE` | Extra flags passed to CMake (e.g., `-DSDL_WAYLAND=OFF`). |

*(Replace `(LIB)` with `SDL3`, `SDL3_IMAGE`, `SDL3_MIXER`, or `SDL3_TTF`.)*

## Platform-Specific Notes

### WASM / Emscripten

1. Install and setup `emsdk` (`git clone https://github.com/emscripten-core/emsdk.git`)
2. Make sure the environment is prepared and build with the Emscripten target: `cargo build --target wasm32-unknown-emscripten`

You may have to manually expose a `SDL_main` function. For cross-generating bindings, run `cargo xtask` instead.

### Apple (iOS / macOS)

Use the `--osx-sdk` and/or `--ios-sdk` flags with `cargo xtask` if cross-generating.

## Development

Make sure Clang is installed for bindgen.

### Updating Bindings

The process for updating bindings is as follows:

1. Find the commit marking the latest release (e.g., [SDL 3.4.16](https://github.com/libsdl-org/SDL/releases/tag/release-3.4.16), in the top right corner, is commit [fa2c02b](https://github.com/libsdl-org/SDL/commit/fa2c02bb6e21974a89ea9824bc53c9932abe5f9c))
2. Head to the corresponding bindings crate and bump the version (`0.1.5` -> `0.1.6`), the SDL version (`3.4.14` -> `3.4.16`), and the commit (`147a8ee...` -> `fa2c02b...`). Note that the commit must be the full hash.
3. If you want to generate bindings for every target, [prepare your environment for cross compilation beforehand.](#platform-specific-notes)
4. Run: `cargo xtask`

### Adding Bindings

#### Adding A New Platform

Bindings are pre-generated for most platforms. If you need to support a new platform:

1. Clone with submodules: `git clone --recurse-submodules https://github.com/achusbyr/sdl-sys-rs.git`
2. To add a new target, update the `TARGETS` array in `xtask/src/main.rs`.
3. Run the generator: `cargo xtask`

## License

This project is licensed under the Zlib License.
