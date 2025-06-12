use anyhow::Result;
use clap::Args;

use crate::{
    cmd::xid::utils::XIDDocumentReadable,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Print the count of the XID document's services.
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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document()?;
        Ok(xid_document.services().len().to_string())
    }
}
