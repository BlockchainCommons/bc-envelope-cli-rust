use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::read_envelope;

/// Create an attachment.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The vendor of the attachment. Usually a reverse domain name.
    vendor: String,

    /// An optional `conforms-to` value of the attachment. Usually a URI.
    #[arg(long, short)]
    conforms_to: Option<String>,

    /// The payload of the attachment. Entirely defined by the vendor.
    ///
    /// If not supplied, it is read from stdin.
    payload: Option<String>,
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let payload = read_envelope(self.payload.as_deref())?;
        let assertion = Envelope::new_attachment(
            payload,
            self.vendor.as_str(),
            self.conforms_to.as_deref(),
        );
        Ok(assertion.ur_string())
    }
}
