use bc_components::URI;
use bc_envelope::PublicKeys;
use bc_ur::URDecodable;
use bc_xid::XIDDocument;
use anyhow::Result;
use clap::Args;

use crate::cmd::xid::{utils::read_uri, xid_privilege::XIDPrivilege};

pub trait ServiceArgsLike {
    fn uri(&self) -> Option<&URI>;
    fn name(&self) -> Option<&str>;
    fn capability(&self) -> Option<&str>;
    fn permissions(&self) -> &[XIDPrivilege];
    fn keys(&self) -> &[PublicKeys];
    fn delegates(&self) -> &[XIDDocument];

    fn read_uri(&self) -> Result<URI> {
        read_uri(self.uri())
    }
}

fn parse_public_keys(s: &str) -> Result<PublicKeys, String> {
    PublicKeys::from_ur_string(s).map_err(|e| e.to_string())
}

fn parse_xid_document(s: &str) -> Result<XIDDocument, String> {
    XIDDocument::from_ur_string(s).map_err(|e| e.to_string())
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct ServiceArgs {
    /// A user-assigned name for the key.
    #[arg(long)]
    name: Option<String>,

    /// The capability identifier of the service.
    #[arg(long)]
    capability: Option<String>,

    /// A specific key for use with the service. May be repeated.
    #[arg(long = "key", name = "PUBLIC_KEYS", num_args = 1)]
    #[clap(value_parser = parse_public_keys)]
    keys: Vec<PublicKeys>,

    /// A delegate for the service. May be repeated.
    #[arg(long = "delegate", name = "XID", num_args = 1)]
    #[clap(value_parser = parse_xid_document)]
    delegates: Vec<XIDDocument>,

    /// Grant a specific permission to the service. May be repeated.
    #[arg(long = "allow", name = "PRIVILEGE", default_value = "all", num_args = 1)]
    permissions: Vec<XIDPrivilege>,

    /// The service URI. If omitted, the URI will be read from stdin.
    uri: Option<URI>,
}

impl ServiceArgsLike for ServiceArgs {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn capability(&self) -> Option<&str> {
        self.capability.as_deref()
    }

    fn keys(&self) -> &[PublicKeys] {
        &self.keys
    }

    fn delegates(&self) -> &[XIDDocument] {
        &self.delegates
    }

    fn permissions(&self) -> &[XIDPrivilege] {
        &self.permissions
    }

    fn uri(&self) -> Option<&URI> {
        self.uri.as_ref()
    }
}
