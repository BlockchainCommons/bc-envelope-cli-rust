use clap::Args;

/// Add an assertion to the given envelope. The assertion must be a single envelope containing the entire assertion.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
