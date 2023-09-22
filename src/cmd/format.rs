use bc_envelope::preamble::*;
use clap::{Args, ValueEnum};
use dcbor::CBORTaggedEncodable;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Print the envelope in textual format.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output format.
    #[arg(long = "type", id = "TYPE", default_value = "envelope")]
    format_type: FormatType,

    /// For `--type tree` hides the NODE case, which provides a more semantically readable tree output.
    #[arg(long)]
    hide_nodes: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
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
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let e = self.get_envelope()?;
        let output = match self.format_type {
            FormatType::Envelope => with_format_context!(|context| {
                e.format_opt(Some(context))
            }),
            FormatType::Tree => with_format_context!(|context| {
                e.tree_format_opt(self.hide_nodes, Some(context))
            }),
            FormatType::Diag => with_format_context!(|context| {
                e.diagnostic_opt(true, Some(context))
            }),
            FormatType::Cbor => hex::encode(e.tagged_cbor_data()),
        };
        Ok(output)
    }
}
