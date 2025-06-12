use anyhow::Result;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use bc_xid::HasNickname;
use clap::Args;

use crate::{
    cmd::xid::utils::XIDDocumentReadable,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Find the XID document's keys by assigned name.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    name: String,

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

        let keys = xid_document.keys();
        let result = keys
            .iter()
            .filter_map(|key| {
                if key.nickname() == self.name {
                    Some(key.to_envelope().ur_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(result)
    }
}
