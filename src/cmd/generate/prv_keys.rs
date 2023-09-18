use clap::Args;

/// Generate a private key base.
///
/// Generated randomly, or deterministically if a seed is provided.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        todo!();
    }
}
