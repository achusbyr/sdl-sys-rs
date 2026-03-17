mod bindgen;
mod cli;

use crate::bindgen::initiate;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    initiate::begin(TARGETS, cli.read_sdkroot_env);
    Ok(())
}

pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
    "wasm32-unknown-emscripten",
];
