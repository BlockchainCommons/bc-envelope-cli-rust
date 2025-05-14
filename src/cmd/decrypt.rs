use anyhow::{ bail, Result };
use clap::Args;

use crate::{ envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }, utils::read_password };
use bc_components::{ SymmetricKey, PrivateKeyBase };
use bc_envelope::prelude::*;

use super::{ ASKPASS_HELP, ASKPASS_LONG_HELP };

/// Decrypt the envelope's subject using the provided key or password.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The symmetric key to use to decrypt the envelope's subject. (ur:crypto-key)
    #[arg(long, short, conflicts_with = "recipient", conflicts_with = "password")]
    key: Option<String>,

    /// The password to derive the symmetric key.
    ///
    /// If not provided, will be prompted. May not be used with the `--key` option.
    #[arg(long, short, conflicts_with = "key", conflicts_with = "recipient", num_args(0..=1))]
    password: Option<Option<String>>,

    #[arg(long, requires = "password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,

    /// The recipient to whom the envelope's subject should be decrypted. (ur:crypto-prvkey-base)
    #[arg(long, short, conflicts_with = "key", conflicts_with = "password")]
    recipient: Option<String>,

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
        if let Some(key_ur) = &self.key {
            let key = SymmetricKey::from_ur_string(key_ur)?;
            Ok(envelope.decrypt_subject(&key)?.ur_string())
        } else if let Some(password) = &self.password {
            let password = read_password(
                "Decryption password:",
                password.as_deref(),
                self.askpass
            )?;
            Ok(envelope.unlock_subject(password.as_bytes())?.ur_string())
        } else if let Some(recipient_ur) = &self.recipient {
            let recipient = PrivateKeyBase::from_ur_string(recipient_ur)?;
            Ok(envelope.decrypt_subject_to_recipient(&recipient)?.ur_string())
        } else {
            bail!("missing key or recipient");
        }
    }
}
