use anyhow::Result;
use bc_envelope::prelude::*;
use clap::{Args, ValueEnum};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Print the envelope in textual format.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output format.
    #[arg(long = "type", id = "TYPE", default_value = "envelope")]
    format_type: FormatType,

    /// For `--type tree` hides the NODE case and digests, which provides a
    /// more semantically readable tree output.
    #[arg(long)]
    hide_nodes: bool,

    /// For `--type tree`, specifies the format for displaying digests.
    #[arg(long, short, default_value = "short")]
    digest_format: DigestFormatType,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum FormatType {
    /// Envelope notation.
    Envelope,
    /// Envelope tree.
    Tree,
    /// CBOR diagnostic notation.
    Diag,
    /// CBOR hex.
    Cbor,
    /// UR format.
    UR,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DigestFormatType {
    /// Default: Display a shortened version of the digest (first 8 characters).
    Short,
    /// Display the full digest for each element in the tree.
    Full,
    /// Display a `ur:digest` UR for each element in the tree.
    UR,
}

impl From<DigestFormatType> for DigestDisplayFormat {
    fn from(value: DigestFormatType) -> Self {
        match value {
            DigestFormatType::Short => DigestDisplayFormat::Short,
            DigestFormatType::Full => DigestDisplayFormat::Full,
            DigestFormatType::UR => DigestDisplayFormat::UR,
        }
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let e = self.read_envelope()?;
        let output = match self.format_type {
            FormatType::Envelope => e.format(),
            FormatType::Tree => e.tree_format_opt(
                TreeFormatOpts::default()
                    .hide_nodes(self.hide_nodes)
                    .digest_display(self.digest_format.into()),
            ),
            FormatType::Diag => e.diagnostic(),
            FormatType::Cbor => hex::encode(e.tagged_cbor_data()),
            FormatType::UR => e.ur_string(),
        };
        Ok(output)
    }
}
