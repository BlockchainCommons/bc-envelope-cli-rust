pub mod single;
pub mod assertion;

use clap::{Subcommand, Args};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubjectCommands,
}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Single(single::CommandArgs),
    Assertion(assertion::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            SubjectCommands::Single(args) => args.exec(),
            SubjectCommands::Assertion(args) => args.exec(),
        }
    }
}
