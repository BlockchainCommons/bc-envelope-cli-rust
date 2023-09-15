use clap::{Subcommand, Args};

/// Elide a subset of elements.
#[derive(Debug, Args)]
pub struct ElideArgs {
    #[command(subcommand)]
    command: Option<ElideCommands>,
}

#[derive(Debug, Subcommand)]
enum ElideCommands {
    Revealing,
    Removing,
}

pub fn elide_command(args: &ElideArgs) {
    todo!();
}
