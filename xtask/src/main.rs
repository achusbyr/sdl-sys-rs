mod bindgen;
mod cli;

use crate::bindgen::initiate;
use clap::Parser;

fn main() {
    initiate::begin(TARGETS, cli::Cli::parse().read_sdkroot_env);
}

pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "wasm32-unknown-emscripten",
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
];
