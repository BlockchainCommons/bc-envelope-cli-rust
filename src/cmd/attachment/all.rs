use clap::Args;
use bc_envelope::prelude::*;
use anyhow::Result;

use crate::utils::read_envelope;

/// Retrieve all the envelope's assertions.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    envelope: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = read_envelope(self.envelope.as_deref())?;
        let attachments = envelope.attachments()?;
        let result = attachments.into_iter().map(|a| a.ur_string()).collect::<Vec<_>>().join("\n");
        Ok(result)
    }
}
