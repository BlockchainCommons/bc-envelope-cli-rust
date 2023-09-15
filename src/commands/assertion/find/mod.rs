pub mod predicate;
pub mod object;

use clap::{Subcommand, Args};

use self::{object::ObjectArgs, predicate::PredicateArgs};

#[derive(Debug, Args)]
pub struct FindArgs {
    #[command(subcommand)]
    command: Option<FindCommands>,
}

#[derive(Debug, Subcommand)]
enum FindCommands {
    Object(ObjectArgs),
    Predicate(PredicateArgs),
}

pub fn find_command(args: &FindArgs) {
    todo!();
}
