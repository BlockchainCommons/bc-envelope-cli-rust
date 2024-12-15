use bc_ur::prelude::*;
use bc_ur::URDecodable;
use bc_xid::{Delegate, HasPermissions, Privilege, XIDDocument};
use clap::Args;
use anyhow::{bail, Result};

use crate::{
    cmd::xid::{key_privilege::KeyPrivilege, utils::XIDDocumentReadable},
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

/// Add a delegate to the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The XID or XID document of the delegate to add.
    delegate: String,

    /// Grant a specific permission to the delegate. May be repeated.
    #[arg(long = "allow", name = "PRIVILEGE", default_value = "all", num_args = 1)]
    permissions: Vec<KeyPrivilege>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let delegate_xid = XIDDocument::from_ur_string(&self.delegate)?;
        let mut delegate = Delegate::new(delegate_xid);

        if self.permissions.is_empty() {
            bail!("At least one permission must be granted to the delegate.");
        }

        // If `All` is in the permissions, just add it.
        if self.permissions.contains(&KeyPrivilege::All) {
            delegate.add_allow(Privilege::All);
        } else {
            // Otherwise, add each permission.
            for permission in &self.permissions {
                delegate.add_allow((*permission).into());
            }
        }

        let mut xid_document = self.read_xid_document()?;
        xid_document.add_delegate(delegate);

        let unsigned_envelope = xid_document.to_unsigned_envelope();
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
