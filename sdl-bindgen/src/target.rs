use std::str::FromStr;
use target_lexicon::Triple;

/// Convert a target triple (e.g. `x86_64-unknown-linux-gnu`) into `(os, arch)` cfg strings.
pub fn parse_target_triple(target: &str) -> (String, String) {
    let triple = Triple::from_str(target).unwrap_or_else(|e| {
        panic!("Failed to parse target triple '{}': {}", target, e);
    });

    let arch = match triple.architecture {
        target_lexicon::Architecture::X86_64 | target_lexicon::Architecture::X86_64h => "x86_64",
        target_lexicon::Architecture::Aarch64(_) => "aarch64",
        target_lexicon::Architecture::X86_32(_) => "x86",
        target_lexicon::Architecture::Arm(_) => "arm",
        target_lexicon::Architecture::Riscv32(_) => "riscv32",
        target_lexicon::Architecture::Riscv64(_) => "riscv64",
        target_lexicon::Architecture::Wasm32 => "wasm32",
        target_lexicon::Architecture::Wasm64 => "wasm64",
        target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsel) => {
            "mipsel"
        }
        target_lexicon::Architecture::Mips32(_) => "mips",
        _ => {
            eprintln!(
                "WARNING: Unrecognized architecture in target triple '{}'",
                target
            );
            "unknown"
        }
    };

    let os = match (triple.operating_system, triple.environment) {
        (target_lexicon::OperatingSystem::Windows, _) => "windows",
        (
            target_lexicon::OperatingSystem::Linux,
            target_lexicon::Environment::Android | target_lexicon::Environment::Androideabi,
        ) => "android",
        (target_lexicon::OperatingSystem::Linux, _) => "linux",
        (
            target_lexicon::OperatingSystem::Darwin(_) | target_lexicon::OperatingSystem::MacOSX(_),
            _,
        ) => "macos",
        (target_lexicon::OperatingSystem::IOS(_), _) => "ios",
        (target_lexicon::OperatingSystem::TvOS(_), _) => "tvos",
        (target_lexicon::OperatingSystem::WatchOS(_), _) => "watchos",
        (
            target_lexicon::OperatingSystem::VisionOS(_) | target_lexicon::OperatingSystem::XROS(_),
            _,
        ) => "visionos",
        (target_lexicon::OperatingSystem::Horizon, _) => "horizon",
        (target_lexicon::OperatingSystem::Psp, _) => "psp",
        (target_lexicon::OperatingSystem::Emscripten, _) => "emscripten",
        (target_lexicon::OperatingSystem::Haiku, _) => "haiku",
        (target_lexicon::OperatingSystem::Freebsd, _) => "freebsd",
        (target_lexicon::OperatingSystem::Openbsd, _) => "openbsd",
        (target_lexicon::OperatingSystem::Netbsd, _) => "netbsd",
        (target_lexicon::OperatingSystem::Dragonfly, _) => "dragonfly",
        (target_lexicon::OperatingSystem::Solaris, _) => "solaris",
        (target_lexicon::OperatingSystem::Unknown, _) => "unknown",
        _ => {
            eprintln!("WARNING: Unrecognized OS in target triple '{}'", target);
            "unknown"
        }
    };

    (os.to_string(), arch.to_string())
}
