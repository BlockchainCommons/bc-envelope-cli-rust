use bc_components::URI;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use clap::Args;
use anyhow::Result;

use crate::{cmd::xid::utils::XIDDocumentReadable, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

/// Find the XID document's keys by assigned name.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    uri: URI,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs { }

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document()?;

        let services = xid_document.services();
        let result = services.iter().filter_map(|service| {
            if service.uri() == &self.uri {
                Some(service.to_envelope().ur_string())
            } else {
                None
            }
        }).collect::<Vec<String>>().join("\n");
        Ok(result)
    }
}
