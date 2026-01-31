use anyhow::Result;
use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_sskr_1() -> Result<()> {
    let result = run_cli(&["sskr", "split", ALICE_KNOWS_BOB_EXAMPLE])?;
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &result],
        indoc!(r#"
            ENCRYPTED [
                'sskrShare': SSKRShare
            ]
        "#)
    )?;
    let restored = run_cli(&["sskr", "join", &result])?;
    assert_eq!(restored, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

#[test]
fn test_sskr_2() -> Result<()> {
    let result = run_cli(&[
        "sskr",
        "split",
        "-t",
        "2",
        "-g",
        "2-of-3",
        "-g",
        "2-of-3",
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;
    let shares = result
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let indexes = [0, 1, 4, 5];
    let recovered_shares = indexes
        .iter()
        .map(|i| shares[*i].clone())
        .collect::<Vec<_>>();

    let mut args1 = vec!["sskr", "join"];
    args1.extend(recovered_shares.iter().map(|s| s.as_str()));
    let restored1 = run_cli(&args1)?;
    assert_eq!(restored1, ALICE_KNOWS_BOB_EXAMPLE);

    let restored2 =
        run_cli_stdin(&["sskr", "join"], &recovered_shares.join("\n"))?;
    assert_eq!(restored2, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}
