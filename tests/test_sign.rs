use anyhow::Result;
use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_sign() -> Result<()> {
    let prvkeys = "ur:crypto-prvkey-base/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let signed =
        run_cli(&["sign", "--signer", prvkeys, ALICE_KNOWS_BOB_EXAMPLE])?;
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
    let prvkeys = "ur:crypto-prvkey-base/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let wrapped_signed = run_cli_piped(&[
        &["subject", "type", "wrapped", ALICE_KNOWS_BOB_EXAMPLE],
        &["sign", "--signer", prvkeys],
    ])?;
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
        &[
            "sign",
            "--signer",
            ALICE_PRVKEY_BASE,
            "--signer",
            CAROL_PRVKEY_BASE,
        ],
    ])?;
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
