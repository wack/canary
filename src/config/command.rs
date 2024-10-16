use clap::Subcommand;
use miette::Result;

use crate::cmd::Version;

/// one of the top-level commands accepted by
/// the canary CLI.
#[derive(Subcommand, Clone)]
pub enum CanaryCommand {
    /// Print the CLI version and exit
    Version,
}

impl CanaryCommand {
    /// dispatch the user-provided arguments to the command handler.
    pub async fn dispatch(&self) -> Result<()> {
        match self.clone() {
            Self::Version => Version::new().dispatch(),
        }
    }
}
