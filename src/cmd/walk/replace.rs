use std::collections::HashSet;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Replace nodes matching target digests with a replacement envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Replacement envelope (UR format).
    #[arg(required = true)]
    replacement: String,
}

impl CommandArgs {
    pub fn exec_with_envelope_and_target(
        &self,
        envelope: Envelope,
        target_digests: Option<HashSet<Digest>>,
    ) -> Result<String> {
        let target_digests = target_digests.ok_or_else(|| {
            anyhow::anyhow!(
                "walk replace requires --target digests to specify which nodes to replace"
            )
        })?;

        let replacement = Envelope::from_ur_string(&self.replacement)?;
        let result = envelope.walk_replace(&target_digests, &replacement)?;
        Ok(result.ur_string())
    }
}
