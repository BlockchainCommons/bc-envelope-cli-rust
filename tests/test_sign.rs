use std::process::{Command, Stdio};

use anyhow::Result;
use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_sign() -> Result<()> {
    let prvkeys = "ur:crypto-prvkeys/lftansgohdcxpfsndiahcxsfrhjoltglmebwwnnstovocffejytdbwihdkrtdykebkiebglbtteetansgehdcxvsdapeurgauovlbsvdfhvdcevywlptspfgnejpbksadehkhkfzehhfaysrsrbsdstbtagyeh";
    let signed =
        run_cli(&["sign", "--signer", prvkeys, ALICE_KNOWS_BOB_EXAMPLE])?;
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &signed],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
                'signed': Signature
            ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;

    run_cli(&["verify", &signed, "--verifier", &pubkeys])?;

    assert!(
        run_cli(&["verify", ALICE_KNOWS_BOB_EXAMPLE, "--verifier", &pubkeys])
            .is_err()
    );

    let bad_prvkeys = run_cli(&["generate", "prvkeys"])?;
    let bad_pubkeys = run_cli(&["generate", "pubkeys", &bad_prvkeys])?;
    assert!(
        run_cli(&["verify", &signed, "--verifier", &bad_pubkeys, &signed])
            .is_err()
    );

    Ok(())
}

#[test]
fn test_sign_2() -> Result<()> {
    let prvkeys = "ur:crypto-prvkeys/lftansgohdcxpfsndiahcxsfrhjoltglmebwwnnstovocffejytdbwihdkrtdykebkiebglbtteetansgehdcxvsdapeurgauovlbsvdfhvdcevywlptspfgnejpbksadehkhkfzehhfaysrsrbsdstbtagyeh";
    let wrapped_signed = run_cli_piped(&[
        &["subject", "type", "wrapped", ALICE_KNOWS_BOB_EXAMPLE],
        &["sign", "--signer", prvkeys],
    ])?;
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &wrapped_signed],
        indoc!(r#"
            {
                "Alice" [
                    "knows": "Bob"
                ]
            } [
                'signed': Signature
            ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;
    run_cli(&["verify", &wrapped_signed, "--verifier", &pubkeys])?;
    Ok(())
}

#[test]
fn test_sign_3() -> Result<()> {
    let e = run_cli_piped(&[
        &["subject", "type", "string", "Hello."],
        &["sign", "--signer", ALICE_PRVKEYS, "--signer", CAROL_PRVKEYS],
    ])?;
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &e],
        indoc!(r#"
            "Hello." [
                'signed': Signature
                'signed': Signature
            ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_sign_with_crypto_prvkeys() -> Result<()> {
    // Test that the sign command accepts ur:crypto-prvkeys
    let prvkeys = "ur:crypto-prvkeys/lftansgohdcxredidrnyhlnefzihclvepyfsvaemgsylfxamlstaprdnrsrkfmlukpaelrdtfgprtansgehdcxmybzpysoadgmcwoxlpensnfzwecspkihmkwlstvabzensbprnelssbfnqzbnfthlmycekeds";
    let signed =
        run_cli(&["sign", "--signer", prvkeys, ALICE_KNOWS_BOB_EXAMPLE])?;
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &signed],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
                'signed': Signature
            ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;

    run_cli(&["verify", &signed, "--verifier", &pubkeys])?;

    Ok(())
}

#[test]
fn test_signer_key_without_signer_flag_reports_hint() -> Result<()> {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("envelope"))
        .args(["sign", ALICE_PRVKEY_BASE])
        .output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr)?;
    // expected-text-output-rubric:
    let expected = "Error: signer keys must be passed with --signer/-s; did you mean: envelope sign --signer <key> [ENVELOPE]?\n";
    assert_actual_expected!(stderr, expected);

    Ok(())
}

#[test]
fn test_closed_stdout_pipe_does_not_panic() -> Result<()> {
    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("envelope"))
        .args(["subject", "type", "string", "hello"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    drop(child.stdout.take());
    let output = child.wait_with_output()?;
    let stderr = String::from_utf8(output.stderr)?;

    assert!(output.status.success(), "stderr:\n{stderr}");
    assert!(!stderr.contains("panicked"), "stderr:\n{stderr}");

    Ok(())
}
