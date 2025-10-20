use std::collections::HashSet;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Find nodes matching obscuration types.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Match elided nodes.
    #[arg(long)]
    elided: bool,

    /// Match encrypted nodes.
    #[arg(long)]
    encrypted: bool,

    /// Match compressed nodes.
    #[arg(long)]
    compressed: bool,
}

impl CommandArgs {
    pub fn exec_with_envelope_and_target(
        &self,
        envelope: Envelope,
        target: Option<HashSet<Digest>>,
    ) -> Result<String> {
        let mut obscure_types = Vec::new();

        if self.elided {
            obscure_types.push(ObscureType::Elided);
        }

        if self.encrypted {
            obscure_types.push(ObscureType::Encrypted);
        }

        if self.compressed {
            obscure_types.push(ObscureType::Compressed);
        }

        let digests = envelope.nodes_matching(target.as_ref(), &obscure_types);
        super::output_digests(digests)
    }
}
