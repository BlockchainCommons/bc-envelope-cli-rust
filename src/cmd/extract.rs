use anyhow::{bail, Result};
use bc_components::{ARID, URI, UUID, Digest, with_tags, tags::ENVELOPE};
use bc_envelope::prelude::*;
use clap::{Args, ValueEnum};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SubjectType {
    Assertion,

    Object,

    Predicate,

    /// ARID: Apparently Random Identifier (ur:arid)
    Arid,

    /// ARID: Apparently Random Identifier (hex)
    AridHex,

    /// Boolean value
    Bool,

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

    /// The type for an extracted UR.
    #[arg(long)]
    ur_type: Option<String>,

    /// The expected tag for an extracted UR.
    #[arg(long)]
    ur_tag: Option<u64>,

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
        let string = match self.subject_type {
            SubjectType::Assertion => extract_assertion(envelope)?,
            SubjectType::Object => extract_object(envelope)?,
            SubjectType::Predicate => extract_predicate(envelope)?,

            SubjectType::Arid => envelope.extract_subject::<ARID>()?.ur_string(),
            SubjectType::AridHex => envelope.extract_subject::<ARID>()?.hex(),
            SubjectType::Bool => envelope.extract_subject::<bool>()?.to_string(),
            SubjectType::Cbor => extract_cbor_string(envelope)?,
            SubjectType::Data => hex::encode(envelope.subject().try_leaf()?.to_cbor().try_into_byte_string()?),
            SubjectType::Date => envelope.extract_subject::<dcbor::Date>()?.to_string(),
            SubjectType::Digest => envelope.extract_subject::<Digest>()?.ur_string(),
            SubjectType::Envelope => envelope.subject().ur_string(),
            SubjectType::Known => extract_known_value_string(envelope)?,
            SubjectType::Number => envelope.extract_subject::<f64>()?.to_string(),
            SubjectType::String => envelope.extract_subject::<String>()?,
            SubjectType::Ur => self.extract_ur(envelope)?,
            SubjectType::Uri => envelope.extract_subject::<URI>()?.to_string(),
            SubjectType::Uuid => envelope.extract_subject::<UUID>()?.to_string(),
            SubjectType::Wrapped => envelope.unwrap_envelope()?.ur_string(),
        };
        Ok(string)
    }
}

fn extract_assertion(envelope: Envelope) -> Result<String> {
    if let Some(assertion) = envelope.as_assertion() {
        let pred_obj = [assertion.clone().as_predicate().unwrap(), assertion.as_object().unwrap()];
        Ok(pred_obj.into_iter().map(|e| e.ur_string()).collect::<Vec<String>>().join("\n"))
    } else {
        bail!("Envelope is not an assertion.");
    }
}

fn extract_object(envelope: Envelope) -> Result<String> {
    if let Some(assertion) = envelope.as_assertion() {
        Ok(assertion.as_object().unwrap().ur_string())
    } else {
        bail!("Envelope is not an assertion.");
    }
}

fn extract_predicate(envelope: Envelope) -> Result<String> {
    if let Some(assertion) = envelope.as_assertion() {
        Ok(assertion.as_predicate().unwrap().ur_string())
    } else {
        bail!("Envelope is not an assertion.");
    }
}

impl CommandArgs {
    fn extract_ur(&self, envelope: Envelope) -> Result<String> {
        Ok(if let Some(cbor) = envelope.clone().subject().as_leaf() {
            if let CBORCase::Tagged(tag, untagged_cbor) = cbor.into_case() {
                let known_tag = with_tags!(|tags: &dyn dcbor::TagsStoreTrait| {
                    tags.tag_for_value(tag.value())
                });
                // Default to the provided ur_type if there is one.
                let mut ur_type: Option<String> = self.ur_type.clone();
                // If there is a known_tag and it has a name, then use that as the ur_type.
                if let Some(known_tag) = known_tag {
                    if let Some(name) = known_tag.name() {
                        ur_type = Some(name.to_string());
                    }
                }
                // If there is no ur_type, then error.
                if ur_type.is_none() {
                    bail!("UR type required");
                }
                bc_ur::UR::new(ur_type.unwrap(), untagged_cbor)?.to_string()
            } else {
                bail!("Can't convert to UR: CBOR in envelope subject has no tag");
            }
        } else if envelope.is_wrapped() {
            if self.ur_tag.is_some() || self.ur_type.is_some() {
                if self.ur_tag != Some(ENVELOPE.value()) {
                    bail!("UR tag mismatch");
                }
                if self.ur_type != Some(ENVELOPE.name().unwrap()) {
                    bail!("UR type mismatch");
                }
            }
            envelope.unwrap_envelope()?.ur_string()
        } else {
            bail!("No CBOR data found in envelope subject");
        })
    }
}

fn extract_known_value_string(envelope: Envelope) -> Result<String> {
    let _k = envelope.extract_subject::<KnownValue>()?;
    Ok(envelope.subject().format())
}

fn extract_cbor_string(envelope: Envelope) -> Result<String> {
    let subject = &envelope.subject();
    Ok(if let Some(cbor) = subject.as_leaf() {
        cbor.hex()
    } else if subject.is_wrapped() {
        envelope.unwrap_envelope()?.to_cbor().hex()
    } else if let Some(known_value) = subject.as_known_value() {
        known_value.to_cbor().hex()
    } else {
        bail!("No CBOR data found in envelope subject");
    })
}
