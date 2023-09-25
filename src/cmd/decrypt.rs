use anyhow::bail;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_components::{SymmetricKey, PrivateKeyBase};
use bc_envelope::prelude::*;

/// Decrypt the envelope's subject using the provided key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The symmetric key to use to decrypt the envelope's subject. (ur:crypto-key)
    #[arg(long, short, conflicts_with = "recipient")]
    key: Option<String>,

    /// The recipient to whom the envelope's subject should be decrypted. (ur:crypto-prvkeys)
    #[arg(long, short)]
    recipient: Option<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

// ```swift
// mutating func run() throws {
//     resetOutput()
//     try fill()
//     guard let envelope else {
//         throw EnvelopeToolError.missingArgument("envelope")
//     }
//     if let key {
//         printOut(try envelope.decryptSubject(with: key).ur)
//     } else if let recipient {
//         printOut(try envelope.decrypt(to: recipient).ur)
//     } else {
//         throw EnvelopeToolError.missingArgument("key or recipient")
//     }
// }
// ```

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;
        if let Some(key_ur) = &self.key {
            let key = SymmetricKey::from_ur_string(key_ur)?;
            Ok(envelope.decrypt_subject(&key)?.ur_string())
        } else if let Some(recipient_ur) = &self.recipient {
            let recipient = PrivateKeyBase::from_ur_string(recipient_ur)?;
            Ok(envelope.decrypt_to_recipient(&recipient)?.ur_string())
        } else {
            bail!("missing key or recipient");
        }
    }
}
