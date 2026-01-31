use anyhow::Result;
use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_salt() -> Result<()> {
    let salted = run_cli(&["salt", ALICE_KNOWS_BOB_EXAMPLE])?;

    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &salted],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
                'salt': Salt
            ]
        "#)
    )?;
    Ok(())
}
