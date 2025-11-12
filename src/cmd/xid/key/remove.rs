use anyhow::Result;
use clap::Args;

use crate::{
    cmd::xid::{
        PrivateOptions, ReadWritePasswordArgs, SigningArgs, VerifyArgs,
        XIDDocumentReadable, read_public_key, xid_document_to_ur_string,
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Remove the given key from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The key to remove. If omitted, the key will be will read from stdin.
    #[arg(name = "KEYS")]
    keys: Option<String>,

    /// Whether to include, omit, elide, or encrypt private keys.
    #[arg(long = "private", default_value = "include")]
    private_opts: PrivateOptions,

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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let public_keys = read_public_key(self.keys.as_deref())?;
        let mut xid_document = self
            .read_xid_document_with_password_and_verify(
                &self.password_args.read,
                self.verify_args.verify_signature(),
            )?;
        xid_document.remove_key(&public_keys)?;

        let signing_options = self
            .signing_args
            .signing_options(Some(&self.password_args.read))?;

        xid_document_to_ur_string(
            &xid_document,
            self.private_opts,
            Some(&self.password_args.write),
            None,
            None,
            signing_options,
        )
    }
}
