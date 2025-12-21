use anyhow::Result;
use bc_envelope::Envelope;
use bc_ur::prelude::*;
use clap::Args;
use dcbor::Date;
use provenance_mark::ProvenanceMarkGenerator;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike, parse_ur_to_cbor,
    xid::{
        GeneratorOutputArgs, OutputOptions, PrivateOutputArgs,
        ReadWritePasswordArgs, SigningArgs, VerifyArgs, XIDDocumentReadable,
        xid_document_to_ur_string,
    },
};

/// Advance the provenance mark to the next state.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,

    /// Date for the new provenance mark (ISO 8601 format, e.g., "2024-01-15").
    /// If not provided, the current date is used.
    #[arg(long)]
    date: Option<String>,

    /// Additional info to attach to the new mark (as any UR type).
    /// Accepts any UR (ur:envelope, ur:digest, ur:arid, etc.)
    #[arg(long)]
    info: Option<String>,

    /// The integer CBOR tag for the info UR if it's an unknown type.
    #[arg(long)]
    ur_tag: Option<u64>,

    /// External provenance mark generator (as ur:envelope).
    /// Required if the document does not have an embedded generator.
    #[arg(long = "external-generator")]
    external_generator: Option<String>,

    #[command(flatten)]
    private_output_args: PrivateOutputArgs,

    #[command(flatten)]
    generator_output_args: GeneratorOutputArgs,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    signing_args: SigningArgs,
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

        // Parse optional date parameter
        let date = if let Some(date_str) = &self.date {
            Some(Date::from_string(date_str)?)
        } else {
            None
        };

        // Parse optional info parameter - convert any UR to CBOR
        let info = if let Some(info_str) = &self.info {
            Some(parse_ur_to_cbor(info_str, self.ur_tag)?)
        } else {
            None
        };

        // Determine if we should use embedded or provided generator
        if let Some(generator_str) = &self.external_generator {
            // User provided a generator - use
            // next_provenance_mark_with_provided_generator
            let generator_envelope = Envelope::from_ur_string(generator_str)?;
            let mut generator: ProvenanceMarkGenerator =
                generator_envelope.try_into()?;

            xid_document.next_provenance_mark_with_provided_generator(
                &mut generator,
                date,
                info,
            )?;
        } else {
            // No generator provided - use
            // next_provenance_mark_with_embedded_generator
            let password = self
                .password_args
                .read
                .read_password("Decryption password:")?
                .map(|s| s.as_bytes().to_vec());
            xid_document.next_provenance_mark_with_embedded_generator(
                password, date, info,
            )?;
        }

        // Convert updated document to UR string
        let signing_options = self
            .signing_args
            .signing_options(Some(&self.password_args.read))?;

        let output_opts = OutputOptions::new(
            self.private_output_args.private,
            self.generator_output_args.generator,
        );

        xid_document_to_ur_string(
            &xid_document,
            &output_opts,
            Some(&self.password_args.write),
            self.password_args
                .read
                .read_password("Decryption password:")?,
            signing_options,
        )
    }
}
