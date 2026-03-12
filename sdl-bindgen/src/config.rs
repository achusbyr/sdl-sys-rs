pub struct SysCrateConfig {
    pub lib_name: &'static str,
    pub include_dirs: &'static [&'static str],
    pub headers: &'static [&'static str],
    pub out_dir: &'static str,
    pub hint_prefix: &'static str,
    pub prop_prefix: &'static str,
    pub allowlist_file: Option<&'static str>,
}

pub const CRATES: &[SysCrateConfig] = &[
    SysCrateConfig {
        lib_name: "SDL3",
        include_dirs: &["submodules/SDL/include"],
        headers: &[
            "submodules/SDL/include/SDL3/SDL.h",
            "submodules/SDL/include/SDL3/SDL_main.h",
        ],
        out_dir: "sdl-sys-bindgen/src/generated",
        hint_prefix: "SDL_HINT_",
        prop_prefix: "SDL_PROP_",
        allowlist_file: None,
    },
    SysCrateConfig {
        lib_name: "SDL3_image",
        include_dirs: &["submodules/SDL/include", "submodules/SDL_image/include"],
        headers: &["submodules/SDL_image/include/SDL3_image/SDL_image.h"],
        out_dir: "sdl-image-sys/src/generated",
        hint_prefix: "IMG_HINT_",
        prop_prefix: "IMG_PROP_",
        allowlist_file: Some(".*SDL3_image.*"),
    },
    SysCrateConfig {
        lib_name: "SDL3_mixer",
        include_dirs: &["submodules/SDL/include", "submodules/SDL_mixer/include"],
        headers: &["submodules/SDL_mixer/include/SDL3_mixer/SDL_mixer.h"],
        out_dir: "sdl-mixer-sys/src/generated",
        hint_prefix: "MIX_HINT_",
        prop_prefix: "MIX_PROP_",
        allowlist_file: Some(".*SDL3_mixer.*"),
    },
    SysCrateConfig {
        lib_name: "SDL3_ttf",
        include_dirs: &["submodules/SDL/include", "submodules/SDL_ttf/include"],
        headers: &["submodules/SDL_ttf/include/SDL3_ttf/SDL_ttf.h"],
        out_dir: "sdl-ttf-sys/src/generated",
        hint_prefix: "TTF_HINT_",
        prop_prefix: "TTF_PROP_",
        allowlist_file: Some(".*SDL3_ttf.*"),
    },
];

pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "aarch64-apple-darwin",
    "wasm32-unknown-emscripten",
];
