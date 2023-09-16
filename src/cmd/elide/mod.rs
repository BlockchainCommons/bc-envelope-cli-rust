pub mod removing;
pub mod revealing;

use clap::{Subcommand, Args};

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
    fn exec(&self) -> Result<String, anyhow::Error> {
        match &self.command {
            ElideCommands::Revealing(args) => args.exec(),
            ElideCommands::Removing(args) => args.exec(),
        }
    }
}
