use anyhow::Result;
use bc_components::URI;
use bc_ur::prelude::*;
use clap::Args;

use crate::{
    cmd::xid::XIDDocumentReadable,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Add a resolution method to a XID document
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The resolution method to add
    #[arg(name = "URI")]
    method: URI,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let mut xid_document = self.read_xid_document()?;
        xid_document.add_resolution_method(self.method.clone());
        Ok(xid_document.ur_string())
    }
}
