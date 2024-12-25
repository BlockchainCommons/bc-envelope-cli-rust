use bc_ur::prelude::*;
use bc_xid::{Delegate, XIDDocument};
use clap::Args;
use anyhow::{bail, Result};

use crate::{
    cmd::xid::{xid_privilege::XIDPrivilege, utils::XIDDocumentReadable},
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

use super::add_delegate_permissions;
use super::xid_document_to_unsigned_envelope_ur_string;

/// Add a delegate to the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The XID or XID document of the delegate to add.
    delegate: String,

    /// Grant a specific permission to the delegate. May be repeated.
    #[arg(long = "allow", name = "PRIVILEGE", default_value = "all", num_args = 1)]
    permissions: Vec<XIDPrivilege>,

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

        add_delegate_permissions(&mut delegate, &self.permissions);

        let mut xid_document = self.read_xid_document()?;
        xid_document.add_delegate(delegate)?;

        Ok(xid_document_to_unsigned_envelope_ur_string(xid_document))
    }
}
