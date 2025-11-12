pub mod get;
pub mod next;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Work with provenance marks.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Get(get::CommandArgs),
    Next(next::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Get(args) => args.exec(),
            SubCommands::Next(args) => args.exec(),
        }
    }
}
