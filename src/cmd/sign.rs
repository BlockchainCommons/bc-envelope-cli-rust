use anyhow::bail;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_components::PrivateKeyBase;
use bc_envelope::prelude::*;

/// Sign the envelope subject with the provided private key base.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The private key base to sign the envelope subject with (ur:prv).
    ///
    /// Can be provided multiple times to sign with multiple private keys.
    #[arg(long, short)]
    prvkeys: Vec<String>,

    /// An optional note to add to the envelope.
    #[arg(long, short)]
    note: Option<String>,

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
        let envelope = self.get_envelope()?;
        if self.prvkeys.is_empty() {
            bail!("at least one prvkey must be provided");
        }
        let prvkeys: Vec<PrivateKeyBase> = self
            .prvkeys
            .iter()
            .map(PrivateKeyBase::from_ur_string)
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(note) = &self.note {
            if prvkeys.len() != 1 {
                bail!("can only add a note on a single signature");
            }
            Ok(envelope.sign_with_opt(&prvkeys[0], Some(note), []).ur_string())
        } else {
            Ok(envelope.sign_with_keys(&prvkeys).ur_string())
        }
    }
}
