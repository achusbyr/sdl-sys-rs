/// This build script uses `SdlBuilder` from `sdl-build-helper` to manage
/// the CMake build process and linking for SDL_image, ensuring it
/// hooks into the base SDL3 library correctly.
use sdl_build_helper::SdlBuilder;

fn main() {
    SdlBuilder::new("SDL3_image", "sdl3_image", "../submodules/SDL_image")
        .with_repo_url("https://github.com/libsdl-org/SDL_image")
        .requires_base_sdl(true)
        .with_cmake_option("SDLIMAGE_SAMPLES", "OFF")
        .build()
        .expect("Failed to build SDL3_image");
}
