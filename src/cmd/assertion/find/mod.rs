pub mod predicate;
pub mod object;

use clap::{Subcommand, Args};

/// Find all assertions matching the given criteria.
#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,

    #[command(flatten)]
    pub default_command: predicate::CommandArgs,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Object(object::CommandArgs),
    Predicate(predicate::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        match self.command.as_ref() {
            Some(SubCommands::Object(args)) => args.exec(),
            Some(SubCommands::Predicate(args)) => args.exec(),
            None => self.default_command.exec(),
        }
    }
}
