use clap::Args;
use crate::data_types::DataType;

pub trait SubjectArgsLike {
    fn subject_type(&self) -> DataType;
    fn subject_value(&self) -> &str;
    fn ur_tag(&self) -> Option<u64>;
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct SubjectArgs {
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

impl SubjectArgsLike for SubjectArgs {
    fn subject_type(&self) -> DataType {
        self.subject_type
    }

    fn subject_value(&self) -> &str {
        &self.subject_value
    }

    fn ur_tag(&self) -> Option<u64> {
        self.ur_tag
    }
}
