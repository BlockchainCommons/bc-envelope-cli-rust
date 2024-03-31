mod common;
use common::*;

#[test]
fn test_invalid_command() -> anyhow::Result<()> {
    assert!(run_cli(&["invalid"]).is_err());
    Ok(())
}

#[test]
fn test_invalid_data() -> anyhow::Result<()> {
    assert!(run_cli(&[
        "format",
        "ur:seed/oyadgdtokgdpwkrsonfdltvdwttsnddneonbmdbntakkss"
    ])
    .is_err());
    Ok(())
}
