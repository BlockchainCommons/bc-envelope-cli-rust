use anyhow::Result;
use bc_envelope::Envelope;
use bc_ur::prelude::*;
use clap::Args;
use dcbor::Date;
use provenance_mark::ProvenanceMarkGenerator;

use crate::{
    cmd::xid::{
        GeneratorOptions, PrivateOptions, ReadWritePasswordArgs, SigningArgs,
        VerifyArgs, XIDDocumentReadable, xid_document_to_ur_string,
    },
    data_types::parse_ur_to_cbor,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
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
    #[arg(long)]
    generator: Option<String>,

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

impl crate::exec::Exec for CommandArgs {
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
        if let Some(generator_str) = &self.generator {
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

        // Determine generator options: preserve the generator if it exists
        // Check if the document has a generator by trying to get it
        let has_generator = xid_document.provenance_generator().is_some();

        let generator_opts = if has_generator {
            // Generator exists - keep it included (plaintext)
            // Note: If it was encrypted, it's now decrypted after the next()
            // call We re-encrypt it using the write password if
            // specified
            Some(GeneratorOptions::Include)
        } else {
            // No generator, omit it
            Some(GeneratorOptions::Omit)
        };

        xid_document_to_ur_string(
            &xid_document,
            PrivateOptions::default(),
            Some(&self.password_args.write),
            generator_opts,
            self.password_args
                .read
                .read_password("Decryption password:")?,
            signing_options,
        )
    }
}
