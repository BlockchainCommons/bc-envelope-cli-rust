use std::collections::HashSet;

use anyhow::Result;
use bc_components::{Digest, DigestProvider};
use bc_envelope::{
    Envelope,
    known_values::{KEY, PRIVATE_KEY, PROVENANCE, PROVENANCE_GENERATOR},
};
use bc_ur::prelude::*;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        GeneratorOptions, OutputOptions, PrivateOptions, ReadWritePasswordArgs,
        SigningArgs, VerifyArgs, XIDDocumentReadable,
        xid_document_to_ur_string,
    },
};

/// Export a XID document with specified output options.
///
/// This command reads an existing XID document and outputs it with
/// the specified handling of private keys and provenance generator.
/// Use this to create publicly distributable versions of XID documents.
///
/// # Examples
///
/// Create a publicly distributable version with elided secrets:
///
/// ```sh
/// envelope xid export --private elide --generator elide $XID
/// ```
///
/// Create a minimal version without secrets (requires re-signing):
///
/// ```sh
/// envelope xid export --private omit --generator omit --sign inception $XID
/// ```
///
/// Encrypt secrets for secure storage:
///
/// ```sh
/// envelope xid export --private encrypt --generator encrypt \
///     --encrypt-password "secret" $XID
/// ```
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
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
        let private_opts = self.output_opts.private_opts();
        let generator_opts = self.output_opts.generator_opts();

        // For elide or include operations, work directly at the envelope level
        // to preserve signatures. Only omit and encrypt require reconstruction.
        let can_use_envelope_elision = matches!(
            (private_opts, generator_opts),
            (PrivateOptions::Elide, GeneratorOptions::Elide)
                | (PrivateOptions::Elide, GeneratorOptions::Include)
                | (PrivateOptions::Include, GeneratorOptions::Elide)
                | (PrivateOptions::Include, GeneratorOptions::Include)
        );

        if can_use_envelope_elision {
            return self.elide_at_envelope_level(private_opts, generator_opts);
        }

        // For omit, encrypt, or mixed operations, use the reconstruction
        // approach
        self.reconstruct_document()
    }
}

impl CommandArgs {
    /// Perform elision directly on the envelope, preserving signatures.
    fn elide_at_envelope_level(
        &self,
        private_opts: PrivateOptions,
        generator_opts: GeneratorOptions,
    ) -> Result<String> {
        // Read the original envelope
        let envelope = self.read_envelope()?;

        // Verify signature if requested
        if self.verify_args.verify_signature() == XIDVerifySignature::Inception
        {
            // Parse just to verify
            let _ = XIDDocument::from_envelope(
                &envelope,
                None,
                XIDVerifySignature::Inception,
            )?;
        }

        // Collect digests to elide
        let mut digests_to_elide: HashSet<Digest> = HashSet::new();

        // Get the inner XID document
        // A signed envelope has its subject wrapped: { XID [...] } [ 'signed':
        // Signature ]
        let inner = if envelope.subject().is_wrapped() {
            envelope.subject().try_unwrap()?
        } else {
            envelope.clone()
        };

        // Find privateKey assertions to elide
        if private_opts == PrivateOptions::Elide {
            collect_private_key_digests(&inner, &mut digests_to_elide)?;
        }

        // Find provenanceGenerator assertions to elide
        if generator_opts == GeneratorOptions::Elide {
            collect_generator_digests(&inner, &mut digests_to_elide)?;
        }

        // Elide the collected digests from the original envelope
        let elided = if digests_to_elide.is_empty() {
            envelope
        } else {
            envelope.elide_removing_set(&digests_to_elide)
        };

        // Return as XID UR
        Ok(UR::new("xid", elided.to_cbor())?.string())
    }

    /// Reconstruct the document (for omit, encrypt, or other operations).
    fn reconstruct_document(&self) -> Result<String> {
        // Read the XID document with optional password and verification
        let xid_document = self.read_xid_document_with_password_and_verify(
            &self.password_args.read,
            self.verify_args.verify_signature(),
        )?;

        // Get signing options
        let signing_options = self
            .signing_args
            .signing_options(Some(&self.password_args.read))?;

        // Convert to UR string with the specified output options
        xid_document_to_ur_string(
            &xid_document,
            &self.output_opts,
            Some(&self.password_args.write),
            None,
            signing_options,
        )
    }
}

/// Collect digests of privateKey assertions from all keys in the XID document.
fn collect_private_key_digests(
    envelope: &Envelope,
    digests: &mut HashSet<Digest>,
) -> Result<()> {
    // Find all 'key' assertions
    for key_assertion in envelope.assertions_with_predicate(KEY) {
        let key_object = key_assertion.try_object()?;
        // Find the privateKey assertion within this key
        if let Some(pk_assertion) =
            key_object.optional_assertion_with_predicate(PRIVATE_KEY)?
        {
            digests.insert(pk_assertion.digest().to_owned());
        }
    }
    Ok(())
}

/// Collect digests of provenanceGenerator assertions from provenance marks.
fn collect_generator_digests(
    envelope: &Envelope,
    digests: &mut HashSet<Digest>,
) -> Result<()> {
    // Find the 'provenance' assertion
    if let Some(prov_assertion) =
        envelope.optional_assertion_with_predicate(PROVENANCE)?
    {
        let prov_object = prov_assertion.try_object()?;
        // Find the provenanceGenerator assertion within it
        if let Some(gen_assertion) = prov_object
            .optional_assertion_with_predicate(PROVENANCE_GENERATOR)?
        {
            digests.insert(gen_assertion.digest().to_owned());
        }
    }
    Ok(())
}
