use anyhow::{Result, bail};
use bc_envelope_pattern::{Matcher, Pattern};
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Match the envelope subject against a pattern.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The pattern to be matched.
    pattern: String,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let pattern = Pattern::parse(&self.pattern)
            .map_err(|e| anyhow::anyhow!("Failed to parse pattern: {}", e))?;
        let matches = pattern.matches(&envelope);
        if matches {
            Ok(format!("Pattern matched: {}", self.pattern))
        } else {
            bail!("Pattern did not match: {}", self.pattern)
        }
    }
}
