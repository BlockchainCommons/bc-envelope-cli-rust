use std::{rc::Rc, str::FromStr};

use bc_components::{ARID, Digest};
use bc_envelope::{preamble::*, format::FormatContext};
use bc_ur::preamble::*;
use clap::Args;

use crate::data_types::DataType;

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Item type.
    item: DataType,
    /// Item value.
    value: String,
    /// The integer tag for an enclosed UR.
    tag: Option<u64>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope: Rc<Envelope> = match self.item {
            DataType::Arid => parse_arid(&self.value)?,
            DataType::Cbor => parse_cbor(&self.value)?,
            DataType::Data => parse_data(&self.value)?,
            DataType::Date => parse_date(&self.value)?,
            DataType::Digest => parse_digest(&self.value)?,
            DataType::Envelope => parse_envelope(&self.value)?,
            DataType::Known => parse_known_value(&self.value)?,
            DataType::Number => parse_number(&self.value)?,
            DataType::String => parse_string(&self.value)?,
            DataType::Ur => parse_ur(&self.value, self.tag)?,
            DataType::Uri => parse_uri(&self.value)?,
            DataType::Uuid => parse_uuid(&self.value)?,
            DataType::Wrapped => parse_wrapped_envelope(&self.value)?,
        };
        Ok(envelope.ur_string())
    }
}

/// Parse an ARID from a string.
///
/// Accepts either a hex-encoded ARID or a UR-encoded ARID.
fn parse_arid(s: &str) -> anyhow::Result<Rc<Envelope>> {
    if let Ok(hex) = hex::decode(s) {
        let arid = ARID::from_data_ref(&hex)?;
        Ok(Envelope::new(arid))
    } else if let Ok(arid) = ARID::from_ur_string(s) {
        Ok(Envelope::new(arid))
    } else {
        anyhow::bail!("Invalid ARID")
    }
}

/// Parse a CBOR envelope from a string.
fn parse_cbor(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let cbor = CBOR::from_hex(s)?;
    Ok(Envelope::new(cbor))
}

/// Parse a bytestring from a string.
fn parse_data(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let data = CBOR::byte_string(hex::decode(s)?);
    Ok(Envelope::new(data))
}

/// Parse a Date from a string.
fn parse_date(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let date = dcbor::Date::new_from_string(s)?;
    Ok(Envelope::new(date))
}

/// Parse a Digest from a ur:digest string.
fn parse_digest(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let digest = Digest::from_ur_string(s)?;
    Ok(Envelope::new(digest))
}

/// Parse an Envelope from a string.
fn parse_envelope(s: &str) -> anyhow::Result<Rc<Envelope>> {
    Ok(Rc::new(Envelope::from_ur_string(s)?))
}

/// Parse a KnownValue from a string.
///
/// Accepts either a integer or a string.
fn parse_known_value(s: &str) -> anyhow::Result<Rc<Envelope>> {
    if let Ok(number) = s.parse::<u64>() {
        Ok(Envelope::new(KnownValue::new(number)))
    } else {
        with_format_context!(|context: &FormatContext| {
            let store = context.known_values();
            if let Some(known_value) = bc_envelope::known_values::KnownValuesStore::known_value_for_name(s, Some(store)) {
                Ok(Envelope::new(known_value))
            } else {
                anyhow::bail!("Unknown known value")
            }
        })
    }
}

/// Parse a numeric value from a string.
fn parse_number(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let number = s.parse::<f64>()?;
    Ok(Envelope::new(number))
}

/// Parse a string from a string.
fn parse_string(s: &str) -> anyhow::Result<Rc<Envelope>> {
    Ok(Envelope::new(s))
}

/// Parse a UR from a string.
///
/// - If the UR is a ur:envelope, acts like `type envelope`.
/// - If the UR is another type, then it attempts to look up the CBOR tag for the type and
/// encodes the envelope with the tagged CBOR content of the UR.
/// - If the UR is of an unknown type, then the --tag option must be used to specify the CBOR tag
/// to use.
fn parse_ur(s: &str, tag_value: Option<u64>) -> anyhow::Result<Rc<Envelope>> {
    let ur = UR::from_ur_string(s)?;
    if ur.ur_type() == "envelope" {
        let envelope = Rc::new(Envelope::from_ur(&ur)?);
        Ok(envelope.wrap_envelope())
    } else {
        let cbor_tag = with_format_context!(|context: &FormatContext| {
            let store = context.tags();
            if let Some(tag) = store.tag_for_name(ur.ur_type()) {
                Some(tag)
            } else {
                tag_value.map(Tag::new)
            }
        });
        if let Some(cbor_tag) = cbor_tag {
            let cbor = ur.cbor();
            let content_cbor = CBOR::tagged_value(cbor_tag, cbor);
            Ok(Envelope::new(content_cbor))
        } else {
            anyhow::bail!("Unknown UR type")
        }
    }
}

/// Parse a URI from a string.
fn parse_uri(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let uri = bc_components::URI::new(s)?;
    Ok(Envelope::new(uri))
}

/// Parse a UUID from a string.
fn parse_uuid(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let uuid = bc_components::UUID::from_str(s)?;
    Ok(Envelope::new(uuid))
}

/// Parse a wrapped envelope from a ur:envelope string.
fn parse_wrapped_envelope(s: &str) -> anyhow::Result<Rc<Envelope>> {
    let envelope = Rc::new(Envelope::from_ur_string(s)?);
    Ok(envelope.wrap_envelope())
}
