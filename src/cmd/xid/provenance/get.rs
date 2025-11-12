use anyhow::Result;
use bc_ur::prelude::*;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{ReadWritePasswordArgs, VerifyArgs, XIDDocumentReadable},
};

/// Extract the provenance mark from an XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

    #[command(flatten)]
    verify_args: VerifyArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document_with_password_and_verify(
            &self.password_args.read,
            self.verify_args.verify_signature(),
        )?;

        // Extract the provenance mark if it exists
        if let Some(provenance_mark) = xid_document.provenance() {
            // Return the ProvenanceMark as a UR string
            Ok(provenance_mark.ur_string())
        } else {
            // No provenance mark - return empty string
            Ok(String::new())
        }
    }
}
