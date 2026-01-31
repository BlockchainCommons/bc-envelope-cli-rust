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

/// Remove an edge from a XID document.
///
/// The edge to remove is identified by providing the exact edge envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The edge envelope to remove.
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

        // Read the edge to remove and get its digest
        let edge_to_remove = read_envelope(Some(&self.edge))?;
        let digest = edge_to_remove.digest();

        // Remove from XID document using the Edgeable trait
        xid_document
            .remove_edge(digest)
            .ok_or_else(|| anyhow::anyhow!("Edge not found"))?;

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
