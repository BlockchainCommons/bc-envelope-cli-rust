pub mod create;
pub mod confirm;

use clap::{Subcommand, Args};

/// Work with inclusion proofs.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Create(create::CommandArgs),
    Confirm(confirm::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            SubCommands::Create(args) => args.exec(),
            SubCommands::Confirm(args) => args.exec(),
        }
    }
}
