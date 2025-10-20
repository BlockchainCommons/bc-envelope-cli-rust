use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Unelide nodes using provided envelopes.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Envelopes to use for uneliding (space-separated UR envelopes).
    #[arg(value_delimiter = ' ', required = true)]
    envelopes: Vec<String>,
}

impl CommandArgs {
    pub fn exec_with_envelope(&self, envelope: Envelope) -> Result<String> {
        let mut unelide_envelopes = Vec::new();
        for ur_string in &self.envelopes {
            let env = Envelope::from_ur_string(ur_string)?;
            unelide_envelopes.push(env);
        }

        let result = envelope.walk_unelide(&unelide_envelopes);
        Ok(result.ur_string())
    }
}
