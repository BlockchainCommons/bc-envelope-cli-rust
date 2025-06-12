pub mod add;
pub mod all;
pub mod at;
pub mod conforms_to;
pub mod count;
pub mod create;
pub mod find;
pub mod payload;
pub mod vendor;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Work with the envelope's attachments.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Add(add::CommandArgs),
    All(all::CommandArgs),
    At(at::CommandArgs),
    ConformsTo(conforms_to::CommandArgs),
    Count(count::CommandArgs),
    Create(create::CommandArgs),
    Payload(payload::CommandArgs),
    Vendor(vendor::CommandArgs),
    Find(find::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Add(args) => args.exec(),
            SubCommands::All(args) => args.exec(),
            SubCommands::At(args) => args.exec(),
            SubCommands::ConformsTo(args) => args.exec(),
            SubCommands::Count(args) => args.exec(),
            SubCommands::Create(args) => args.exec(),
            SubCommands::Payload(args) => args.exec(),
            SubCommands::Vendor(args) => args.exec(),
            SubCommands::Find(args) => args.exec(),
        }
    }
}
