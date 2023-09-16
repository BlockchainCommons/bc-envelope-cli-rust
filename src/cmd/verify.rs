use clap::Args;

/// Verify a signature on the envelope using the provided public key base.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
