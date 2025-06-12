use anyhow::{Result, anyhow};
use bc_envelope::prelude::*;
use clap::Args;

use crate::utils::read_envelope;

/// Get the attachment at the specified index.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    index: usize,
    envelope: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = read_envelope(self.envelope.as_deref())?;
        let attachments = &envelope.attachments()?;
        let attachment = attachments
            .get(self.index)
            .ok_or_else(|| anyhow!("No attachment at index {}", self.index))?;
        Ok(attachment.ur_string())
    }
}
