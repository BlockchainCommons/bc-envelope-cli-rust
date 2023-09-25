use anyhow::bail;
use bc_components::{SymmetricKey, SSKRSpec, SSKRGroupSpec, SSKRError, PublicKeyBase};
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_envelope::prelude::*;

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
    /// May be specified multiple times. Must be equal to or greater than the group threshold.
    #[arg(short = 'g', long = "group", default_value = "1-of-1")]
    groups: Vec<String>,

    /// The symmetric key to use for encryption.
    ///
    /// If not provided, an ephemeral key is generated.
    #[arg(short = 'k', long)]
    key: Option<String>,

    /// One or more public keys (ur:crypto-pubkeys) to also encrypt the message to.
    ///
    /// May be specified multiple times.
    #[arg(short = 'r', long = "receipient")]
    recipients: Vec<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

/// ```swift
// mutating func run() throws {
//     resetOutput()
//     try fill()

//     guard let envelope else {
//         throw EnvelopeToolError.missingArgument("envelope")
//     }

//     guard groupThreshold <= group.count else {
//         throw EnvelopeToolError.invalidGroupThreshold(groupThreshold)
//     }

//     let regex = try ~/#"(\d{1,2})-of-(\d{1,2})"#
//     let groups: [(Int, Int)] = try group.map {
//         guard
//             let matches = regex.matchedSubstrings(in: $0),
//             matches.count == 2,
//             let m = Int(matches[0]),
//             let n = Int(matches[1])
//         else {
//             throw EnvelopeToolError.invalidGroupSpecifier($0)
//         }
//         return (m, n)
//     }

//     let contentKey = key ?? SymmetricKey()
//     let wrapped = envelope.wrap()
//     let encrypted = try wrapped.encryptSubject(with: contentKey)
//     let groupedShares = encrypted.split(groupThreshold: groupThreshold, groups: groups, contentKey: contentKey)
//     var flattenedShares = groupedShares.flatMap { $0 }
//     if !recipient.isEmpty {
//         flattenedShares = flattenedShares.map {
//             var share = $0
//             for recipientPubliKey in recipient {
//                 share = share.addRecipient(recipientPubliKey, contentKey: contentKey)
//             }
//             return share
//         }
//     }
//     let outputShares = flattenedShares.map {
//         $0.ur.string
//     }.joined(separator: " ")
//     printOut(outputShares)
// }
/// ```

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;

        if self.group_threshold > self.groups.len() {
            bail!("Group threshold must be less than or equal to the number of groups");
        }

        let regex = regex::Regex::new(r"(\d{1,2})-of-(\d{1,2})")?;
        let groups: Vec<(usize, usize)> = self
            .groups
            .iter()
            .map(|group| {
                let matches = regex
                    .captures(group)
                    .ok_or_else(|| anyhow::anyhow!("Invalid group specifier: {}", group))?;
                let m = matches[1].parse()?;
                let n = matches[2].parse()?;
                Ok((m, n))
            })
            .collect::<anyhow::Result<_>>()?;

        let content_key = match &self.key {
            Some(key) => SymmetricKey::from_ur_string(key)?,
            None => SymmetricKey::new(),
        };

        let wrapped = envelope.wrap_envelope();
        let encrypted = wrapped.encrypt_subject(&content_key)?;
        let group_spec_results: Vec<Result<SSKRGroupSpec, SSKRError>> = groups
            .iter()
            .map(|(m, n)| SSKRGroupSpec::new(*m, *n))
            .collect();
        let group_specs = group_spec_results
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        let spec = SSKRSpec::new(self.group_threshold, group_specs)?;
        let grouped_shares = encrypted.sskr_split(&spec, &content_key)?;
        let flattened_shares = grouped_shares.into_iter().flatten().collect::<Vec<_>>();
        let flattened_shares = if self.recipients.is_empty() {
            flattened_shares
        } else {
            let recipients: Vec<PublicKeyBase> = self
                .recipients
                .iter()
                .map(PublicKeyBase::from_ur_string)
                .collect::<anyhow::Result<_>>()?;
            flattened_shares
                .into_iter()
                .map(|share| {
                    let mut share = share;
                    for recipient in &recipients {
                        share = share.add_recipient(recipient, &content_key);
                    }
                    Ok(share)
                })
                .collect::<anyhow::Result<Vec<_>>>()?
        };

        let output_shares = flattened_shares
            .iter()
            .map(|share| share.ur_string())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(output_shares)
    }
}
