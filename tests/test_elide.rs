use indoc::indoc;
use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_elide_1() -> Result<()> {
    let mut target = vec![];
    // Top level
    target.push(run_cli(&["digest", ALICE_KNOWS_BOB_EXAMPLE])?);
    // Subject
    target.push(run_cli_piped(&[&["extract", "envelope", ALICE_KNOWS_BOB_EXAMPLE], &["digest"]])?);
    // Assertion
    let assertion = run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    target.push(run_cli(&["digest", &assertion])?);
    // Object
    target.push(run_cli_piped(&[&["extract", "object", &assertion], &["digest"]])?);

    let digests = target.join(" ");
    let elided = run_cli(&["elide", "revealing", &digests, ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(
        elided,
        "ur:envelope/lftpsoihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpsoiafwjlidgraehkfp"
    );
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
            "Alice" [
                ELIDED: "Bob"
            ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_elide_2() -> Result<()> {
    let target = [run_cli_piped(&[&["subject", "type", "string", "knows"], &["digest"]])?];
    let digests = target.join(" ");
    let elided = run_cli(&["elide", "removing", &digests, ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(
        elided,
        "ur:envelope/lftpsoihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpsoiafwjlidgraehkfp"
    );
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
            "Alice" [
                ELIDED: "Bob"
            ]
        "#)
    )?;
    Ok(())
}
