use anyhow::bail;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{envelope_args::{EnvelopeArgs, EnvelopeArgsLike}, utils::parse_digests};

/// Retrieve the assertion at the given index.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The target set of digests.
    ///
    /// One or more `ur:digest` or `ur:envelope` separated by a single space.
    target: String,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.read_envelope()?;
        let digests = parse_digests(&self.target)?;
        let proof: Option<Envelope> = envelope.proof_contains_set(&digests);
        if let Some(proof) = proof {
            Ok(proof.ur_string())
        } else {
            bail!("No proof found for target set");
        }
    }
}
