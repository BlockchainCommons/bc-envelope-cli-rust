use bc_components::URI;
use bc_envelope::{PrivateKeyBase, PublicKeyBase};
use bc_ur::prelude::*;
use bc_xid::{HasPermissions, Key, PrivateKeyOptions, XIDDocument};
use clap::Args;
use anyhow::{bail, Result};

use crate::{cmd::xid::{key_privilege::KeyPrivilege, private_options::PrivateOptions}, envelope_args::{EnvelopeArgs, EnvelopeArgsLike}};

/// Create a new XID document from an inception key, either ur:crypto-pubkeys or ur:crypto-prvkeys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    // Whether to omit, include, or elide the private key.
    #[arg(long, id = "private", default_value = "omit")]
    private_opts: PrivateOptions,

    /// Provide one or more endpoints for the key.
    #[arg(long, name = "URI", num_args = 1..)]
    endpoints: Vec<URI>,

    /// Grant specific permissions to the key.
    #[arg(long, name = "PRIVILEGE", default_value = "all", num_args = 1..)]
    permissions: Vec<KeyPrivilege>,

    /// The public or private key base to add.
    #[arg(name = "KEYS")]
    keys: Option<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl CommandArgs {
    fn read_keys(&self) -> Result<String> {
        let mut ur_string = String::new();
        if self.keys.is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.keys.as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            bail!("No public or private key base provided");
        }
        Ok(ur_string.trim().to_string())
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_keys()?;
        let envelope = self.read_envelope()?;
        let mut xid_document = XIDDocument::from_unsigned_envelope(&envelope)?;

        let mut key = if let Ok(private_key_base) = PrivateKeyBase::from_ur_string(&keys) {
            Key::new_with_private_key(private_key_base)
        } else if let Ok(public_key_base) = PublicKeyBase::from_ur_string(&keys) {
            Key::new(public_key_base)
        } else {
            bail!("Invalid public or private key base");
        };

        if !self.endpoints.is_empty() {
            for uri in &self.endpoints {
                key.add_endpoint(uri.clone());
            }
        }
        if !self.permissions.is_empty() {
            key.clear_all_permissions();
            for privilege in &self.permissions {
                key.add_permission((*privilege).into());
            }
        }

        xid_document.add_key(key)?;

        let options = PrivateKeyOptions::from(self.private_opts);
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
