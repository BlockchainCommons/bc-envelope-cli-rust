use indoc::indoc;

use bc_envelope::prelude::*;

mod common;
use common::*;

#[test]
fn test_encrypt() -> anyhow::Result<()> {
    let encrypted = run_cli(&[
        "encrypt",
        "--key",
        KEY_EXAMPLE,
        ALICE_KNOWS_BOB_EXAMPLE
    ])?;
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
        ENCRYPTED [
            "knows": "Bob"
        ]
        "#)
    )?;
    let decrypted = run_cli(&[
        "decrypt",
        "--key",
        KEY_EXAMPLE,
        &encrypted,
    ])?;
    assert_eq!(decrypted, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

#[test]
fn test_generate_private_keys_1() -> anyhow::Result<()> {
    let prvkeys = run_cli(&["generate", "prvkeys"])?;
    assert_eq!(UR::from_ur_string(prvkeys)?.ur_type(), "crypto-prvkeys");
    Ok(())
}

#[test]
fn test_generate_private_keys_2() -> anyhow::Result<()> {
    let seed = "ur:crypto-seed/oyadhdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdweocfztrd";
    let prvkeys1 = run_cli(&["generate", "prvkeys", "--seed", seed])?;
    assert_eq!(
        prvkeys1,
        "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah"
    );
    let prvkeys2 = run_cli(&["generate", "prvkeys", "--seed", seed])?;
    assert_eq!(prvkeys1, prvkeys2);

    let pubkeys = run_cli(&["generate", "pubkeys", &prvkeys1])?;
    assert_eq!(
        pubkeys,
        "ur:crypto-pubkeys/lftanshfhdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejobebtdlhd"
    );
    Ok(())
}
