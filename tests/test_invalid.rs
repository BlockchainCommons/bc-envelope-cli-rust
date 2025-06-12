use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_invalid_command() -> Result<()> {
    assert!(run_cli(&["invalid"]).is_err());
    Ok(())
}

#[test]
fn test_invalid_data() -> Result<()> {
    assert!(
        run_cli(&[
            "format",
            "ur:seed/oyadgdtokgdpwkrsonfdltvdwttsnddneonbmdbntakkss"
        ])
        .is_err()
    );
    Ok(())
}
