use anyhow::{Result, bail};
use bc_components::{PrivateKeyBase, PrivateKeys, SymmetricKey};
use bc_envelope::prelude::*;
use clap::Args;

use super::{ASKPASS_HELP, ASKPASS_LONG_HELP};
use crate::{
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
    utils::read_password,
};

/// Decrypt the envelope's subject.
///
/// The subject can be decrypted using one of the following:
///     - A symmetric key (ur:crypto-key) (the content key) that was used to
///       encrypt the subject,
///     - A password that was used to lock the content key,
///     - A recipient's private key (ur:crypto-prvkey-base or ur:crypto-prvkeys)
///       that was used to lock the content key,
///     - An SSH identity that was used to lock the content key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The symmetric key to use to decrypt the envelope's subject.
    /// (ur:crypto-key)
    ///
    /// May not be used with the `--password` or `--recipient` options.
    #[arg(
        long,
        short,
        conflicts_with = "recipient",
        conflicts_with = "password",
        conflicts_with = "ssh_id"
    )]
    key: Option<String>,

    /// The password to derive the symmetric key.
    ///
    /// If not provided, will be prompted. May not be used with the `--key` or
    /// `--recipient` options.
    #[arg(long, short, num_args(0..=1), conflicts_with = "key", conflicts_with = "recipient", conflicts_with = "ssh_id")]
    password: Option<Option<String>>,

    /// Use the `SSH_ASKPASS` environment variable to read the password.
    ///
    /// This option requires the `--password` option to be set.
    #[arg(long, requires = "password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,

    /// The recipient to whom the envelope's subject should be decrypted.
    /// (ur:crypto-prvkey-base or ur:crypto-prvkeys)
    #[arg(
        long,
        short,
        conflicts_with = "key",
        conflicts_with = "password",
        conflicts_with = "ssh_id"
    )]
    recipient: Option<String>,

    /// The SSH identity to use to decrypt the envelope's subject.
    ///
    /// - If provided, the SSH agent will be used to find the Ed25519 key
    ///   associated with the identity.
    /// - If provided but empty, the first available Ed25519 key in the SSH
    ///   agent is used.
    #[arg(
        long,
        short,
        conflicts_with = "key",
        conflicts_with = "password",
        conflicts_with = "recipient"
    )]
    ssh_id: Option<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Read the envelope from the specified file or stdin
        let envelope = self.read_envelope()?;

        if let Some(key_ur) = &self.key {
            // If a content key is provided, decrypt the subject using it
            let key = SymmetricKey::from_ur_string(key_ur)?;
            let decrypt_subject = envelope.decrypt_subject(&key);
            match decrypt_subject {
                Err(_) => bail!("decrypt failed"),
                Ok(subject) => Ok(subject.ur_string()),
            }
        } else if let Some(password) = &self.password {
            // If a password is provided, unlock the subject using it
            if !envelope.is_locked_with_password() {
                bail!("envelope is not locked with a password");
            }
            let password = read_password(
                "Decryption password:",
                password.as_deref(),
                self.askpass,
            )?;
            Ok(envelope.unlock_subject(password.as_bytes())?.ur_string())
        } else if let Some(recipient_ur) = &self.recipient {
            // If a recipient's private key is provided, decrypt the subject
            // using it. Try to parse as PrivateKeys first, then PrivateKeyBase.
            if let Ok(recipient) = PrivateKeys::from_ur_string(recipient_ur) {
                Ok(envelope
                    .decrypt_subject_to_recipient(&recipient)?
                    .ur_string())
            } else if let Ok(recipient) =
                PrivateKeyBase::from_ur_string(recipient_ur)
            {
                Ok(envelope
                    .decrypt_subject_to_recipient(&recipient)?
                    .ur_string())
            } else {
                bail!(
                    "invalid recipient private key: must be ur:crypto-prvkeys or ur:crypto-prvkey-base"
                )
            }
        } else if let Some(ssh_id) = &self.ssh_id {
            // If an SSH identity is provided, decrypt the subject using the SSH
            // agent
            if !envelope.is_locked_with_ssh_agent() {
                bail!("envelope is not locked with an SSH agent");
            }
            Ok(envelope.unlock_subject(ssh_id)?.ur_string())
        } else {
            bail!(
                "missing unlock method: either a symmetric key, password, recipient's private key, or SSH identity must be provided"
            );
        }
    }
}
