use clap::Args;

/// Create a bare assertion with the given predicate and object.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
