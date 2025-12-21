use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike, read_envelope,
    xid::{
        OutputOptions, SigningArgs, VerifyArgs, XIDDocumentReadable,
        xid_document_to_ur_string,
    },
};

/// Remove an attachment from a XID document.
///
/// The attachment to remove is identified by providing the exact attachment
/// envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope to remove.
    attachment: String,

    #[command(flatten)]
    output_opts: OutputOptions,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    signing_args: SigningArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let mut xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;

        // Read the attachment to remove and get its digest
        let attachment_to_remove = read_envelope(Some(&self.attachment))?;
        attachment_to_remove.validate_attachment()?;
        let digest = attachment_to_remove.digest();

        // Remove from XID document using the Attachable trait
        xid_document
            .remove_attachment(digest)
            .ok_or_else(|| anyhow::anyhow!("Attachment not found"))?;

        let signing_options = self.signing_args.signing_options(None)?;

        xid_document_to_ur_string(
            &xid_document,
            &self.output_opts,
            None,
            None,
            signing_options,
        )
    }
}
