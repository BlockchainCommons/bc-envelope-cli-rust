pub mod single;
pub mod assertion;

use clap::{Subcommand, Args};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubjectCommands>,

    #[command(flatten)]
    pub default_command: single::CommandArgs,}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Single(single::CommandArgs),
    Assertion(assertion::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        match self.command.as_ref() {
            Some(SubjectCommands::Single(args)) => args.exec(),
            Some(SubjectCommands::Assertion(args)) => args.exec(),
            None => self.default_command.exec(),
        }
    }
}
