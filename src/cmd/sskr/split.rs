pub use anyhow::Result;
use anyhow::bail;
use bc_components::{PublicKeys, SSKRGroupSpec, SSKRSpec, SymmetricKey};
use bc_envelope::prelude::*;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Split an envelope into several shares using SSKR.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The number of groups that must meet their threshold (1-16).
    ///
    /// Must be equal to or less than the number of groups.
    #[arg(short = 't', long, default_value = "1")]
    group_threshold: usize,

    /// A group specification (e.g., `2-of-3`).
    ///
    /// May be specified multiple times. Must be equal to or greater than the
    /// group threshold.
    #[arg(short = 'g', long = "group", default_value = "1-of-1")]
    groups: Vec<String>,

    /// The symmetric key to use for encryption.
    ///
    /// If not provided, an ephemeral key is generated.
    #[arg(short = 'k', long)]
    key: Option<String>,

    /// One or more public keys (ur:crypto-pubkeys) to also encrypt the message
    /// to.
    ///
    /// May be specified multiple times.
    #[arg(short = 'r', long = "recipient")]
    recipients: Vec<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;

        if self.group_threshold > self.groups.len() {
            bail!(
                "Group threshold must be less than or equal to the number of groups"
            );
        }

        let regex = regex::Regex::new(r"(\d{1,2})-of-(\d{1,2})")?;
        let groups: Vec<(usize, usize)> = self
            .groups
            .iter()
            .map(|group| {
                let matches = regex.captures(group).ok_or_else(|| {
                    anyhow::anyhow!("Invalid group specifier: {}", group)
                })?;
                let m = matches[1].parse()?;
                let n = matches[2].parse()?;
                Ok((m, n))
            })
            .collect::<Result<_>>()?;

        let content_key = match &self.key {
            Some(key) => SymmetricKey::from_ur_string(key)?,
            None => SymmetricKey::new(),
        };

        let wrapped = envelope.wrap();
        let encrypted = wrapped.encrypt_subject(&content_key)?;
        let group_specs: Vec<SSKRGroupSpec> = groups
            .iter()
            .map(|(m, n)| SSKRGroupSpec::new(*m, *n))
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from)?;
        let spec = SSKRSpec::new(self.group_threshold, group_specs)?;
        let grouped_shares = encrypted.sskr_split(&spec, &content_key)?;
        let flattened_shares =
            grouped_shares.into_iter().flatten().collect::<Vec<_>>();
        let flattened_shares = if self.recipients.is_empty() {
            flattened_shares
        } else {
            let recipients: Vec<PublicKeys> = self
                .recipients
                .iter()
                .map(|r| {
                    PublicKeys::from_ur_string(r).map_err(anyhow::Error::from)
                })
                .collect::<Result<_>>()?;
            flattened_shares
                .into_iter()
                .map(|share| {
                    let mut share = share;
                    for recipient in &recipients {
                        share = share.add_recipient(recipient, &content_key);
                    }
                    Ok(share)
                })
                .collect::<Result<Vec<_>>>()?
        };

        let output_shares = flattened_shares
            .iter()
            .map(|share| share.ur_string())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(output_shares)
    }
}
