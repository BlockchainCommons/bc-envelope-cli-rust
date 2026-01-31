use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike, read_envelope, xid::XIDDocumentReadable,
};

/// Find edges in a XID document by criteria.
///
/// Filters can be combined. Only edges matching all specified criteria
/// are returned.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Filter by edge type (isA assertion object, as a UR).
    #[arg(long = "is-a")]
    is_a: Option<String>,

    /// Filter by source XID (as a UR).
    #[arg(long)]
    source: Option<String>,

    /// Filter by target XID (as a UR).
    #[arg(long)]
    target: Option<String>,

    /// Filter by edge subject identifier (as a UR).
    #[arg(long)]
    subject: Option<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document()?;
        let envelope: Envelope = xid_document.into();

        let is_a_env = self
            .is_a
            .as_deref()
            .map(|s| read_envelope(Some(s)))
            .transpose()?;
        let source_env = self
            .source
            .as_deref()
            .map(|s| read_envelope(Some(s)))
            .transpose()?;
        let target_env = self
            .target
            .as_deref()
            .map(|s| read_envelope(Some(s)))
            .transpose()?;
        let subject_env = self
            .subject
            .as_deref()
            .map(|s| read_envelope(Some(s)))
            .transpose()?;

        let matching = envelope.edges_matching(
            is_a_env.as_ref(),
            source_env.as_ref(),
            target_env.as_ref(),
            subject_env.as_ref(),
        )?;

        let result = matching
            .iter()
            .map(|e| e.ur_string())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(result)
    }
}
