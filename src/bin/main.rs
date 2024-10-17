use clap::{CommandFactory, Parser};
use miette::Result;

use canary::Flags;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the args provided to this process, including
    // commands and flags.
    let flags = Flags::parse();
    // Execute whichever command was requested.
    dispatch_command(flags).await
}

/// This function inspects the command that was provided and
/// delegates to its entrypoint.
async fn dispatch_command(flags: Flags) -> Result<()> {
    match flags.cmd() {
        // No command was provided.
        None => empty_command(),
        // One or more flags were
        // TODO: Re-enable
        Some(cmd) => cmd.dispatch().await,
    }
}

/// When the CLI is run without any commands, we print
/// the help text and exit successfully.
fn empty_command() -> Result<()> {
    Flags::command()
        .print_long_help()
        .expect("unable to print help message");
    Ok(())
}
