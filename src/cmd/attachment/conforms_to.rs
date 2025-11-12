use anyhow::Result;
use clap::Args;

use crate::read_envelope;

/// Get the optional conformance of the attachment.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope.
    attachment: Option<String>,
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let attachment = read_envelope(self.attachment.as_deref())?;
        Ok(attachment.attachment_conforms_to()?.unwrap_or_default())
    }
}
