mod attachment;
mod delegate;
pub mod generator_options;
pub use generator_options::*;
pub mod id;
pub mod key;
pub mod key_args;
pub use key_args::*;
pub mod method;
pub mod new;
pub mod password_args;
pub use password_args::*;
pub mod private_options;
pub use private_options::*;
pub mod provenance;
pub mod service;
pub mod signing_args;
pub use signing_args::*;
pub mod xid_utils;
pub use xid_utils::*;
pub mod verify_args;
pub use verify_args::*;
pub mod xid_privilege;
use anyhow::Result;
use clap::{Args, Subcommand};
pub use xid_privilege::*;

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
    Provenance(provenance::CommandArgs),
    Id(id::CommandArgs),
    Key(key::CommandArgs),
    Method(method::CommandArgs),
    Delegate(delegate::CommandArgs),
    Service(service::CommandArgs),
    Attachment(attachment::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::New(args) => args.exec(),
            SubCommands::Provenance(args) => args.exec(),
            SubCommands::Id(args) => args.exec(),
            SubCommands::Key(args) => args.exec(),
            SubCommands::Method(args) => args.exec(),
            SubCommands::Delegate(args) => args.exec(),
            SubCommands::Service(args) => args.exec(),
            SubCommands::Attachment(args) => args.exec(),
        }
    }
}
