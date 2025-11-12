use anyhow::{Result, bail};
use bc_envelope_pattern::{
    FormatPathsOpts, Matcher, PathElementFormat, Pattern, format_paths_opt,
};
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Match the envelope subject against a pattern.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The pattern to be matched.
    pattern: String,

    /// Disable indentation of path elements.
    #[arg(long)]
    no_indent: bool,

    /// Format only the last element of each path.
    #[arg(long)]
    last_only: bool,

    /// Format path elements as envelope URs.
    #[arg(long, group = "format")]
    envelopes: bool,

    /// Format path elements as digest URs.
    #[arg(long, group = "format")]
    digests: bool,

    /// Format path elements as summary.
    #[arg(long, group = "format")]
    summary: bool,

    /// Maximum length for summary truncation.
    #[arg(long, requires = "summary")]
    max_length: Option<usize>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::Exec for CommandArgs {
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
        let (paths, _captures) = pattern.paths_with_captures(&envelope);

        // Build format options from command line arguments
        let element_format = if self.envelopes {
            PathElementFormat::EnvelopeUR
        } else if self.digests {
            PathElementFormat::DigestUR
        } else if self.summary {
            PathElementFormat::Summary(self.max_length)
        } else {
            PathElementFormat::Summary(None)
        };

        let format_options = FormatPathsOpts::new()
            .indent(!self.no_indent)
            .element_format(element_format)
            .last_element_only(self.last_only);

        if !paths.is_empty() {
            Ok(format_paths_opt(&paths, format_options))
        } else {
            bail!("No match")
        }
    }
}
