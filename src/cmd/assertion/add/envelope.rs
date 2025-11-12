use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Add an assertion to the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The assertion to add.
    ///
    /// Must be a single envelope containing the entire assertion.
    assertion: String,

    #[arg(short, long, default_value = "false")]
    salted: bool,

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
        Ok(envelope
            .add_assertion_envelope_salted(assertion, self.salted)?
            .ur_string())
    }
}
