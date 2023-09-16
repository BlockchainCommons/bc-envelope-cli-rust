use clap::{Subcommand, Args};

/// Elide a subset of elements.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<ElideCommands>,
}

#[derive(Debug, Subcommand)]
enum ElideCommands {
    Revealing,
    Removing,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
