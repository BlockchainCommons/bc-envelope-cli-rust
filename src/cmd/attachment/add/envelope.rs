use clap::Args;
use bc_envelope::prelude::*;

use crate::{utils::read_envelope, envelope_args::{EnvelopeArgs, EnvelopeArgsLike}};

/// Add an attachment to the given envelope.
///
/// The attachment are provided as a single attachment envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope.
    attachment: String,

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
        let attachment = read_envelope(Some(&self.attachment))?;
        attachment.clone().validate_attachment()?;
        let e = envelope.add_assertion_envelope(attachment.clone())?;
        Ok(e.ur_string())
    }
}
