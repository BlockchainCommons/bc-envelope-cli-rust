use clap::Args;

/// (DEFAULT) Elide all objects not in the target.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
