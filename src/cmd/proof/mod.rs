pub mod confirm;
pub mod create;
pub use anyhow::Result;
use clap::{Args, Subcommand};

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

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Create(args) => args.exec(),
            SubCommands::Confirm(args) => args.exec(),
        }
    }
}
