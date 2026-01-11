use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike, read_envelope,
    xid::{
        OutputOptions, ReadWritePasswordArgs, SigningArgs, VerifyArgs,
        XIDDocumentReadable, xid_document_to_ur_string,
    },
};

/// Add an attachment to a XID document.
///
/// The attachment can be provided as either:
/// - An attachment envelope (created with `envelope attachment create`)
/// - Component parts (vendor, conforms-to, payload) which will be assembled
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The vendor of the attachment. Usually a reverse domain name.
    #[arg(long, required_unless_present = "attachment")]
    vendor: Option<String>,

    /// An optional `conforms-to` value of the attachment. Usually a URI.
    #[arg(long = "conforms-to")]
    conforms_to: Option<String>,

    /// The payload of the attachment (as a UR). Required when using --vendor.
    #[arg(long, required_unless_present = "attachment")]
    payload: Option<String>,

    /// A pre-made attachment envelope. Alternative to vendor/payload.
    #[arg(long, conflicts_with_all = ["vendor", "conforms_to", "payload"])]
    attachment: Option<String>,

    #[command(flatten)]
    output_opts: OutputOptions,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

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
        let mut xid_document = self
            .read_xid_document_with_password_and_verify(
                &self.password_args.read,
                self.verify_args.verify_signature(),
            )?;

        // Add the attachment using the Attachable trait
        if let Some(attachment_str) = &self.attachment {
            // Use pre-made attachment - need to add it via its components
            let attachment_envelope = read_envelope(Some(attachment_str))?;
            attachment_envelope.validate_attachment()?;

            // Extract components and add
            let payload = attachment_envelope.attachment_payload()?;
            let vendor = attachment_envelope.attachment_vendor()?;
            let conforms_to =
                attachment_envelope.attachment_conforms_to().ok().flatten();

            xid_document.add_attachment(
                payload,
                &vendor,
                conforms_to.as_deref(),
            );
        } else {
            // Build from components
            let vendor = self.vendor.as_ref().unwrap();
            let payload_envelope = read_envelope(self.payload.as_deref())?;

            xid_document.add_attachment(
                payload_envelope,
                vendor,
                self.conforms_to.as_deref(),
            );
        }

        let signing_options = self
            .signing_args
            .signing_options(Some(&self.password_args.read))?;

        xid_document_to_ur_string(
            &xid_document,
            &self.output_opts,
            Some(&self.password_args.write),
            None,
            signing_options,
        )
    }
}
