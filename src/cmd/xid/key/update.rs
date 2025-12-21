use anyhow::{Result, anyhow};
use bc_components::URI;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        GeneratorOutputArgs, KeyArgs, KeyArgsLike, OutputOptions,
        PrivateOptions, ReadWritePasswordArgs, SigningArgs, VerifyArgs,
        XIDDocumentReadable, XIDPrivilege, update_key,
        xid_document_to_ur_string,
    },
};

/// Updates the permissions, endpoints, or name of a key in a XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,

    #[command(flatten)]
    generator_args: GeneratorOutputArgs,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    signing_args: SigningArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl KeyArgsLike for CommandArgs {
    fn nickname(&self) -> &str { self.key_args.nickname() }

    fn private_opts(&self) -> PrivateOptions { self.key_args.private_opts() }

    fn endpoints(&self) -> &[URI] { self.key_args.endpoints() }

    fn permissions(&self) -> &[XIDPrivilege] { self.key_args.permissions() }

    fn keys(&self) -> Option<&str> { self.key_args.keys() }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let public_keys = self.read_public_key()?;

        let mut xid_document = self
            .read_xid_document_with_password_and_verify(
                &self.password_args.read,
                self.verify_args.verify_signature(),
            )?;

        let mut key = xid_document
            .find_key_by_public_keys(&public_keys)
            .cloned()
            .ok_or_else(|| anyhow!("Key not found"))?;

        xid_document.take_key(&key);
        update_key(
            &mut key,
            self.nickname(),
            self.endpoints(),
            self.permissions(),
        );
        xid_document.add_key(key)?;

        let signing_options = self
            .signing_args
            .signing_options(Some(&self.password_args.read))?;

        let output_opts = OutputOptions::new(
            self.private_opts(),
            self.generator_args.generator,
        );

        xid_document_to_ur_string(
            &xid_document,
            &output_opts,
            Some(&self.password_args.write),
            None,
            signing_options,
        )
    }
}
