use anyhow::{bail, Result};
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_components::PublicKeyBase;
use bc_envelope::prelude::*;

/// Verify a signature on the envelope using the provided public key base.
///
/// On success, print the original envelope so it can be piped to the next
/// operation. On failure, exit with an error condition.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Don't output the envelope's UR on success.
    #[arg(long, short, default_value = "false")]
    silent: bool,

    /// The minimum number of required valid signatures.
    #[arg(long, short, default_value = "1")]
    threshold: usize,

    /// The public keys (ur:pubkeys) to verify the envelope's signatures with.
    ///
    /// Can be provided multiple times to verify with multiple public keys.
    #[arg(long, short)]
    pubkeys: Vec<String>,

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
        if self.pubkeys.is_empty() {
            bail!("at least one pubkey must be provided");
        }
        let pubkeys: Vec<PublicKeyBase> = self
            .pubkeys
            .iter()
            .map(PublicKeyBase::from_ur_string)
            .collect::<Result<Vec<_>, _>>()?;
        envelope.clone().verify_signatures_from_threshold(&pubkeys, Some(self.threshold))?;
        Ok(if self.silent { "".to_string() } else { envelope.ur_string() })
    }
}
