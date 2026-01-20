use anyhow::{Result, bail};
use bc_components::URI;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        OutputOptions, ReadWritePasswordArgs, SigningArgs, VerifyArgs,
        XIDDocumentReadable, read_uri, xid_document_to_ur_string,
    },
};

/// Remove a resolution method (dereferenceVia) from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The URI of the resolution method to remove. If omitted, the URI will be
    /// read from stdin.
    uri: Option<URI>,

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
        let uri = read_uri(self.uri.as_ref())?;
        let mut xid_document = self
            .read_xid_document_with_password_and_verify(
                &self.password_args.read,
                self.verify_args.verify_signature(),
            )?;

        if xid_document.remove_resolution_method(&uri).is_none() {
            bail!("Resolution method not found: {}", uri);
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
