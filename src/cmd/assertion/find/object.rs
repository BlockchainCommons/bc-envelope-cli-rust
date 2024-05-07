use bc_envelope::prelude::*;
use clap::Args;
use anyhow::Result;

use crate::{subject_args::{SubjectArgs, SubjectArgsLike}, envelope_args::{EnvelopeArgs, EnvelopeArgsLike}, data_types::{DataType, parse_data_type_to_envelope}};

/// Find all assertions having the given object.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    subject_args: SubjectArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl SubjectArgsLike for CommandArgs {
    fn subject_type(&self) -> DataType {
        self.subject_args.subject_type()
    }

    fn subject_value(&self) -> Option<&str> {
        self.subject_args.subject_value()
    }

    fn ur_tag(&self) -> Option<u64> {
        self.subject_args.ur_tag()
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let object = parse_data_type_to_envelope(self.subject_type(), self.subject_value(), self.ur_tag())?;
        let assertions = envelope.clone().assertions();
        let result = assertions
            .iter().filter(|&a| a.clone().as_object().unwrap().digest() == object.digest()).cloned().collect::<Vec<_>>()
            .iter().map(|a| a.ur_string()).collect::<Vec<String>>().join("\n");
        Ok(result)
    }
}
