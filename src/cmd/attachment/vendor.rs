use clap::Args;

use crate::utils::read_envelope;

/// Get the vendor of the attachment.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope.
    attachment: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let attachment = read_envelope(self.attachment.as_deref())?;
        attachment.attachment_vendor()
    }
}
