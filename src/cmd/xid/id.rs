use anyhow::Result;
use bc_components::XIDProvider;
use bc_ur::prelude::*;
use clap::{Args, ValueEnum};

use crate::{
    cmd::xid::{VerifyArgs, XIDDocumentReadable},
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum IDFormat {
    /// XID Identifier UR
    #[default]
    Ur,

    /// Hexadecimal
    Hex,

    /// Bytewords
    Bytewords,

    /// Bytemoji
    Bytemoji,
}

/// Validate the XID document and return its XID identifier.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output format of the XID identifier. May be repeated to output multiple
    /// formats.
    #[arg(long, default_value = "ur", num_args = 1)]
    format: Vec<IDFormat>,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;
        let result = self
            .format
            .iter()
            .map(|&format| match format {
                IDFormat::Ur => xid_document.xid().ur_string(),
                IDFormat::Hex => xid_document.xid().to_string(),
                IDFormat::Bytewords => {
                    xid_document.xid().bytewords_identifier(true)
                }
                IDFormat::Bytemoji => {
                    xid_document.xid().bytemoji_identifier(true)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(result)
    }
}
