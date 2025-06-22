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
            .map_err(|e| {
                match e {
                    bc_envelope_pattern::Error::UnrecognizedToken(span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        let end = span.end.min(input.len());
                        let error_text = if start < input.len() {
                            &input[start..end]
                        } else {
                            "<end of input>"
                        };
                        anyhow::anyhow!(
                            "Failed to parse pattern at position {}..{}: unrecognized token '{}'\nPattern: {}\n         {}^",
                            start, end, error_text, input,
                            " ".repeat(start)
                        )
                    }
                    bc_envelope_pattern::Error::ExtraData(span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        anyhow::anyhow!(
                            "Failed to parse pattern: extra data at position {}\nPattern: {}\n         {}^",
                            start, input, " ".repeat(start)
                        )
                    }
                    bc_envelope_pattern::Error::UnexpectedToken(token, span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        anyhow::anyhow!(
                            "Failed to parse pattern at position {}: unexpected token {:?}\nPattern: {}\n         {}^",
                            start, token, input, " ".repeat(start)
                        )
                    }
                    _ => anyhow::anyhow!("Failed to parse pattern: {}", e),
                }
            })?;
        let matches = pattern.matches(&envelope);
        if matches {
            Ok(format!("Pattern matched: {}", self.pattern))
        } else {
            bail!("Pattern did not match: {}", self.pattern)
        }
    }
}
