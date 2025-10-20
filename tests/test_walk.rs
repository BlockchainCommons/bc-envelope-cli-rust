mod common;
use anyhow::Result;
use common::*;
use indoc::indoc;

#[test]
fn test_walk_basic() -> Result<()> {
    // The output should contain all node digests space-separated
    #[rustfmt::skip]
    run_cli_expect(
        &["walk", ALICE_KNOWS_BOB_EXAMPLE],
        indoc!(r#"
            ur:digest/hdcxbwmwcwfdkecauerfvsdirpwpfhfgtalfmulesnstvlrpoyfzuyenamdpmdcfutdlstyaqzrk ur:digest/hdcxbwrlfpmwnsemrovtnssrtnotcfgshdvezcjedlbbtypatiwtecoxjnjnhtcafhbysptsnsnl ur:digest/hdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboerhzebtws ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds ur:digest/hdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrddpjtgdfh
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_matching_elided() -> Result<()> {
    // Create an envelope with an elided assertion
    let knows_assertion =
        run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    let assertion_digest = run_cli(&["digest", &knows_assertion])?;
    let elided = run_cli(&[
        "elide",
        "removing",
        &assertion_digest,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    // Verify the elided format
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
            "Alice" [
                ELIDED
            ]
        "#)
    )?;

    // Find elided nodes
    #[rustfmt::skip]
    run_cli_expect(
        &["walk", &elided, "matching", "--elided"],
        "ur:digest/hdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboerhzebtws\n"
    )?;

    Ok(())
}

#[test]
fn test_walk_unelide() -> Result<()> {
    // Create an envelope with an elided assertion
    let knows_assertion =
        run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    let assertion_digest = run_cli(&["digest", &knows_assertion])?;
    let elided = run_cli(&[
        "elide",
        "removing",
        &assertion_digest,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    // Unelide it back using the original assertion
    let unelided = run_cli(&["walk", &elided, "unelide", &knows_assertion])?;

    // Should be equivalent to original
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &unelided],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_decrypt() -> Result<()> {
    let key = run_cli(&["generate", "key"])?;
    let encrypted =
        run_cli(&["encrypt", "--key", &key, ALICE_KNOWS_BOB_EXAMPLE])?;

    // Verify encryption (subject is encrypted, assertions remain)
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
            ENCRYPTED [
                "knows": "Bob"
            ]
        "#)
    )?;

    // Decrypt using walk
    let decrypted = run_cli(&["walk", &encrypted, "decrypt", &key])?;

    // Should be equivalent to original
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &decrypted],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_decompress() -> Result<()> {
    let compressed = run_cli(&["compress", ALICE_KNOWS_BOB_EXAMPLE])?;

    // Verify compression
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &compressed],
        "COMPRESSED\n"
    )?;

    // Decompress using walk
    let decompressed = run_cli(&["walk", &compressed, "decompress"])?;

    // Should be equivalent to original
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &decompressed],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_with_target() -> Result<()> {
    let digest = run_cli(&["digest", ALICE_KNOWS_BOB_EXAMPLE])?;

    // Walk with target filter - should return just that digest
    #[rustfmt::skip]
    run_cli_expect(
        &["walk", "--target", &digest, ALICE_KNOWS_BOB_EXAMPLE],
        "ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds\n"
    )?;

    Ok(())
}
