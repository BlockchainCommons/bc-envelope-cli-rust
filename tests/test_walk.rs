mod common;
use anyhow::Result;
use common::*;
use indoc::indoc;

#[test]
fn test_walk_basic() -> Result<()> {
    // The output should contain all node digests space-separated
    // expected-text-output-rubric:
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
    // expected-text-output-rubric:
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
    // expected-text-output-rubric:
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
    // expected-text-output-rubric:
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
    // expected-text-output-rubric:
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
    // expected-text-output-rubric:
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

#[test]
fn test_walk_replace_basic() -> Result<()> {
    // Create envelopes
    let bob = run_cli(&["subject", "type", "string", "Bob"])?;
    let charlie = run_cli(&["subject", "type", "string", "Charlie"])?;

    // Create an envelope with Bob referenced multiple times
    let envelope = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "likes",
        "envelope",
        &bob,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    // Verify the before state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &envelope],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
                "likes": "Bob"
            ]
        "#)
    )?;

    // Get Bob's digest
    let bob_digest = run_cli(&["digest", &bob])?;

    // Replace all instances of Bob with Charlie
    let modified = run_cli(&[
        "walk",
        "--target",
        &bob_digest,
        &envelope,
        "replace",
        &charlie,
    ])?;

    // Verify the after state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &modified],
        indoc!(r#"
            "Alice" [
                "knows": "Charlie"
                "likes": "Charlie"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_replace_subject() -> Result<()> {
    let alice = run_cli(&["subject", "type", "string", "Alice"])?;
    let carol = run_cli(&["subject", "type", "string", "Carol"])?;

    // Verify the before state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", ALICE_KNOWS_BOB_EXAMPLE],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
            ]
        "#)
    )?;

    // Get Alice's digest
    let alice_digest = run_cli(&["digest", &alice])?;

    // Replace the subject (Alice) with Carol
    let modified = run_cli(&[
        "walk",
        "--target",
        &alice_digest,
        ALICE_KNOWS_BOB_EXAMPLE,
        "replace",
        &carol,
    ])?;

    // Verify the after state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &modified],
        indoc!(r#"
            "Carol" [
                "knows": "Bob"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_replace_elided() -> Result<()> {
    let bob = run_cli(&["subject", "type", "string", "Bob"])?;
    let charlie = run_cli(&["subject", "type", "string", "Charlie"])?;

    // Create an envelope with Bob referenced multiple times
    let envelope = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "likes",
        "envelope",
        &bob,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    // Get Bob's digest
    let bob_digest = run_cli(&["digest", &bob])?;

    // Elide Bob
    let elided = run_cli(&["elide", "removing", &bob_digest, &envelope])?;

    // Verify the elided state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
            "Alice" [
                "knows": ELIDED
                "likes": ELIDED
            ]
        "#)
    )?;

    // Replace the elided Bob with Charlie
    let modified = run_cli(&[
        "walk",
        "--target",
        &bob_digest,
        &elided,
        "replace",
        &charlie,
    ])?;

    // Verify the after state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &modified],
        indoc!(r#"
            "Alice" [
                "knows": "Charlie"
                "likes": "Charlie"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_replace_multiple_targets() -> Result<()> {
    let bob = run_cli(&["subject", "type", "string", "Bob"])?;
    let carol_subj = run_cli(&["subject", "type", "string", "Carol"])?;
    let redacted = run_cli(&["subject", "type", "string", "REDACTED"])?;

    // Create an envelope with Bob and Carol
    let envelope = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "likes",
        "envelope",
        &carol_subj,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    // Verify the before state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &envelope],
        indoc!(r#"
            "Alice" [
                "knows": "Bob"
                "likes": "Carol"
            ]
        "#)
    )?;

    // Get Bob's and Carol's digests
    let bob_digest = run_cli(&["digest", &bob])?;
    let carol_digest = run_cli(&["digest", &carol_subj])?;

    // Combine the digests into a space-separated string for --target
    let targets = format!("{} {}", bob_digest.trim(), carol_digest.trim());

    // Replace both Bob and Carol with REDACTED
    let modified = run_cli(&[
        "walk", "--target", &targets, &envelope, "replace", &redacted,
    ])?;

    // Verify the after state
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &modified],
        indoc!(r#"
            "Alice" [
                "knows": "REDACTED"
                "likes": "REDACTED"
            ]
        "#)
    )?;

    Ok(())
}

#[test]
fn test_walk_replace_requires_target() -> Result<()> {
    let charlie = run_cli(&["subject", "type", "string", "Charlie"])?;

    // Try to replace without specifying --target (should fail)
    let result =
        run_cli_raw(&["walk", ALICE_KNOWS_BOB_EXAMPLE, "replace", &charlie]);

    assert!(result.is_err());

    Ok(())
}
