use anyhow::Result;
use bc_components::{XID, XIDProvider};
use bc_ur::prelude::*;
use bc_xid::XIDVerifySignature;
use clap::{Args, ValueEnum};

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{VerifyArgs, XIDDocumentReadable, xid_from_document_envelope},
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

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        if self.verify_args.verify_signature() == XIDVerifySignature::None {
            let envelope = self.read_envelope()?;
            let xid = xid_from_document_envelope(&envelope)?;
            Ok(format_xid(&xid, &self.format))
        } else {
            let xid_document = self.read_xid_document_with_verify(
                self.verify_args.verify_signature(),
            )?;
            Ok(format_xid(&xid_document.xid(), &self.format))
        }
    }
}

fn format_xid(xid: &XID, formats: &[IDFormat]) -> String {
    formats
        .iter()
        .map(|&format| match format {
            IDFormat::Ur => xid.ur_string(),
            IDFormat::Hex => xid.to_string(),
            IDFormat::Bytewords => xid.bytewords_identifier(true),
            IDFormat::Bytemoji => xid.bytemoji_identifier(true),
        })
        .collect::<Vec<_>>()
        .join("\n")
}
