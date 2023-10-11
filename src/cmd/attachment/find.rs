use clap::Args;
use bc_envelope::prelude::*;

use crate::utils::read_envelope;

/// Retrieve attachments having the specified attributes.
///
/// If no attributes are specified, all attachments are returned.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[arg(long, short)]
    vendor: Option<String>,

    #[arg(long, short)]
    conforms_to: Option<String>,

    envelope: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = read_envelope(self.envelope.as_deref())?;
        let attachments = envelope.attachments_with_vendor_and_conforms_to(self.vendor.as_deref(), self.conforms_to.as_deref())?;
        let result = attachments.into_iter().map(|a| a.ur_string()).collect::<Vec<_>>().join("\n");
        Ok(result)
    }
}
