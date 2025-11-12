use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::read_envelope;

/// Get the payload of the attachment.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope.
    attachment: Option<String>,
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let attachment = read_envelope(self.attachment.as_deref())?;
        Ok(attachment.attachment_payload()?.ur_string())
    }
}
