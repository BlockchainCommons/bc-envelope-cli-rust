use anyhow::Result;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{VerifyArgs, XIDDocumentReadable},
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
        let xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;
        Ok(xid_document.keys().len().to_string())
    }
}
