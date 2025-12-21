use anyhow::Result;
use bc_components::URI;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        OutputOptions, SigningArgs, VerifyArgs, XIDDocumentReadable,
        xid_document_to_ur_string,
    },
};

/// Remove the given resolution method from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The resolution method to remove
    #[arg(name = "URI")]
    method: URI,

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
        xid_document.remove_resolution_method(&self.method);

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
