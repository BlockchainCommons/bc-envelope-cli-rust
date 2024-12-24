use bc_xid::{Delegate, XIDDocument};
use clap::Args;
use anyhow::{Result, bail};
use bc_ur::prelude::*;

use crate::{
    cmd::xid::{ key_privilege::KeyPrivilege, utils::XIDDocumentReadable },
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

use super::{add_delegate_permissions, xid_document_to_unsigned_envelope_ur_string};

/// Update a delegate in the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The XID document to replace the existing one.
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
        let replacement_xid_document = XIDDocument::from_ur_string(self.delegate.as_str())?;
        let mut delegate = Delegate::new(&replacement_xid_document);

        let mut xid_document = self.read_xid_document()?;
        xid_document.take_delegate(&delegate);
        if self.permissions.is_empty() {
            bail!("At least one permission must be granted to the delegate.");
        }

        add_delegate_permissions(&mut delegate, &self.permissions);
        xid_document.add_delegate(delegate)?;

        Ok(xid_document_to_unsigned_envelope_ur_string(xid_document))
    }
}
