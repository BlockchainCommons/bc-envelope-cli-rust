use std::rc::Rc;

use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_envelope::prelude::*;

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
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;
        let assertion = Rc::new(Envelope::from_ur_string(&self.assertion)?);
        Ok(envelope.add_assertion_envelope_salted(assertion, self.salted)?.ur_string())
    }
}
