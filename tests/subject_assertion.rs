use indoc::indoc;

mod common;
use common::run_cli;

#[test]
fn test_subject_assertion_known_known() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "assertion",
        "known", "isA",
        "known", "Seed",
    ], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/oyadcsspsaykcfmh
        "#}
    );
    Ok(())
}
