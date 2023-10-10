pub mod create;

use clap::{Subcommand, Args};

/// Work with the envelope's attachments.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    // Add(add::CommandArgs),
    // All(all::CommandArgs),
    // At(at::CommandArgs),
    // Count(count::CommandArgs),
    Create(create::CommandArgs),
    // Find(find::CommandArgs),
    // Remove(remove::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            // SubCommands::Add(args) => args.exec(),
            // SubCommands::All(args) => args.exec(),
            // SubCommands::At(args) => args.exec(),
            // SubCommands::Count(args) => args.exec(),
            SubCommands::Create(args) => args.exec(),
            // SubCommands::Find(args) => args.exec(),
            // SubCommands::Remove(args) => args.exec(),
        }
    }
}
