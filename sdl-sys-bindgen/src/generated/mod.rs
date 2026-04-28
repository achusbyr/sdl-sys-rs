pub mod constants;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod bindings_x86_64_unknown_linux_gnu;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use bindings_x86_64_unknown_linux_gnu::*;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod bindings_x86_64_pc_windows_gnu;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use bindings_x86_64_pc_windows_gnu::*;

#[cfg(all(target_os = "emscripten", target_arch = "wasm32"))]
mod bindings_wasm32_unknown_emscripten;
#[cfg(all(target_os = "emscripten", target_arch = "wasm32"))]
pub use bindings_wasm32_unknown_emscripten::*;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod bindings_aarch64_apple_darwin;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use bindings_aarch64_apple_darwin::*;

#[cfg(all(target_os = "ios", target_arch = "aarch64"))]
mod bindings_aarch64_apple_ios;
#[cfg(all(target_os = "ios", target_arch = "aarch64"))]
pub use bindings_aarch64_apple_ios::*;

#[cfg(not(any(all(target_os = "linux", target_arch = "x86_64"), all(target_os = "windows", target_arch = "x86_64"), all(target_os = "emscripten", target_arch = "wasm32"), all(target_os = "macos", target_arch = "aarch64"), all(target_os = "ios", target_arch = "aarch64"))))]
compile_error!("Unsupported target: run sdl-bindgen to generate bindings for this platform");
