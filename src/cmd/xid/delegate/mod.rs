pub mod add;
pub mod all;
pub mod at;
pub mod count;
pub mod find;
pub mod remove;
pub mod update;

use clap::{Subcommand, Args};
use anyhow::Result;

/// Work with a XID document's keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Add(add::CommandArgs),
    All(all::CommandArgs),
    At(at::CommandArgs),
    Count(count::CommandArgs),
    Find(find::CommandArgs),
    Remove(remove::CommandArgs),
    Update(update::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Add(args) => args.exec(),
            SubCommands::All(args) => args.exec(),
            SubCommands::At(args) => args.exec(),
            SubCommands::Count(args) => args.exec(),
            SubCommands::Find(args) => args.exec(),
            SubCommands::Remove(args) => args.exec(),
            SubCommands::Update(args) => args.exec(),
        }
    }
}
