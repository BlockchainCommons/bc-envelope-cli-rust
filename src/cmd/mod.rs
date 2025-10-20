pub mod assertion;
pub mod attachment;
pub mod compress;
pub mod decompress;
pub mod decrypt;
pub mod digest;
pub mod elide;
pub mod encrypt;
pub mod export;
pub mod extract;
pub mod format;
pub mod generate;
pub mod import;
pub mod info;
pub mod pattern;
pub mod proof;
pub mod salt;
pub mod sign;
pub mod sskr;
pub mod subject;
pub mod verify;
pub mod xid;

pub const ASKPASS_HELP: &str =
    "Prompt for the password using an external program.";
pub const ASKPASS_LONG_HELP: &str = "If set, the password will be obtained by executing the program \
referenced by the SSH_ASKPASS environment variable. Attempts to fall \
back to common locations for askpass helpers if the variable is not set. \
If the program is not found, the password will be read from the \
terminal.";
