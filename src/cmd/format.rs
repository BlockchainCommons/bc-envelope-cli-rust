use anyhow::Result;
use bc_envelope::prelude::*;
use clap::{Args, ValueEnum};

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Print the envelope in textual format.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output format.
    #[arg(long = "type", id = "TYPE", default_value = "envelope")]
    format_type: FormatType,

    /// For `tree` and `mermaid`, hides the NODE case and digests, which
    /// provides a more semantically readable tree output.
    #[arg(long)] // No short because it conflicts with `-h` for help.
    hide_nodes: bool,

    /// For `tree`, specifies the format for displaying digests.
    #[arg(long, short, default_value = "short")]
    digest_format: DigestFormatType,

    /// For `mermaid`, specifies the color theme of the diagram.
    #[arg(long, short, default_value = "default")]
    theme: MermaidThemeType,

    /// For `mermaid`, specifies the orientation of the diagram.
    #[arg(long, short, default_value = "left-to-right")]
    orientation: MermaidOrientationType,

    /// For `mermaid`, do not color the nodes or edges.
    #[arg(long, short)]
    monochrome: bool,

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
    /// Mermaid format.
    Mermaid,
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

#[derive(ValueEnum, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MermaidOrientationType {
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
}

impl From<MermaidOrientationType> for MermaidOrientation {
    fn from(value: MermaidOrientationType) -> Self {
        match value {
            MermaidOrientationType::LeftToRight => {
                MermaidOrientation::LeftToRight
            }
            MermaidOrientationType::TopToBottom => {
                MermaidOrientation::TopToBottom
            }
            MermaidOrientationType::RightToLeft => {
                MermaidOrientation::RightToLeft
            }
            MermaidOrientationType::BottomToTop => {
                MermaidOrientation::BottomToTop
            }
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MermaidThemeType {
    Default,
    Neutral,
    Dark,
    Forest,
    Base,
}

impl From<MermaidThemeType> for MermaidTheme {
    fn from(value: MermaidThemeType) -> Self {
        match value {
            MermaidThemeType::Default => MermaidTheme::Default,
            MermaidThemeType::Neutral => MermaidTheme::Neutral,
            MermaidThemeType::Dark => MermaidTheme::Dark,
            MermaidThemeType::Forest => MermaidTheme::Forest,
            MermaidThemeType::Base => MermaidTheme::Base,
        }
    }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let e = self.read_envelope()?;
        let output = match self.format_type {
            FormatType::Envelope => e.format(),
            FormatType::Tree => e.tree_format_opt(
                &TreeFormatOpts::default()
                    .hide_nodes(self.hide_nodes)
                    .digest_display(self.digest_format.into()),
            ),
            FormatType::Mermaid => e.mermaid_format_opt(
                &MermaidFormatOpts::default()
                    .hide_nodes(self.hide_nodes)
                    .theme(self.theme.into())
                    .monochrome(self.monochrome)
                    .orientation(self.orientation.into()),
            ),
            FormatType::Diag => e.diagnostic(),
            FormatType::Cbor => hex::encode(e.tagged_cbor_data()),
            FormatType::UR => e.ur_string(),
        };
        Ok(output)
    }
}
