pub mod elide_args;
pub mod removing;
pub mod revealing;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Elide a subset of elements.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: ElideCommands,
}

#[derive(Debug, Subcommand)]
enum ElideCommands {
    Revealing(revealing::CommandArgs),
    Removing(removing::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            ElideCommands::Revealing(args) => args.exec(),
            ElideCommands::Removing(args) => args.exec(),
        }
    }
}
