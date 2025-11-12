use anyhow::Result;
use bc_components::URI;
use bc_xid::{XIDDocument, XIDGenesisMarkOptions, XIDInceptionKeyOptions};
use clap::Args;
use dcbor::Date;

use super::{
    InputKey, KeyArgs, KeyArgsLike, PrivateOptions, SigningArgs,
    WritePasswordArgs, XIDPrivilege, generator_options::GeneratorOptions,
    update_key, xid_document_to_ur_string,
};
use crate::parse_ur_to_cbor;

/// Create a new XID document from an inception key
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,

    /// Whether to include or omit the provenance mark generator.
    ///
    /// If omitted (default), no provenance mark will be created.
    /// If included, a provenance mark will be generated and the generator will
    /// be attached. If encrypted, a provenance mark will be generated and
    /// the generator will be encrypted with the same password used for
    /// private keys.
    #[arg(long = "generator", default_value = "omit")]
    generator_opts: GeneratorOptions,

    /// Date for the genesis provenance mark (ISO 8601 format, e.g.,
    /// "2024-01-15"). Only used when --generator is 'include' or
    /// 'encrypt'. If not provided, the current date is used.
    #[arg(long)]
    date: Option<String>,

    /// Additional info to attach to the genesis mark (as any UR type).
    /// Only used when --generator is 'include' or 'encrypt'.
    /// Accepts any UR (ur:envelope, ur:digest, ur:arid, etc.)
    #[arg(long)]
    info: Option<String>,

    /// The integer CBOR tag for the info UR if it's an unknown type.
    #[arg(long)]
    ur_tag: Option<u64>,

    #[command(flatten)]
    password_args: WritePasswordArgs,

    #[command(flatten)]
    signing_args: SigningArgs,
}

impl KeyArgsLike for CommandArgs {
    fn nickname(&self) -> &str { self.key_args.nickname() }

    fn private_opts(&self) -> PrivateOptions { self.key_args.private_opts() }

    fn endpoints(&self) -> &[URI] { self.key_args.endpoints() }

    fn permissions(&self) -> &[XIDPrivilege] { self.key_args.permissions() }

    fn keys(&self) -> Option<&str> { self.key_args.keys() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_key()?;

        // Read password once if needed for either private keys or generator
        // encryption
        let shared_password = if self.generator_opts.is_encrypt()
            || self.private_opts().is_encrypt()
        {
            Some(self.password_args.read_password("Encryption password:")?)
        } else {
            None
        };

        // Determine genesis mark options based on generator_opts
        let genesis_mark_opts = match self.generator_opts {
            GeneratorOptions::Omit => XIDGenesisMarkOptions::None,
            GeneratorOptions::Include | GeneratorOptions::Encrypt => {
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

                // Use a random seed to initialize the provenance mark generator
                let mut rng = bc_rand::SecureRandomNumberGenerator;
                let random_seed =
                    provenance_mark::ProvenanceSeed::new_using(&mut rng);
                XIDGenesisMarkOptions::Seed(random_seed, None, date, info)
            }
            GeneratorOptions::Elide => {
                anyhow::bail!(
                    "Elide is not allowed for 'xid new'. Use 'omit' (the default) to create without a provenance mark, or 'include'/'encrypt' to create with one."
                )
            }
        };

        let mut xid_document = match &keys {
            InputKey::PrivateBase(private_key_base) => XIDDocument::new(
                XIDInceptionKeyOptions::PrivateKeyBase(
                    private_key_base.clone(),
                ),
                genesis_mark_opts,
            ),
            InputKey::Public(public_keys) => XIDDocument::new(
                XIDInceptionKeyOptions::PublicKeys(public_keys.clone()),
                genesis_mark_opts,
            ),
            InputKey::PrivateKeys(private_keys) => {
                let public_keys = private_keys.public_keys()?;
                XIDDocument::new(
                    XIDInceptionKeyOptions::PublicAndPrivateKeys(
                        public_keys,
                        private_keys.clone(),
                    ),
                    genesis_mark_opts,
                )
            }
            InputKey::PrivateAndPublicKeys(private_keys, public_keys) => {
                XIDDocument::new(
                    XIDInceptionKeyOptions::PublicAndPrivateKeys(
                        public_keys.clone(),
                        private_keys.clone(),
                    ),
                    genesis_mark_opts,
                )
            }
        };

        let mut key = xid_document.keys().iter().next().unwrap().clone();
        xid_document.take_key(&key);
        update_key(
            &mut key,
            self.nickname(),
            self.endpoints(),
            self.permissions(),
        );
        xid_document.add_key(key)?;

        let signing_options = self.signing_args.signing_options(None)?;

        xid_document_to_ur_string(
            &xid_document,
            self.private_opts(),
            Some(&self.password_args),
            Some(self.generator_opts),
            shared_password,
            signing_options,
        )
    }
}
