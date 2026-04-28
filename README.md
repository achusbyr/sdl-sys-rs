# sdl-sys-rs

[![License: Zlib](https://img.shields.io/badge/License-Zlib-blue.svg)](https://opensource.org/licenses/Zlib)
[![Rust: 1.80+](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org)

Low-level Rust FFI bindings for **SDL3** and its satellite libraries, featuring a robust build system and an idiomatic application wrapper.

## Why this project?

- **Pre-generated Bindings:** No need for LLVM/Clang during a standard build. Bindings for common targets (Linux, Windows, macOS, iOS, Emscripten) are bundled.
- **Typestate Build System:** A safe, compile-time checked builder for handling complex C library linkage and source builds.
- **Main Loop Wrapper:** An idiomatic, `no_std`-friendly trait-based wrapper for SDL3's callback-based application architecture.
- **Cross-Platform:** Built-in support for cross-compiling to WASM (Emscripten) and mobile platforms.

## Workspace Crates

| Crate | Purpose |
|---|---|
| [`sdl-sys-bindgen`](./sdl-sys-bindgen) | Core FFI bindings for SDL3. |
| [`sdl-image-sys`](./sdl-image-sys) | Bindings for SDL3_image. |
| [`sdl-mixer-sys`](./sdl-mixer-sys) | Bindings for SDL3_mixer. |
| [`sdl-ttf-sys`](./sdl-ttf-sys) | Bindings for SDL3_ttf. |
| [`sdl-main-wrapper`](./sdl-main-wrapper) | High-level `SdlApp` trait and `run_app` infrastructure. |
| `sdl-build-helper` | Internal Typestate-based build logic (shared by `-sys` crates). |
| `xtask` | Developer tool for regenerating bindings. |

---

## Quick Start

### 1. Simple Application Loop

The `sdl-main-wrapper` crate allows you to write SDL3 applications using a safe, callback-based trait.

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

---

## Configuration Overrides

When using `build-from-source`, you can fine-tune the build via environment variables:

| Variable | Description |
|---|---|
| `SDL3_SOURCE_OVERRIDE` | Path to a local SDL3 source directory (skips Git clone). |
| `SDL3_REPOSITORY_OVERRIDE` | Custom Git URL for the SDL3 repository. |
| `SDL3_BRANCH_OVERRIDE` | Custom branch, tag, or commit hash to checkout. |
| `SDL3_CMAKE_OVERRIDE` | Extra flags passed to CMake (e.g., `-DSDL_WAYLAND=OFF`). |

*(Replace `SDL3` with `SDL3_IMAGE`, `SDL3_MIXER`, or `SDL3_TTF` for satellite libraries.)*

---

## Advanced: Regenerating Bindings

Bindings are pre-generated for most platforms. If you need to support a new platform or a custom SDL version:

1. Clone with submodules: `git clone --recurse-submodules https://github.com/achusbyr/sdl-sys-rs.git`
2. Ensure `bindgen` prerequisites (Clang/LLVM) are installed.
3. Run the generator:
   ```bash
   cargo xtask
   ```
4. To add a new target, modify `xtask/src/main.rs` and update the `TARGETS` array.

---

## Platform-Specific Notes

### WASM / Emscripten

1. Install and setup `emsdk`.
2. Build with the Emscripten target:
   ```bash
   cargo build --target wasm32-unknown-emscripten
   ```
3. Ensure your `SDL_main` is properly exported, you may have to manually expose a `SDL_main` function.

### Apple (iOS / macOS)

Use the `--osx-sdk` or `--ios-sdk` flags with `cargo xtask` if cross-generating.

## License

This project is licensed under the Zlib License.
