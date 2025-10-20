use std::collections::HashSet;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Decompress nodes.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {}

impl CommandArgs {
    pub fn exec_with_envelope_and_target(
        &self,
        envelope: Envelope,
        target: Option<HashSet<Digest>>,
    ) -> Result<String> {
        let result = envelope.walk_decompress(target.as_ref());
        Ok(result.ur_string())
    }
}
