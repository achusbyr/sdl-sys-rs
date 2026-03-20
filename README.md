# sdl-sys-rs

Rust FFI bindings for **SDL3** and its satellite libraries (SDL3_image, SDL3_mixer, SDL3_ttf), generated with [bindgen](https://github.com/rust-lang/rust-bindgen).

## Workspace Crates

| Crate | Purpose |
|---|---|
| `sdl-sys-bindgen` | Raw FFI bindings for SDL3 |
| `sdl-image-sys` | Raw FFI bindings for SDL3_image |
| `sdl-mixer-sys` | Raw FFI bindings for SDL3_mixer |
| `sdl-ttf-sys` | Raw FFI bindings for SDL3_ttf |
| `sdl-main-wrapper` | Idiomatic Rust wrapper for SDL's callback-based app loop |
| `sdl-bindgen` | Binding generator (developer tool, not published) |
| `sdl-build-helper` | Shared build script logic for `-sys` crates |

## Cargo Features

The `-sys` crates (`sdl-sys-bindgen`, `sdl-image-sys`, `sdl-mixer-sys`, `sdl-ttf-sys`) share a common set of Cargo features to control how the C libraries are found and linked:

- **`build-from-source`**: Downloads and builds the C library from source using CMake and Git.
  - You can override some settings using environment variables:
    - `<LIB_NAME_CAPITALIZED>_REPOSITORY_OVERRIDE` (e.g., `SDL3_REPOSITORY_OVERRIDE`) overrides the Git repository URL.
    - `<LIB_NAME_CAPITALIZED>_BRANCH_OVERRIDE` (e.g., `SDL3_BRANCH_OVERRIDE`) overrides the Git branch or commit to checkout.
    - `<LIB_NAME_CAPITALIZED>_CMAKE_OVERRIDE` (e.g., `SDL3_CMAKE_OVERRIDE`) overrides the CMake flags passed.
- **`link-static`**: Links the C library statically.
- **`use-pkg-config`**: Uses `pkg-config` to discover the library on the system.
- **`use-vcpkg`**: Uses `vcpkg` to discover the library.

If no features are enabled, the build script attempts a default discovery for a dynamically linked system installation of the library.

The `sdl-main-wrapper` crate has the following features:
- **`args`**: Enables command-line argument support, enables `std` feature.
- **`std`**: Enables standard library support, enables `alloc` feature.
- **`alloc`** (enabled by default): Enables using the `alloc` crate.

`sdl-main-wrapper` can be used without any features enabled, but you won't be able to access command line arguments via `init()` and when returning an `Err` from `init()`, the error won't be formatted.

## Getting Started

### Prerequisites

- **Rust** (edition 2024)
- **SDL** shared libraries installed system-wide (or reachable via `LD_LIBRARY_PATH`)
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
> When cross compiling to Apple platforms, you will need to get the appropriate SDK and set the `SDKROOT` environment variable to point to it. You might also want to check the `read_sdkroot_env` flag for `cargo xtask`.
> Additionally, when generating bindings for Emscripten, make sure you have emsdk installed and the `EMSDK` environment variable is set.

Only needed if you update the SDL submodules or want to add a new target:

```sh
cargo xtask
```

This reads the C headers from `submodules/` (or clones the repo if `build-from-source` is enabled) and writes generated Rust files into each crate's `src/generated/` directory.

## Adding a New Target

1. Add the target triple to `TARGETS` in `sdl-bindgen/src/main.rs`
2. Run `cargo xtask`
3. The new bindings will be generated with appropriate `#[cfg]` attributes

## Emscripten Notes

Prefer using main callbacks (e.g., `sdl-main-wrapper`) over a blocking loop. Also, Emscripten will link to SDL_main rather than Rust's `fn main()`, so whatever you would have done in your `main()` function, it should look similar to:
```rust
// Implementation...

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
  run_app::<MyApp>()
}

fn main() {
  // We can have both SDL_main() and main() to support both platforms that either use SDL_main or support main().
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

## Apple platforms

If you are building on macOS, you should hopefully not need this section as it will be automatically handled by Rust.
Note that you will need the Rust target installed for the target Apple platform. For example, `rustup target add aarch64-apple-darwin` for 64-bit ARM macOS.

### macOS

You need a SDK for generating bindings and a toolchain for compiling. You may want to check the `osxcross` project if interested in cross compiling. Once you have your SDK, set the appropriate SDK root environment variable to the path to the SDK.

### iOS

You may have to obtain a Xcode build and extract the iPhoneOS.sdk from it. You can find the SDK at `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk`. Once you have it, set the appropriate SDK root environment variable to the SDK's path.

You may encounter a symbol error when building, particularly about `__isPlatformVersionAtLeast`. A proper fix would be to compile the compiler-rt libraries for iOS and use that, but another way is to include a hack:
```h
// fix.h

#ifndef FIX_H
#define FIX_H
#include <stdint.h>
#include <sys/types.h>

static inline int32_t __isPlatformVersionAtLeast(uint32_t platform, uint32_t major, uint32_t minor, uint32_t patch) {
    return 1; // Always claim the version is new enough
}

#endif
```
However, if the application tries to use a feature that is not available on the running platform's current version, an error might occur.
This Fish script can be used as an utility to quickly get things running (make sure `fix.h` is available, along with clang, llvm, and lld):
```bash
#!/usr/bin/fish

set -x FIX_PATH $(pwd)/fix.h

set -x SDKROOT /path/to/iPhoneOS.sdk/
set -x CLANG_TARGET "arm64-apple-ios15.0"

set -x COMMON_FLAGS "--target=$CLANG_TARGET -isysroot $SDKROOT -fuse-ld=lld -miphoneos-version-min=15.0 -include $FIX_PATH"

set -x CC_aarch64_apple_ios "clang"
set -x CXX_aarch64_apple_ios "clang++"
set -x CFLAGS_aarch64_apple_ios "$COMMON_FLAGS"
set -x CXXFLAGS_aarch64_apple_ios "$COMMON_FLAGS"

set -x CMAKE_C_FLAGS "$COMMON_FLAGS"
set -x CMAKE_CXX_FLAGS "$COMMON_FLAGS"
set -x OBJCFLAGS "$COMMON_FLAGS"
set -x CMAKE_OBJC_FLAGS "$COMMON_FLAGS"

set -x CMAKE_INSTALL_NAME_TOOL (which llvm-install-name-tool)
set -x CMAKE_AR (which llvm-ar)
set -x CMAKE_RANLIB (which llvm-ranlib)
set -x CMAKE_STRIP (which llvm-strip)

set -x CARGO_TARGET_AARCH64_APPLE_IOS_LINKER "clang"
set -x CARGO_TARGET_AARCH64_APPLE_IOS_RUSTFLAGS \
  "-C link-arg=--target=$CLANG_TARGET " \
  "-C link-arg=-isysroot" "-C link-arg=$SDKROOT" \
  "-C link-arg=-L$SDKROOT/usr/lib" \
  "-C link-arg=-F$SDKROOT/System/Library/Frameworks" \
  "-C link-arg=-fuse-ld=lld" \
  "-C link-arg=-miphoneos-version-min=15.0"

cargo build --target aarch64-apple-ios
```
Alternatively, if you have access to a build of Xcode, you can extract it and go to `Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/<version>/lib/darwin/`, where there will be a `libclang_rt.ios.a`. You won't need `fix.h` for this method.
Put that file anywhere, then modify COMMON_FLAGS to point `-L` to the directory the file is in:
```bash
#!/usr/bin/fish

set -x SDKROOT /path/to/iPhoneOS.sdk/
set -x CLANG_TARGET "arm64-apple-ios15.0"

set -x COMMON_FLAGS "--target=$CLANG_TARGET -isysroot $SDKROOT -fuse-ld=lld -miphoneos-version-min=15.0 -L<path_to_directory> -lclang_rt.ios"

set -x CC_aarch64_apple_ios "clang"
set -x CXX_aarch64_apple_ios "clang++"
set -x CFLAGS_aarch64_apple_ios "$COMMON_FLAGS"
set -x CXXFLAGS_aarch64_apple_ios "$COMMON_FLAGS"

set -x CMAKE_C_FLAGS "$COMMON_FLAGS"
set -x CMAKE_CXX_FLAGS "$COMMON_FLAGS"
set -x OBJCFLAGS "$COMMON_FLAGS"
set -x CMAKE_OBJC_FLAGS "$COMMON_FLAGS"

set -x CMAKE_INSTALL_NAME_TOOL (which llvm-install-name-tool)
set -x CMAKE_AR (which llvm-ar)
set -x CMAKE_RANLIB (which llvm-ranlib)
set -x CMAKE_STRIP (which llvm-strip)

set -x CARGO_TARGET_AARCH64_APPLE_IOS_LINKER "clang"
set -x CARGO_TARGET_AARCH64_APPLE_IOS_RUSTFLAGS \
  "-C link-arg=--target=$CLANG_TARGET " \
  "-C link-arg=-isysroot" "-C link-arg=$SDKROOT" \
  "-C link-arg=-L$SDKROOT/usr/lib" \
  "-C link-arg=-F$SDKROOT/System/Library/Frameworks" \
  "-C link-arg=-fuse-ld=lld" \
  "-C link-arg=-miphoneos-version-min=15.0"

cargo build --target aarch64-apple-ios
```
