use std::ops::Deref;

use bc_components::ARID;
use bc_ur::UREncodable;
use clap::{Args, ValueEnum};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SubjectType {
    /// ARID: Apparently Random Identifier (ur:arid)
    Arid,

    Assertion,

    /// CBOR data in hex
    Cbor,

    /// Binary byte string in hex
    Data,

    /// Date (ISO 8601)
    Date,

    /// Cryptographic digest (ur:digest)
    Digest,

    /// Envelope (ur:envelope)
    Envelope,

    /// Known Value (number or string)
    Known,

    /// Numeric value,
    Number,

    Object,

    Predicate,

    /// UTF-8 String
    String,

    /// Uniform Resource (UR)
    Ur,

    /// URI
    Uri,

    /// UUID
    Uuid,

    /// Wrapped Envelope (ur:envelope)
    Wrapped,
}

/// Extract the subject of the input envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Subject type.
    #[arg(name = "TYPE")]
    subject_type: SubjectType,

    /// The expected tag for an extracted UR.
    #[arg(long)]
    tag: Option<u64>,

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
        let string = match self.subject_type {
            SubjectType::Arid => envelope.extract_subject::<ARID>()?.ur_string(),
            SubjectType::Assertion => todo!(),
            SubjectType::Cbor => todo!(),
            SubjectType::Data => todo!(),
            SubjectType::Date => todo!(),
            SubjectType::Digest => todo!(),
            SubjectType::Envelope => envelope.subject().ur_string(),
            SubjectType::Known => todo!(),
            SubjectType::Number => todo!(),
            SubjectType::Object => todo!(),
            SubjectType::Predicate => todo!(),
            SubjectType::String => envelope.extract_subject::<String>()?.deref().clone(),
            SubjectType::Ur => todo!(),
            SubjectType::Uri => todo!(),
            SubjectType::Uuid => todo!(),
            SubjectType::Wrapped => todo!(),
        };
        Ok(string)
    }
}
