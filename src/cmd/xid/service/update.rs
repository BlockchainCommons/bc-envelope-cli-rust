use bc_components::URI;
use bc_xid::{HasName, HasPermissions};
use clap::Args;
use anyhow::{ Result, anyhow };

use crate::{
    cmd::xid::{
        private_options::PrivateOptions, utils::{ xid_document_to_ur_string, XIDDocumentReadable}, xid_privilege::XIDPrivilege
    },
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

use super::service_args::{ServiceArgs, ServiceArgsLike};

/// Updates the permissions, delegates, keys, capability identifer, or name of a service in a XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    service_args: ServiceArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl ServiceArgsLike for CommandArgs {
    fn uri(&self) -> Option<&URI> {
        self.service_args.uri()
    }

    fn name(&self) -> Option<&str> {
        self.service_args.name()
    }

    fn capability(&self) -> Option<&str> {
        self.service_args.capability()
    }

    fn permissions(&self) -> &[XIDPrivilege] {
        self.service_args.permissions()
    }

    fn keys(&self) -> &[bc_envelope::PublicKeys] {
        self.service_args.keys()
    }

    fn delegates(&self) -> &[bc_xid::XIDDocument] {
        self.service_args.delegates()
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs { }

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let uri = self.read_uri()?;

        let mut xid_document = self.read_xid_document()?;

        let mut service = xid_document
            .find_service_by_uri(&uri)
            .cloned()
            .ok_or_else(|| anyhow!("Service not found"))?;

        xid_document.take_service(&uri);

        if let Some(name) = self.name() {
            if !name.is_empty() {
                service.set_name(name);
            }
        }

        if let Some(capability) = self.capability() {
            if !capability.is_empty() {
                service.set_capability(capability);
            }
        }

        if !self.permissions().is_empty() {
            service.clear_all_permissions();
            for privilege in self.permissions() {
                service.add_allow((*privilege).into());
            }
        }

        if !self.keys().is_empty() {
            service.key_referenecs_mut().clear();
            for key in self.keys() {
                xid_document.check_contains_key(key)?;
                service.add_key(key)?;
            }
        }

        if !self.delegates().is_empty() {
            service.delegate_references_mut().clear();
            for delegate in self.delegates() {
                xid_document.check_contains_delegate(delegate)?;
                service.add_delegate(delegate)?;
            }
        }

        xid_document.check_service_consistency(&service)?;
        xid_document.add_service(service)?;

        Ok(xid_document_to_ur_string(&xid_document, PrivateOptions::Include))
    }
}
