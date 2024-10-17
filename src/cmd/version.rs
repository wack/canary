use miette::Result;

/// This is the version of the canary CLI, pulled from Cargo.toml.
pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Print the CLI version to stdout.
#[derive(Default)]
pub struct Version;

impl Version {
    pub fn new() -> Self {
        Self::default()
    }

    /// Print the version and exit.
    pub fn dispatch(self) -> Result<()> {
        // TODO: Reincorporate the "Terminal" abstraction to
        //       mediate writing to stdout from one spot.
        println!("{CLI_VERSION}");
        Ok(())
    }
}
