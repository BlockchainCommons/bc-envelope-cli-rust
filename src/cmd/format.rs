use bc_envelope::{Envelope, with_format_context};
use bc_ur::URDecodable;
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
    fn exec(&self) -> Result<String, anyhow::Error> {
        // If no envelope string, throw an error
        if self.envelope.is_none() {
            return Err(anyhow::anyhow!("No envelope provided"));
        }
        let ur_string = self.envelope.as_ref().unwrap();
        let e = Envelope::from_ur_string(ur_string)?;
        let output = with_format_context!(|context| {
            e.format_opt(Some(context))
        });
        Ok(output)
    }
}
