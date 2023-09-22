mod common;
use common::*;

#[test]
fn test_subject_assertion_known_known() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "assertion", "known", "isA", "known", "Seed"],
        None,
        "ur:envelope/oyadcsspsaykcfmh",
    )
}
