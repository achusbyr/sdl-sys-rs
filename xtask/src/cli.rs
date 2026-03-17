use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Individually set SDKROOT for Apple platforms. This is for when generating bindings for Apple platforms from non-Apple platforms, which relies on setting the SDKROOT environment variable to a SDK. However, this means both macOS and iOS targets will look for their SDK in the single SDKROOT variable. Using this will let the generator set SDKROOT before generating bindings.
    /// Currently supports: macOS, iOS.
    /// The appropriate environment variables must be set: OSX_SDKROOT, IOS_SDKROOT.
    #[arg(short, long)]
    pub read_sdkroot_env: bool,
}
