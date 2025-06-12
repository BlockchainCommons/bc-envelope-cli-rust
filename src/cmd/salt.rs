use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Add random salt to the envelope.
///
/// If the size of the salt is not specified, the amount is random. For small
/// objects, the number of bytes added will generally be from 8...16. For larger
/// objects the number of bytes added will generally be from 5%...25% of the
/// size of the object.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The size of the salt to add to the envelope.
    #[arg(long, short)]
    size: Option<usize>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        Ok((if let Some(size) = self.size {
            envelope.add_salt_with_len(size)?
        } else {
            envelope.add_salt()
        })
        .ur_string())
    }
}
