use anyhow::{bail, Result};
use bc_envelope::prelude::*;
use clap::Args;

use crate::{envelope_args::{EnvelopeArgs, EnvelopeArgsLike}, utils::parse_digests};

/// Confirm that an elided envelope contains a target digest using a proof.
///
/// On success, print the original envelope so it can be piped to the next
/// operation. On failure, exit with an error condition.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The proof envelope to use.
    proof: String,

    /// The target set of digests.
    ///
    /// One or more `ur:digest` or `ur:envelope` separated by a single space.
    target: String,

    #[arg(long, short = 's', default_value = "false")]
    silent: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let proof = Envelope::from_ur_string(&self.proof)?;
        let digests = parse_digests(&self.target)?;
        if !envelope.clone().confirm_contains_set(&digests, &proof) {
            bail!("Proof does not confirm target");
        }
        Ok(if self.silent { "".to_string() } else { envelope.ur_string() })
    }
}
