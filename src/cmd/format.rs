use clap::{Args, ValueEnum};

/// (default) Print the envelope in textual format.
#[derive(Debug, Args)]
pub struct CommandArgs {
    /// Output format.
    #[arg(long = "type", id = "TYPE", default_value = "envelope")]
    format_type: FormatType,

    /// For `--type tree` hides the NODE case, which provides a more semantically readable tree output.
    #[arg(long)]
    hide_nodes: bool,

    /// The envelope to output.
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
    fn exec(&self) {
        todo!();
    }
}
