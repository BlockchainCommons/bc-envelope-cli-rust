use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;
use dcbor::prelude::Date;

/// Generate a private key base.
///
/// Generated randomly, or deterministically if a seed is provided.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The seed from which to derive the private key base (ur:seed or
    /// ur:envelope).
    #[arg(long, short)]
    seed: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        if let Some(seed_ur) = &self.seed {
            let seed = parse_seed_input(seed_ur)?;
            let private_key_base =
                bc_components::PrivateKeyBase::new_with_provider(seed);
            Ok(private_key_base.ur_string())
        } else {
            let private_key_base = bc_components::PrivateKeyBase::new();
            Ok(private_key_base.ur_string())
        }
    }
}

fn parse_seed_input(input: &str) -> Result<bc_components::Seed> {
    match bc_components::Seed::from_ur_string(input) {
        Ok(seed) => Ok(seed),
        Err(_) => seed_from_envelope(input),
    }
}

fn seed_from_envelope(input: &str) -> Result<bc_components::Seed> {
    let envelope = Envelope::from_ur_string(input)?;
    envelope.check_type(&known_values::SEED_TYPE)?;

    let data = envelope
        .subject()
        .try_leaf()?
        .try_into_byte_string()?
        .to_vec();
    let name = envelope
        .extract_optional_object_for_predicate::<String>(known_values::NAME)?
        .unwrap_or_default();
    let note = envelope
        .extract_optional_object_for_predicate::<String>(known_values::NOTE)?
        .unwrap_or_default();
    let creation_date = envelope
        .extract_optional_object_for_predicate::<Date>(known_values::DATE)?
        .map(|date| date.as_ref().clone());

    bc_components::Seed::new_opt(
        data,
        optional_non_empty(name),
        optional_non_empty(note),
        creation_date,
    )
    .map_err(Into::into)
}

fn optional_non_empty(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}
