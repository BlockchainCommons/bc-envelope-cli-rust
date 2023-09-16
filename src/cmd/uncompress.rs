use clap::Args;

/// Uncompress the envelope or its subject.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
