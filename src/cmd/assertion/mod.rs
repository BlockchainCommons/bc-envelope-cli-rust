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
#[command(args_conflicts_with_subcommands = true)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,

    #[command(flatten)]
    pub default_command: add::CommandArgs,
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
        match self.command.as_ref() {
            Some(SubCommands::Add(args)) => args.exec(),
            Some(SubCommands::All(args)) => args.exec(),
            Some(SubCommands::At(args)) => args.exec(),
            Some(SubCommands::Count(args)) => args.exec(),
            Some(SubCommands::Create(args)) => args.exec(),
            Some(SubCommands::Find(args)) => args.exec(),
            Some(SubCommands::Remove(args)) => args.exec(),
            None => self.default_command.exec(),
        }
    }
}
