/// This build script uses `SdlBuilder` from `sdl-build-helper` to manage
/// the CMake build process and linking for SDL3_gfx.
use sdl_build_helper::SdlBuilder;

fn main() {
    SdlBuilder::new("SDL3_gfx", "sdl3_gfx", "../submodules/SDL3_gfx")
        .with_repo_url("https://github.com/sabdul-khabir/SDL3_gfx")
        .requires_base_sdl3(true)
        .with_cmake_option("SDL3_GFX_BUILD_TESTS", "OFF")
        .with_cmake_option("SDL3_GFX_BUILD_EXAMPLES", "OFF")
        .with_cmake_option("SDL3_GFX_VENDORED", "ON")
        .build();
}
