use std::collections::HashSet;

use clap::{Args, ValueEnum};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_envelope::prelude::*;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Depth {
    /// Return just the envelope's top digest.
    Top,
    /// Return the digests necessary to reveal the subject.
    Shallow,
    /// Return the digests needed to reveal the entire contents of the envelope.
    Deep,
}

/// Print the envelope's digest.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[arg(long = "depth", default_value = "top")]
    depth: Depth,

    #[arg(long = "hex", default_value = "false")]
    hex: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    // ```swift
    // mutating func run() throws {
    //     resetOutput()
    //     try fill()
    //     guard let envelope else {
    //         throw EnvelopeToolError.missingArgument("envelope")
    //     }
    //     let digests: Set<Digest>
    //     switch depth {
    //     case .top:
    //         digests = [envelope.digest]
    //     case .shallow:
    //         digests = envelope.shallowDigests
    //     case .deep:
    //         digests = envelope.deepDigests
    //     }
    //     printOut(digests.sorted().map { isHex ? $0.hex : $0.ur.string }.joined(separator: " "))
    // }
    // ```

    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;
        let digests: HashSet<Digest> = match self.depth {
            Depth::Top => vec![envelope.digest().into_owned()].into_iter().collect::<HashSet<_>>(),
            Depth::Shallow => envelope.shallow_digests(),
            Depth::Deep => envelope.deep_digests(),
        };
        let mut ordered_digests = digests.iter().cloned().collect::<Vec<_>>();
        ordered_digests.sort();
        let output = ordered_digests.iter().map(|d| if self.hex { d.hex() } else { d.ur_string() }).collect::<Vec<String>>().join(" ");
        Ok(output)
    }
}
