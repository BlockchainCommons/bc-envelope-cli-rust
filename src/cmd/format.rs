use bc_envelope::preamble::*;
use bc_ur::URDecodable;
use clap::{Args, ValueEnum};
use dcbor::CBORTaggedEncodable;

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

    /// The envelope to output. If the envelope is not supplied on the command line, it is read from stdin.
    envelope: Option<String>,
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
        let mut ur_string = String::new();
        if self.envelope.is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.envelope.as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            anyhow::bail!("No envelope provided");
        }
        let e = Envelope::from_ur_string(ur_string.trim())?;
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
