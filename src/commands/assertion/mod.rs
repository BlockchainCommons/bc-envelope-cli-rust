pub mod add;
pub mod all;
pub mod at;
pub mod count;
pub mod create;
pub mod find;
pub mod remove;

use clap::{Subcommand, Args};

use self::{add::AddArgs, all::AllArgs, at::AtArgs, count::CountArgs, create::CreateArgs, find::FindArgs, remove::RemoveArgs};

/// Work with the envelope's assertions.
#[derive(Debug, Args)]
pub struct AssertionArgs {
    #[command(subcommand)]
    command: Option<AssertionCommands>,
}

#[derive(Debug, Subcommand)]
enum AssertionCommands {
    Add(AddArgs),
    All(AllArgs),
    At(AtArgs),
    Count(CountArgs),
    Create(CreateArgs),
    Find(FindArgs),
    Remove(RemoveArgs),
}

pub fn assertion_command(args: &AssertionArgs) {
    todo!();
}
