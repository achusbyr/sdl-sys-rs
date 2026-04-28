use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    /// Path to the macOS SDK (for cross-generating bindings).
    #[arg(long)]
    pub osx_sdk: Option<PathBuf>,

    /// Path to the iOS SDK (for cross-generating bindings).
    #[arg(long)]
    pub ios_sdk: Option<PathBuf>,
}
