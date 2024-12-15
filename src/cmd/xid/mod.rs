pub mod new;
pub mod key;
pub mod private_options;
pub mod key_privilege;
pub mod key_args;
pub mod utils;
pub mod method;

use clap::{Subcommand, Args};
use anyhow::Result;

/// Work with Extensible Identifiers (XID).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    New(new::CommandArgs),
    Key(key::CommandArgs),
    Method(method::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::New(args) => args.exec(),
            SubCommands::Key(args) => args.exec(),
            SubCommands::Method(args) => args.exec(),
        }
    }
}
