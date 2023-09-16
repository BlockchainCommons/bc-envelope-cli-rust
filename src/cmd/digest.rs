use clap::Args;

/// Print the envelope's digest.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
