use anyhow::Result;
use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::XIDVerifySignature;
use clap::Args;
use provenance_mark::ProvenanceMark;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        ReadWritePasswordArgs, VerifyArgs, XIDDocumentReadable,
        xid_document_envelope, xid_from_document_envelope,
    },
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
        if self.verify_args.verify_signature() == XIDVerifySignature::None {
            let envelope = self.read_envelope()?;
            xid_from_document_envelope(&envelope)?;
            let envelope = xid_document_envelope(&envelope)?;
            if let Some(provenance_assertion) = envelope
                .optional_assertion_with_predicate(known_values::PROVENANCE)?
            {
                let provenance_envelope = provenance_assertion.try_object()?;
                let provenance_mark: ProvenanceMark =
                    provenance_envelope.extract_subject()?;
                return Ok(provenance_mark.ur_string());
            }
            return Ok(String::new());
        }

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
