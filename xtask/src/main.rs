mod bindgen;
mod cli;

use crate::bindgen::initiate;
use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    initiate::begin(TARGETS, cli.osx_sdk, cli.ios_sdk);
}

pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "wasm32-unknown-emscripten",
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
];
