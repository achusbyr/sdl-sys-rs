# sdl-sys-rs

[![License: Zlib](https://img.shields.io/badge/License-Zlib-blue.svg)](https://opensource.org/licenses/Zlib)
[![Rust: 1.89+](https://img.shields.io/badge/Rust-1.89+-orange.svg)](https://www.rust-lang.org)

Low-level Rust FFI bindings for **SDL3** and its satellite libraries, featuring a robust build system and an idiomatic application wrapper.

## Features

- **Pre-generated Bindings:** LLVM/Clang not required during a standard build. Bindings for common targets (Linux, Windows GNU, macOS, iOS, Emscripten) are bundled.
- **Build Helper:** Shared builder (`sdl-build-helper`) for handling C library linkage and source builds.
- **Main Loop Wrapper:** `no_std`-friendly trait-based wrapper for SDL3's callback-based application architecture.
- **Cross-Platform:** Built-in support for cross-compiling to WASM (Emscripten) and mobile platforms.

## Workspace Crates

| Crate | Purpose |
| --- | --- |
| [`sdl-sys-bindgen`](./bindings/sdl-sys-bindgen) | Core FFI bindings for SDL3. |
| [`sdl-image-sys`](./bindings/sdl-image-sys) | Bindings for SDL3_image. |
| [`sdl-mixer-sys`](./bindings/sdl-mixer-sys) | Bindings for SDL3_mixer. |
| [`sdl-ttf-sys`](./bindings/sdl-ttf-sys) | Bindings for SDL3_ttf. |
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

    fn init() -> Result<Self, Self::Error> {
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

fn main() {
    let code = run_app::<MyApp>();
    // On Emscripten `run_app` returns right after scheduling the callbacks,
    // so exiting here would tear the runtime down before the app ever runs.
    #[cfg(not(target_os = "emscripten"))]
    std::process::exit(code);
}
```

A panic inside any `SdlApp` method aborts the process (the callbacks cross an
`extern "C"` boundary), so report failures through `SDL_AppResult` instead.

The wrapper remains `no_std`-compatible, and its optional `alloc` feature enables
detailed initialization error messages.

### 2. Linking Strategies

The `-sys` crates provide several features to control linkage:

- **`build-from-source`**: Builds the selected crate's SDL library via CMake, using local sources or cloning them when needed. Requires `cmake`.
- **`link-static`**: Forces static linkage of the selected SDL library. System dependencies may remain dynamically linked.
- **`use-pkg-config`**: Uses `pkg-config` to find system libraries.
- **`use-vcpkg`**: Uses `vcpkg` to find system libraries.

If no features are selected, the build script defaults to dynamic linkage against system-installed libraries.

Source-built satellite libraries (SDL3_image, SDL3_mixer, and SDL3_ttf) can use a
system-installed SDL3, provided its CMake package configuration is discoverable.
For a nonstandard installation, set `SDL3_DIR` to the directory containing
`SDL3Config.cmake` or add the installation prefix to `CMAKE_PREFIX_PATH`.

> [!Important]
> **Satellite features do not propagate to the core crate (by design).**
> Enabling `build-from-source`, `link-static`, `use-pkg-config` or `use-vcpkg` on
> `sdl-image-sys`, `sdl-mixer-sys` or `sdl-ttf-sys` only affects *that* library.
> This lets you, for example, link a system-installed SDL3 while building SDL3_image
> from source. If you want SDL3 built from source or linked statically as well,
> enable the same feature on `sdl-sys-bindgen` explicitly. Every library
> must still be ABI-compatible with the SDL3 you end up linking.

If `sdl-sys-bindgen` uses the `build-from-source` feature, the build helper passes the source-built SDL3 CMake package
location to the satellites automatically.

## Configuration Overrides

When using `build-from-source`, you can fine-tune the build via environment variables:

| Variable | Description |
| --- | --- |
| `(LIB)_SOURCE_OVERRIDE` | Path to a local SDL3 source directory (skips Git clone). |
| `(LIB)_REPOSITORY_OVERRIDE` | Custom Git URL for the SDL3 repository. |
| `(LIB)_BRANCH_OVERRIDE` | Custom branch, tag, or commit hash to checkout. Prefer a commit hash: a branch or tag is resolved once and the cached clone is **not** re-fetched, so moving branches will not update. Clear the build's `OUT_DIR` cache (e.g. `cargo clean`) to force a fresh clone. |
| `(LIB)_CMAKE_OVERRIDE` | Space-separated `KEY=VALUE` or `-DKEY=VALUE` flags passed to CMake (e.g., `-DSDL_WAYLAND=OFF`). Values cannot contain spaces. Malformed entries are ignored with a build warning. |

*(Replace `(LIB)` with `SDL3`, `SDL3_IMAGE`, `SDL3_MIXER`, or `SDL3_TTF`.)*

## Platform-Specific Notes

> [!Note]
> **Macros:**
> The generated bindings include plain numeric `#define` constants, SDL's hint and
> property names, and whatever bindgen can evaluate. Function-like macros (for
> example `SDL_BUTTON_MASK`, and constants built from them such as
> `SDL_BUTTON_LMASK`) are not translated. You'll have to define them in your own code.

### Linux

The Linux bindings are generated for `x86_64` and selected by `target_os` and
`target_arch` only, so `*-linux-musl` targets use the same bindings as glibc.
SDL's public API is the same, but the bindings are only generated and tested
against `x86_64-unknown-linux-gnu`.

### Windows (GNU / MinGW-w64)

> [!Warning]
> MSVC targets (`*-pc-windows-msvc`) are not supported.

Windows support requires a **GNU-environment** target. Both `x86_64-pc-windows-gnu`
(MinGW-w64 GCC) and `x86_64-pc-windows-gnullvm` (LLVM/LLD) are supported and share
the same generated bindings and inline wrappers.

1. Install the target toolchain: `rustup toolchain install stable-x86_64-pc-windows-gnu`
   (or `stable-x86_64-pc-windows-gnullvm`).
2. Install a matching 64-bit toolchain and put its compiler, linker, and build
   tools on `PATH`. For the GNU target use MinGW-w64 GCC (for example, through
   MSYS2's MinGW-w64 environment), for gnullvm use the [llvm-mingw](https://github.com/mstorsjo/llvm-mingw) toolchain.
   A C compiler is needed even with pre-generated bindings because the build script
   compiles wrappers for SDL's header-only inline functions.
3. Build with `cargo +stable-x86_64-pc-windows-gnu build --target x86_64-pc-windows-gnu`
   (or the `gnullvm` equivalents).

For system linkage, install GNU-compatible SDL3 and satellite libraries and
make their library directories discoverable to the linker. For shared linkage,
make the required DLLs discoverable at runtime, for example beside the executable
or on `PATH`.

For `build-from-source`, also install CMake and a compatible build tool, such as
Ninja or MinGW Make, and the development dependencies required by the enabled
backends. Static source builds additionally require pkgconf (see below).
Clang/libclang and MinGW-w64 headers are needed only when regenerating Windows
bindings with `cargo xtask`, not when using the bundled bindings.

#### Cross-compiling satellites

SDL3_image and SDL3_mixer locate optional dependencies with `find_package` and
pkg-config. When cross-compiling from a Linux host, these lookups will
find the *host's* headers (for example `/usr/include/webp` and
`/usr/include/opus`), which then conflict with the Windows target's headers.
Limit CMake's search to the target sysroot in the satellite's override, in
addition to the toolchain arguments above:

```sh
export SDL3_IMAGE_CMAKE_OVERRIDE="$SDL3_CMAKE_OVERRIDE \
  -DCMAKE_FIND_ROOT_PATH=/opt/llvm-mingw/x86_64-w64-mingw32 \
  -DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER \
  -DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY \
  -DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY \
  -DCMAKE_FIND_ROOT_PATH_MODE_PACKAGE=ONLY"
```

SDL3_ttf additionally requires FreeType, which llvm-mingw does not include. Provide
a cross-compiled FreeType or disable that backend.

#### gnullvm details

The `x86_64-pc-windows-gnullvm` target expects a compiler driver named
`x86_64-w64-mingw32-clang` and an LLVM-based MinGW-w64 sysroot that provides
`libunwind`. The easiest source of both is the [llvm-mingw](https://github.com/mstorsjo/llvm-mingw)
toolchain, which is self-contained and targets `x86_64`, `i686`, `armv7`, and
`aarch64`. Put its `bin` directory on `PATH` and it works, needing no wrapper
scripts and no extra linker flags are required:

```sh
export PATH="/path/to/llvm-mingw/bin:$PATH"
```

<details>
<summary>Using GCC-based MinGW-w64 sysroot instead</summary>

A GCC-based MinGW-w64 sysroot is not enough on its own: it ships no
`libunwind`, which the Rust standard library for this target requires. If you
must use one, create an `x86_64-w64-mingw32-clang` wrapper that runs
`clang --target=x86_64-w64-windows-gnu -fuse-ld=lld` and make `libunwind.a`
(for example, a copy of the sysroot's `libgcc_eh.a`) discoverable on its search
path.

The resulting binaries use the UCRT and dynamically link `libunwind.dll`, so
ship `libunwind.dll` (from the toolchain's `bin` directory) beside the
executable. For a `build-from-source` cross build, point CMake at the same
toolchain via `SDL3_CMAKE_OVERRIDE` (the satellite crates accept the analogous
`SDL3_IMAGE_CMAKE_OVERRIDE`, etc.):

```sh
export SDL3_CMAKE_OVERRIDE="-DCMAKE_SYSTEM_NAME=Windows \
  -DCMAKE_C_COMPILER=x86_64-w64-mingw32-clang \
  -DCMAKE_CXX_COMPILER=x86_64-w64-mingw32-clang++ \
  -DCMAKE_RC_COMPILER=x86_64-w64-mingw32-windres"
```

</details>

### WASM / Emscripten

1. Install and setup `emsdk` (`git clone https://github.com/emscripten-core/emsdk.git`)
2. Make sure the environment is prepared (`emsdk install/activate`, `source`, etc.) and build with the Emscripten target: `cargo build --target wasm32-unknown-emscripten`

You may have to manually expose a `SDL_main` function. For cross-generating bindings, run `cargo xtask` instead.

### Apple (iOS / macOS)

Use the `--osx-sdk` and/or `--ios-sdk` flags with `cargo xtask` if cross-generating.

## Development

Make sure Clang is installed for bindgen. It is also the compiler for the
`gnullvm` Windows target.

### Updating Bindings

The process for updating bindings is as follows:

1. Find the commit marking the latest release (e.g., [SDL 3.4.16](https://github.com/libsdl-org/SDL/releases/tag/release-3.4.16), in the top right corner, is commit [fa2c02b](https://github.com/libsdl-org/SDL/commit/fa2c02bb6e21974a89ea9824bc53c9932abe5f9c))
2. Head to the corresponding bindings crate and bump the version (`0.1.5` -> `0.1.6`), the SDL version (`3.4.14` -> `3.4.16`), and the commit (`147a8ee...` -> `fa2c02b...`). Note that the commit must be the full hash.
3. Update the submodule in [`submodules`](./submodules) to the correct tag/release.
4. If you want to generate bindings for every target, [prepare your environment for cross compilation beforehand.](#platform-specific-notes)
5. Run: `cargo xtask`

### Static Source Builds

On supported targets, `build-from-source` combined with `link-static` uses the
installed `.pc` metadata to propagate transitive native dependencies (for example,
FreeType and HarfBuzz for SDL3_ttf). This requires **pkgconf**, available as
`pkg-config` or `pkgconf`, with `--with-path` support. Source-built SDL archives
are linked statically, external system dependencies may still be shared.
Shared source builds do not require this extra discovery step.

For cross-compilation, configure the target-specific `PKG_CONFIG`,
`PKG_CONFIG_LIBDIR`, and `PKG_CONFIG_SYSROOT_DIR` as appropriate.

### Verifying Remote Source Checkout

Local checkout/cache tests run with:

```bash
cargo test -p sdl-build-helper --features build-from-source
```

An opt-in network test clones the pinned SDL3_ttf release from GitHub, initializes
its submodules recursively, verifies the detached commit and cache validity,
and checks cache reuse. It downloads repository histories into a temporary
directory, which is removed after the test, and it does not run CMake.

```bash
cargo test -p sdl-build-helper --features build-from-source \
  checkout_real_remote_initializes_submodules_and_reuses_cache -- --ignored --nocapture
```

### Adding Bindings

#### Adding A New Platform

Bindings are pre-generated for most platforms. If you need to support a new platform:

1. Clone the repository with submodules: `git clone --recurse-submodules https://github.com/achusbyr/sdl-sys-rs.git`
2. To add a new target, update the `TARGETS` array in `xtask/src/main.rs`.
3. Run the generator: `cargo xtask`

## License

This project is licensed under the Zlib License.
