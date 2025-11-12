use anyhow::Result;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use bc_xid::XIDDocument;
use clap::Args;

use crate::{
    cmd::xid::XIDDocumentReadable,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Find a delegate in the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The XID of the delegate to find. Can be a bare XID or a XID Document.
    delegate: String,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let target_xid_document =
            XIDDocument::from_ur_string(self.delegate.as_str())?;
        let xid_document = self.read_xid_document()?;
        if let Some(delegate) =
            xid_document.find_delegate_by_xid(&target_xid_document)
        {
            Ok(delegate.to_envelope().ur_string())
        } else {
            Ok("".to_string())
        }
    }
}
