use clap::Args;

/// Create an envelope with the given assertion (predicate and object).
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
