use bc_ur::UREncodable;
use clap::Args;

use crate::data_types::{DataType, parse_data_type_to_envelope};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Subject type.
    #[arg(name = "TYPE")]
    subject_type: DataType,
    /// Subject value.
    #[arg(name = "VALUE")]
    subject_value: String,
    /// The integer tag for an enclosed UR.
    #[arg(long)]
    ur_tag: Option<u64>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        Ok(parse_data_type_to_envelope(self.subject_type, &self.subject_value, self.ur_tag)?.ur_string())
    }
}
