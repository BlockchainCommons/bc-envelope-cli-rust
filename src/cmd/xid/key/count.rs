use anyhow::Result;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        VerifyArgs, XIDDocumentReadable, xid_document_envelope,
        xid_from_document_envelope,
    },
};

/// Print the count of the XID document's keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        if self.verify_args.verify_signature()
            == bc_xid::XIDVerifySignature::None
        {
            let envelope = self.read_envelope()?;
            xid_from_document_envelope(&envelope)?;
            let envelope = xid_document_envelope(&envelope)?;
            return Ok(envelope
                .assertions_with_predicate(bc_envelope::known_values::KEY)
                .len()
                .to_string());
        }

        let xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;
        Ok(xid_document.keys().len().to_string())
    }
}
