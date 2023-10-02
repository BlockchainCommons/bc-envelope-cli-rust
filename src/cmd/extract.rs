use std::{ops::Deref, rc::Rc};

use anyhow::bail;
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
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.read_envelope()?;
        let string = match self.subject_type {
            SubjectType::Assertion => extract_assertion(envelope)?,
            SubjectType::Object => extract_object(envelope)?,
            SubjectType::Predicate => extract_predicate(envelope)?,

            SubjectType::Arid => envelope.extract_subject::<ARID>()?.ur_string(),
            SubjectType::AridHex => envelope.extract_subject::<ARID>()?.hex(),
            SubjectType::Bool => envelope.extract_subject::<bool>()?.to_string(),
            SubjectType::Cbor => extract_cbor(envelope)?,
            SubjectType::Data => hex::encode(envelope.extract_subject::<CBOR>()?.expect_byte_string()?),
            SubjectType::Date => envelope.extract_subject::<dcbor::Date>()?.to_string(),
            SubjectType::Digest => envelope.extract_subject::<Digest>()?.ur_string(),
            SubjectType::Envelope => envelope.subject().ur_string(),
            SubjectType::Known => extract_known(envelope)?,
            SubjectType::Number => envelope.extract_subject::<f64>()?.to_string(),
            SubjectType::String => envelope.extract_subject::<String>()?.deref().clone(),
            SubjectType::Ur => self.extract_ur(envelope)?,
            SubjectType::Uri => envelope.extract_subject::<URI>()?.to_string(),
            SubjectType::Uuid => envelope.extract_subject::<UUID>()?.to_string(),
            SubjectType::Wrapped => envelope.unwrap_envelope()?.ur_string(),
        };
        Ok(string)
    }
}

fn extract_assertion(envelope: Rc<Envelope>) -> anyhow::Result<String> {
    if let Some(assertion) = envelope.assertion() {
        let pred_obj = [assertion.clone().predicate().unwrap(), assertion.object().unwrap()];
        Ok(pred_obj.into_iter().map(|e| e.ur_string()).collect::<Vec<String>>().join("\n"))
    } else {
        bail!("Envelope is not an assertion.");
    }
}

fn extract_object(envelope: Rc<Envelope>) -> anyhow::Result<String> {
    if let Some(assertion) = envelope.assertion() {
        Ok(assertion.object().unwrap().ur_string())
    } else {
        bail!("Envelope is not an assertion.");
    }
}

fn extract_predicate(envelope: Rc<Envelope>) -> anyhow::Result<String> {
    if let Some(assertion) = envelope.assertion() {
        Ok(assertion.predicate().unwrap().ur_string())
    } else {
        bail!("Envelope is not an assertion.");
    }
}

impl CommandArgs {
    fn extract_ur(&self, envelope: Rc<Envelope>) -> anyhow::Result<String> {
        Ok(if let Some(cbor) = envelope.clone().subject().leaf() {
            if let CBOR::Tagged(tag, untagged_cbor) = cbor {
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

fn extract_known(envelope: Rc<Envelope>) -> anyhow::Result<String> {
    let _k = envelope.extract_subject::<KnownValue>()?;
    Ok(with_format_context!(|context| {
        envelope.subject().format_opt(Some(context))
    }))
}

fn extract_cbor(envelope: Rc<Envelope>) -> anyhow::Result<String> {
    Ok(if let Some(cbor) = envelope.leaf() {
        cbor.hex()
    } else if envelope.is_wrapped() {
        envelope.unwrap_envelope()?.cbor().hex()
    } else if let Some(known_value) = envelope.known_value() {
        known_value.cbor().hex()
    } else {
        bail!("No CBOR data found in envelope subject");
    })
}
