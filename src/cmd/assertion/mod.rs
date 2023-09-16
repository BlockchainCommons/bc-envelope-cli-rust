pub mod add;
pub mod all;
pub mod at;
pub mod count;
pub mod create;
pub mod find;
pub mod remove;

use clap::{Subcommand, Args};

/// Work with the envelope's assertions.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Add(add::CommandArgs),
    All(all::CommandArgs),
    At(at::CommandArgs),
    Count(count::CommandArgs),
    Create(create::CommandArgs),
    Find(find::CommandArgs),
    Remove(remove::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
