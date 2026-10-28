use std::str::FromStr;
use target_lexicon::Triple;

/// Build the cfg predicate used to select bindings for a target.
///
/// Windows targets are matched on the GNU environment, which covers both the
/// MinGW (`gnu`) and LLVM (`gnullvm`) toolchains that share the Windows GNU ABI.
pub fn binding_cfg(target: &str) -> String {
    let (os, arch) = parse_target_triple(target);
    if os == "windows" {
        format!("all(target_os = \"{os}\", target_arch = \"{arch}\", target_env = \"gnu\")")
    } else {
        format!("all(target_os = \"{os}\", target_arch = \"{arch}\")")
    }
}

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

#[cfg(test)]
mod tests {
    use super::binding_cfg;

    #[test]
    fn windows_bindings_require_gnu_environment() {
        assert_eq!(
            binding_cfg("x86_64-pc-windows-gnu"),
            "all(target_os = \"windows\", target_arch = \"x86_64\", target_env = \"gnu\")"
        );
    }

    #[test]
    fn windows_gnu_and_gnullvm_share_a_predicate() {
        // Both toolchains report target_env = "gnu", so one predicate selects
        // the same generated module for each.
        let gnu = binding_cfg("x86_64-pc-windows-gnu");
        let gnullvm = binding_cfg("x86_64-pc-windows-gnullvm");
        assert_eq!(gnu, gnullvm);
        assert_eq!(
            gnu,
            "all(target_os = \"windows\", target_arch = \"x86_64\", target_env = \"gnu\")"
        );
    }

    #[test]
    fn linux_selection_is_unchanged() {
        assert_eq!(
            binding_cfg("x86_64-unknown-linux-gnu"),
            "all(target_os = \"linux\", target_arch = \"x86_64\")"
        );
    }
}
