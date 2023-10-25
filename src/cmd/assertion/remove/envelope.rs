use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_envelope::prelude::*;

/// Remove an assertion from the given envelope. The assertion must be a single envelope containing the entire assertion.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The assertion to remove.
    ///
    /// Must be a single envelope containing the entire assertion.
    assertion: String,

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
        let envelope = self.read_envelope()?;
        let assertion = Envelope::from_ur_string(&self.assertion)?;
        Ok(envelope.remove_assertion(assertion).ur_string())
    }
}
