use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_subject_assertion_known_known() -> Result<()> {
    run_cli_expect(
        &["subject", "assertion", "known", "isA", "known", "Seed"],
        "ur:envelope/oyadcsspsaykcfmh",
    )
}
