use anyhow::{ bail, Result };
use clap::Args;

use crate::envelope_args::{ EnvelopeArgs, EnvelopeArgsLike };
use bc_components::{ SymmetricKey, PublicKeys };
use bc_envelope::prelude::*;

/// Encrypt the envelope's subject using the provided key.
///
/// If the key is not provided and recipients are provided, an ephemerally-generated key is used.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The symmetric key to use to encrypt the envelope's subject. (ur:crypto-key)
    ///
    /// If not provided and recipients are provided, an ephemerally-generated key is used.
    #[arg(long, short)]
    key: Option<String>,

    /// The recipients to whom the envelope's subject should be encrypted. (ur:crypto-pubkeys)
    ///
    /// May be provided multiple times.
    #[arg(long, short)]
    recipient: Vec<String>,

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

        // Convert recipients to `PublicKeys`.
        let recipients = self.recipient
            .iter()
            .map(|s| PublicKeys::from_ur_string(s).map_err(anyhow::Error::from))
            .collect::<Result<Vec<PublicKeys>, anyhow::Error>>()?;

        // Get the key
        let key = match self.key {
            Some(ref key) => SymmetricKey::from_ur_string(key)?,
            None => {
                if recipients.is_empty() {
                    bail!("Must provide either a key or recipients.");
                }
                SymmetricKey::new()
            }
        };

        // Encrypt the subject.
        let mut encrypted_envelope = envelope.encrypt_subject(&key)?;
        // If there are recipients, add them.
        for recipient in recipients {
            encrypted_envelope = encrypted_envelope.add_recipient(&recipient, &key);
        }
        Ok(encrypted_envelope.ur_string())
    }
}
