/// This build script uses `SdlBuilder` from `sdl-build-helper` to manage
/// the CMake build process and linking for SDL_mixer.
use sdl_build_helper::SdlBuilder;

fn main() {
    SdlBuilder::new("SDL3_mixer", "sdl3_mixer", "../submodules/SDL_mixer")
        .with_repo_url("https://github.com/libsdl-org/SDL_mixer")
        .requires_base_sdl3(true)
        .with_cmake_option("SDLMIXER_SAMPLES", "OFF")
        .with_cmake_option("SDLMIXER_VENDORED", "ON")
        .build();
}
