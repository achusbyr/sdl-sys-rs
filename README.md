# sdl-sys-rs

Rust FFI bindings for **SDL3** and its satellite libraries (SDL3_image, SDL3_mixer, SDL3_ttf), generated with [bindgen](https://github.com/rust-lang/rust-bindgen).

## Workspace Crates

| Crate | Purpose |
|---|---|
| `sdl-sys-bindgen` | Raw FFI bindings for SDL3 |
| `sdl-image-sys` | Raw FFI bindings for SDL3_image |
| `sdl-mixer-sys` | Raw FFI bindings for SDL3_mixer |
| `sdl-ttf-sys` | Raw FFI bindings for SDL3_ttf |
| `sdl3-gfx-sys` | Raw FFI bindings for SDL3_gfx |
| `sdl-main-wrapper` | Idiomatic Rust wrapper for SDL's callback-based app loop |
| `sdl-bindgen` | Binding generator (developer tool, not published) |
| `sdl-build-helper` | Shared build script logic for `-sys` crates |

## Cargo Features

The `-sys` crates (`sdl-sys-bindgen`, `sdl-image-sys`, `sdl-mixer-sys`, `sdl-ttf-sys`, `sdl3-gfx-sys`) share a common set of Cargo features to control how the C libraries are found and linked:

- **`build-from-source`**: Downloads and builds the C library from source using CMake and Git.
  - You can override the default repository and branch using environment variables:
    - `<LIB_NAME_CAPITALIZED>_REPOSITORY_OVERRIDE` (e.g., `SDL3_REPOSITORY_OVERRIDE`) overrides the Git repository URL.
    - `<LIB_NAME_CAPITALIZED>_BRANCH_OVERRIDE` (e.g., `SDL3_BRANCH_OVERRIDE`) overrides the Git branch or commit to checkout.
- **`link-static`**: Links the C library statically.
- **`use-pkg-config`**: Uses `pkg-config` to discover the library on the system.
- **`use-vcpkg`**: Uses `vcpkg` to discover the library.

If no features are enabled, the build script attempts a default discovery for a dynamically linked system installation of the library.

The `sdl-main-wrapper` crate has the following features:
- **`std`** (enabled by default): Enables standard library support.
- **`alloc`**: Enables `alloc` support without `std`.

## Getting Started

### Prerequisites

- **Rust** (edition 2024)
- **SDL3** shared libraries installed system-wide (or reachable via `LD_LIBRARY_PATH`)
- **Clang** (required by bindgen)

### Clone with Submodules

```sh
git clone --recurse-submodules https://github.com/achusbyr/sdl-sys-rs.git
cd sdl-sys-rs
```

If you already cloned without submodules:

```sh
git submodule update --init --recursive
```

### Regenerate Bindings (optional)

> [!Warning]
> When cross compiling to Apple platforms, you will need to get the appropriate SDK and set the `SDKROOT` environment variable to point to it.
> Additionally, when generating bindings for emscripten, make sure you have emsdk installed and the `EMSDK` environment variable is set.

Only needed if you update the SDL submodules or want to add a new target:

```sh
cargo run -p sdl-bindgen
```

This reads the C headers from `submodules/` (or clones the repo if `build-from-source` is enabled) and writes generated Rust files into each crate's `src/generated/` directory.

### Build

```sh
cargo build
```

## Adding a New Target

1. Add the target triple to `TARGETS` in `sdl-bindgen/src/main.rs`
2. Run `cargo run -p sdl-bindgen`
3. The new bindings will be generated with appropriate `#[cfg]` attributes

## Emscripten Notes

Prefer using main callbacks (e.g., `sdl-main-wrapper`) over a blocking loop. Also, Emscripten will link to SDL_main rather than Rust's `fn main()`, so whatever you do in your `main()` function, it should look similar to:
```rust
#![no_main]

// Implementation...

#[no_mangle]
pub unsafe extern "C" fn SDL_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
  run_app::<MyApp>()
}
```

### Compiling & Linking

For both compiling and linking, make sure the `EMSDK` environment variable is set to the path of your emsdk installation.
You have two choices for compiling:
1. Use `RUSTFLAGS="-C link-arg=-sUSE_SDL=3" cargo build --target wasm32-unknown-emscripten`. This uses Emscripten's port of SDL3.
2. Build SDL3 from source. After enabling the `build-from-source` feature, you must ensure host tools compile for your host platform while the target compiles using `emcc`:
```sh
export CMAKE_TOOLCHAIN_FILE="$EMSDK/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake"
export CC_wasm32_unknown_emscripten="$EMSDK/upstream/emscripten/emcc"
export CXX_wasm32_unknown_emscripten="$EMSDK/upstream/emscripten/em++"
export AR_wasm32_unknown_emscripten="$EMSDK/upstream/emscripten/emar"
```
Finally, build with:
```sh
cargo build --target wasm32-unknown-emscripten
```
`*.wasm` and `*.js` files will be located in the `target/wasm32-unknown-emscripten/` directory.