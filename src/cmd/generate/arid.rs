use clap::Args;

/// Generate an Apparently Random Identifer (ARID).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
