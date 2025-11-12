use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Remove an assertion from the given envelope. The assertion must be a single
/// envelope containing the entire assertion.
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
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let assertion = Envelope::from_ur_string(&self.assertion)?;
        Ok(envelope.remove_assertion(assertion).ur_string())
    }
}
