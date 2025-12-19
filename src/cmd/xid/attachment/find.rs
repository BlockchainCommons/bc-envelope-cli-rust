use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike, xid::XIDDocumentReadable};

/// Find attachments in a XID document by vendor and/or conforms-to.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Filter by vendor name.
    #[arg(long)]
    vendor: Option<String>,

    /// Filter by conforms-to value.
    #[arg(long = "conforms-to")]
    conforms_to: Option<String>,

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

        let attachments = envelope.attachments_with_vendor_and_conforms_to(
            self.vendor.as_deref(),
            self.conforms_to.as_deref(),
        )?;

        let result = attachments
            .iter()
            .map(|a| a.ur_string())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(result)
    }
}
