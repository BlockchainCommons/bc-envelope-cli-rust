pub mod delegate;
pub mod id;
pub mod key;
pub mod key_args;
pub mod method;
pub mod new;
pub mod password_args;
pub mod private_options;
pub mod generator_options;
pub mod service;
pub mod utils;
pub mod xid_privilege;

use anyhow::Result;
use clap::{Args, Subcommand};

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
    Id(id::CommandArgs),
    Key(key::CommandArgs),
    Method(method::CommandArgs),
    Delegate(delegate::CommandArgs),
    Service(service::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::New(args) => args.exec(),
            SubCommands::Id(args) => args.exec(),
            SubCommands::Key(args) => args.exec(),
            SubCommands::Method(args) => args.exec(),
            SubCommands::Delegate(args) => args.exec(),
            SubCommands::Service(args) => args.exec(),
        }
    }
}
