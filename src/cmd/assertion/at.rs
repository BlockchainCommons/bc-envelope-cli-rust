use bc_envelope::prelude::*;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Retrieve the assertion at the given index.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the assertion to retrieve.
    index: usize,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;
        let assertions = envelope.assertions();
        let assertion = assertions.get(self.index).ok_or_else(|| anyhow::anyhow!("Index out of bounds"))?;
        Ok(assertion.ur_string())
    }
}
