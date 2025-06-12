use anyhow::{Result, bail};
use bc_envelope::prelude::*;
use clap::Args;

/// Join a set of SSKR shares back into the original envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The shares to join (ur:envelope).
    shares: Vec<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // If envelopes is empty, read them from stdin, one per line.
        let mut shares = self.shares.clone();
        if shares.is_empty() {
            let mut line = String::new();
            while std::io::stdin().read_line(&mut line)? > 0 {
                shares.push(line.trim().to_string());
                line.clear();
            }
        }

        let shares: Vec<Envelope> = shares
            .iter()
            .map(|s| Envelope::from_ur_string(s).unwrap())
            .collect();

        // Make sure we have at least one.
        if shares.is_empty() {
            bail!("No share envelopes provided");
        }

        let shares_refs: Vec<&Envelope> = shares.iter().collect();
        let wrapped = bc_envelope::Envelope::sskr_join(&shares_refs)?;
        let result = wrapped.unwrap_envelope()?;
        Ok(result.ur_string())
    }
}
