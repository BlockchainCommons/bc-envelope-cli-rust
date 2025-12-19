use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike, xid::XIDDocumentReadable};

/// Get the count of attachments in a XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document()?;
        let envelope: Envelope = xid_document.into();

        Ok(envelope.attachments()?.len().to_string())
    }
}
