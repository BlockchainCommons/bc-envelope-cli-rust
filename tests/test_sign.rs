use indoc::indoc;
use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_sign() -> Result<()> {
    let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let signed = run_cli(&[
        "sign",
        "--prvkeys",
        prvkeys,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;
    run_cli_expect(
        &["format", &signed],
        indoc!(r#"
        "Alice" [
            "knows": "Bob"
            'verifiedBy': Signature
        ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;

    run_cli(&["verify", &signed, "--pubkeys", &pubkeys])?;

    assert!(run_cli(&["verify", ALICE_KNOWS_BOB_EXAMPLE, "--pubkeys", &pubkeys]).is_err());

    let bad_prvkeys = run_cli(&["generate", "prvkeys"])?;
    let bad_pubkeys = run_cli(&["generate", "pubkeys", &bad_prvkeys])?;
    assert!(run_cli(&["verify", &signed, "--pubkeys", &bad_pubkeys, &signed]).is_err());

    Ok(())
}

#[test]
fn test_sign_2() -> Result<()> {
    let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let wrapped_signed = run_cli_piped(&[
        &["subject", "type", "wrapped", ALICE_KNOWS_BOB_EXAMPLE],
        &["sign", "--prvkeys", prvkeys]
    ])?;
    run_cli_expect(
        &["format", &wrapped_signed],
        indoc!(r#"
        {
            "Alice" [
                "knows": "Bob"
            ]
        } [
            'verifiedBy': Signature
        ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;
    run_cli(&["verify", &wrapped_signed, "--pubkeys", &pubkeys])?;
    Ok(())
}

#[test]
fn test_sign_3() -> Result<()> {
    let e = run_cli_piped(&[
        &["subject", "type", "string", "Hello."],
        &["sign", "--prvkeys", ALICE_PRVKEYS, "--prvkeys", CAROL_PRVKEYS]
    ])?;
    run_cli_expect(
        &["format", &e],
        indoc!(r#"
        "Hello." [
            'verifiedBy': Signature
            'verifiedBy': Signature
        ]
        "#)
    )?;
    Ok(())
}
