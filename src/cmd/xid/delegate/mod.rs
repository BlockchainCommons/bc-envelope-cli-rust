pub mod add;
pub mod all;
pub mod at;
pub mod count;
pub mod find;
pub mod remove;
pub mod update;

use bc_ur::prelude::*;
use bc_xid::{Delegate, HasPermissions, Privilege, XIDDocument};
use clap::{Subcommand, Args};
use anyhow::Result;

use super::key_privilege::KeyPrivilege;

/// Work with a XID document's keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Add(add::CommandArgs),
    All(all::CommandArgs),
    At(at::CommandArgs),
    Count(count::CommandArgs),
    Find(find::CommandArgs),
    Remove(remove::CommandArgs),
    Update(update::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Add(args) => args.exec(),
            SubCommands::All(args) => args.exec(),
            SubCommands::At(args) => args.exec(),
            SubCommands::Count(args) => args.exec(),
            SubCommands::Find(args) => args.exec(),
            SubCommands::Remove(args) => args.exec(),
            SubCommands::Update(args) => args.exec(),
        }
    }
}

fn add_delegate_permissions(delegate: &mut Delegate, permissions: &[KeyPrivilege]) {
    // If `All` is in the permissions, just add it.
    if permissions.contains(&KeyPrivilege::All) {
        delegate.add_allow(Privilege::All);
    } else {
        // Otherwise, add each permission.
        for permission in permissions {
            delegate.add_allow((*permission).into());
        }
    }
}

fn xid_document_to_unsigned_envelope_ur_string(xid_document: XIDDocument) -> String {
    let unsigned_envelope = xid_document.to_unsigned_envelope();
    UR::new("xid", unsigned_envelope.to_cbor()).unwrap().string()
}
