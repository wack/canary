#![allow(dead_code)]

pub use config::Flags;

/// An adapter connects to some observable resource (like CloudWatch) and
/// emits events, like failed and succeeded requests.
mod adapter;
/// Contains the dispatch logic for running individual CLI subcommands.
/// The CLI's main function calls into these entrypoints for each subcommand.
mod cmd;
/// configuration of the CLI, either from the environment of flags.
mod config;
/// This is the data pipeline responsible for the control flow
/// of data from observers into number crunchers.
mod pipeline;
