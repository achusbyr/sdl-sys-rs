/// This build script uses `SdlBuilder` from `sdl-build-helper` to manage
/// the CMake build process and linking for SDL_ttf.
use sdl_build_helper::SdlBuilder;

fn main() {
    SdlBuilder::new("SDL3_ttf", "sdl3_ttf", "../submodules/SDL_ttf")
        .with_repo_url("https://github.com/libsdl-org/SDL_ttf")
        .requires_base_sdl(true)
        .with_cmake_option("SDLTTF_SAMPLES", "OFF")
        .build();
}
