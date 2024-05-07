use clap::Args;
use bc_envelope::prelude::*;
use anyhow::Result;

use crate::{utils::read_envelope, envelope_args::{EnvelopeArgs, EnvelopeArgsLike}};

/// Add an attachment to the given envelope by specifying its components.
///
/// The components of the attachment are provided as separate arguments.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The vendor of the attachment. Usually a reverse domain name.
    vendor: String,

    /// An optional `conforms-to` value of the attachment. Usually a URI.
    #[arg(long, short)]
    conforms_to: Option<String>,

    /// The payload of the attachment. Entirely defined by the vendor.
    payload: String,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let payload = read_envelope(Some(&self.payload))?;
        let e = envelope.add_attachment(payload, self.vendor.as_str(), self.conforms_to.as_deref());
        Ok(e.ur_string())
    }
}
