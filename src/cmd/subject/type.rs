use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    DataType, SubjectArgs, SubjectArgsLike, parse_data_type_to_envelope,
};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    subject_args: SubjectArgs,
}

impl SubjectArgsLike for CommandArgs {
    fn subject_type(&self) -> DataType { self.subject_args.subject_type() }

    fn subject_value(&self) -> Option<&str> {
        self.subject_args.subject_value()
    }

    fn ur_tag(&self) -> Option<u64> { self.subject_args.ur_tag() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        Ok(parse_data_type_to_envelope(
            self.subject_type(),
            Some(self.read_subject_value()?).as_deref(),
            self.ur_tag(),
        )?
        .ur_string())
    }
}
