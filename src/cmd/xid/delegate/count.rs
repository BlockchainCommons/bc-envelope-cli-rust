use anyhow::Result;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike, xid::XIDDocumentReadable};

/// Print the count of the XID document's delegates.
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
        Ok(xid_document.delegates().len().to_string())
    }
}
