pub mod removing;
pub mod revealing;
pub mod elide_args;

use clap::{Subcommand, Args};
use anyhow::Result;

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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            ElideCommands::Revealing(args) => args.exec(),
            ElideCommands::Removing(args) => args.exec(),
        }
    }
}
