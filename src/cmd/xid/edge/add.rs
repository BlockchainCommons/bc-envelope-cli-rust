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

/// Add an edge to a XID document.
///
/// The edge must be provided as a pre-constructed edge envelope.
/// It may be signed (wrapped) or unsigned.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The edge envelope to add (as a UR).
    edge: String,

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

        let edge_envelope = read_envelope(Some(&self.edge))?;

        // Validate edge structure
        edge_envelope
            .validate_edge()
            .map_err(|e| anyhow::anyhow!("Invalid edge envelope: {}", e))?;

        // Add edge using the Edgeable trait
        xid_document.add_edge(edge_envelope);

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
