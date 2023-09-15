use clap::{Subcommand, Args};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
pub struct SubjectArgs {
    #[command(subcommand)]
    command: Option<SubjectCommands>,
}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Single,
    Assertion,
}

pub fn subject_command(args: &SubjectArgs) {
    todo!();
}
