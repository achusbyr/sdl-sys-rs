/// This build script uses `SdlBuilder` from `sdl-build-helper` to manage
/// the CMake build process and linking for the core SDL3 library.
use sdl_build_helper::SdlBuilder;

fn main() {
    SdlBuilder::new("SDL3", "sdl3", "../submodules/SDL")
        .with_repo_url("https://github.com/libsdl-org/SDL")
        .with_cmake_option("SDL_TESTS", "OFF")
        .with_cmake_option("SDL_EXAMPLES", "OFF")
        .build()
        .expect("Failed to build SDL3");
}
