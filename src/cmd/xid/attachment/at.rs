use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike, xid::XIDDocumentReadable};

/// Get the attachment at the specified index from a XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the attachment to retrieve (0-based).
    index: usize,

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

        let attachments = envelope.attachments()?;
        let attachment = attachments.get(self.index).ok_or_else(|| {
            anyhow::anyhow!("Index {} out of range", self.index)
        })?;
        Ok(attachment.ur_string())
    }
}
