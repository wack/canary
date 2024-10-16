use clap::{command, Parser};

use super::colors::EnableColors;
use super::command::CanaryCommand;

/// Canary is tool to manage self-promoting deployments.
#[derive(Parser)]
pub struct Flags {
    /// The subcommand to execute
    #[command(subcommand)]
    cmd: Option<CanaryCommand>,

    /// Whether to color the output
    #[arg(long, value_enum, default_value_t=EnableColors::default())]
    enable_colors: EnableColors,
}

impl Flags {
    /// Return the top-level command provided, if it exists.
    pub fn cmd(&self) -> Option<&CanaryCommand> {
        self.cmd.as_ref()
    }

    /// Getter that returns the user-provided preference
    /// for using color codes in the terminal output.
    pub fn enable_colors(&self) -> EnableColors {
        self.enable_colors
    }
}
