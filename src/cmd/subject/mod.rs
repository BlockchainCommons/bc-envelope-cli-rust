use clap::{Subcommand, Args};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubjectCommands>,
}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Single,
    Assertion,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
