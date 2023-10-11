use clap::Args;
use crate::utils::read_envelope;

/// Print the count of the envelope's assertions.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    envelope: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = read_envelope(self.envelope.as_deref())?;
        Ok(envelope.attachments()?.len().to_string())
    }
}
