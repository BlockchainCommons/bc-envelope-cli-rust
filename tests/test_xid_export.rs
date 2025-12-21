use indoc::indoc;
mod common;
use common::*;

/// Test that elision preserves signatures without re-signing.
/// This is the key behavior: envelope-level elision maintains the merkle tree.
#[test]
fn test_xid_export_elide_preserves_signature() {
    // Create a signed XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_before = run_cli(&["digest", &signed_xid]).unwrap();

    // Export with elided secrets - NO re-signing needed
    let elided_xid =
        run_cli(&["xid", "export", "--private", "elide", &signed_xid]).unwrap();

    // Digest should be identical (elision preserves merkle tree)
    let digest_after = run_cli(&["digest", &elided_xid]).unwrap();
    assert_eq!(digest_before, digest_after);

    // Verify the signature still works on the elided document
    run_cli(&["xid", "id", "--verify", "inception", &elided_xid]).unwrap();

    // Verify the format shows ELIDED and signature
    run_cli_expect(
        &["format", &elided_xid],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        'allow': 'All'
                        'nickname': "Alice"
                        ELIDED
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#},
    )
    .unwrap();
}

/// Test that omitting secrets invalidates the signature (changes merkle tree).
#[test]
fn test_xid_export_omit_invalidates_signature() {
    // Create a signed XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_before = run_cli(&["digest", &signed_xid]).unwrap();

    // Export with omitted secrets (no re-signing)
    let omitted_xid =
        run_cli(&["xid", "export", "--private", "omit", &signed_xid]).unwrap();

    // Digest should be DIFFERENT (omit changes merkle tree)
    let digest_after = run_cli(&["digest", &omitted_xid]).unwrap();
    assert_ne!(digest_before, digest_after);

    // Signature should be invalid (or missing since structure changed)
    let result =
        run_cli_raw(&["xid", "id", "--verify", "inception", &omitted_xid]);
    assert!(
        result.is_err(),
        "Signature should be invalid after omitting"
    );
}

/// Test that omit can be re-signed to produce a valid document.
#[test]
fn test_xid_export_omit_can_be_resigned() {
    // Create a signed XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Export with omitted secrets AND re-sign
    let omitted_resigned_xid = run_cli(&[
        "xid",
        "export",
        "--private",
        "omit",
        "--sign",
        "inception",
        &signed_xid,
    ])
    .unwrap();

    // Verify the new signature works
    run_cli(&["xid", "id", "--verify", "inception", &omitted_resigned_xid])
        .unwrap();

    // Verify format shows no ELIDED markers and no private keys
    run_cli_expect(
        &["format", &omitted_resigned_xid],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        'allow': 'All'
                        'nickname': "Alice"
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#},
    )
    .unwrap();
}

/// Test eliding only private keys (not generator) preserves signature.
#[test]
fn test_xid_export_private_elide_only() {
    // Create a signed XID document with provenance
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--generator",
        "include",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_before = run_cli(&["digest", &signed_xid]).unwrap();

    // Export with only private keys elided - NO re-signing needed
    let exported = run_cli(&[
        "xid",
        "export",
        "--private",
        "elide",
        "--generator",
        "include",
        &signed_xid,
    ])
    .unwrap();

    // Digest should be identical
    let digest_after = run_cli(&["digest", &exported]).unwrap();
    assert_eq!(digest_before, digest_after);

    // Verify signature still works
    run_cli(&["xid", "id", "--verify", "inception", &exported]).unwrap();

    // Verify private key is ELIDED but generator is visible
    let formatted = run_cli(&["format", &exported]).unwrap();
    assert!(formatted.contains("ELIDED"));
    assert!(formatted.contains("provenanceGenerator"));
}

/// Test eliding only generator (not private keys) preserves signature.
#[test]
fn test_xid_export_generator_elide_only() {
    // Create a signed XID document with provenance
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--generator",
        "include",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_before = run_cli(&["digest", &signed_xid]).unwrap();

    // Export with only generator elided - NO re-signing needed
    let exported = run_cli(&[
        "xid",
        "export",
        "--private",
        "include",
        "--generator",
        "elide",
        &signed_xid,
    ])
    .unwrap();

    // Digest should be identical
    let digest_after = run_cli(&["digest", &exported]).unwrap();
    assert_eq!(digest_before, digest_after);

    // Verify signature still works
    run_cli(&["xid", "id", "--verify", "inception", &exported]).unwrap();

    // Verify private keys are visible but generator is ELIDED
    let formatted = run_cli(&["format", &exported]).unwrap();
    assert!(formatted.contains("PrivateKeys"));
    // The provenance assertion should have ELIDED inside it
    assert!(formatted.contains("'provenance':"));
}

/// Test that default export includes everything and preserves signature.
#[test]
fn test_xid_export_default_includes_everything() {
    // Create a signed XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_before = run_cli(&["digest", &signed_xid]).unwrap();

    // Export with defaults (include everything) - NO re-signing needed
    let exported = run_cli(&["xid", "export", &signed_xid]).unwrap();

    // Digest should be identical
    let digest_after = run_cli(&["digest", &exported]).unwrap();
    assert_eq!(digest_before, digest_after);

    // Verify signature still works
    run_cli(&["xid", "id", "--verify", "inception", &exported]).unwrap();

    // Verify private keys are present and no ELIDED markers
    let formatted = run_cli(&["format", &exported]).unwrap();
    assert!(formatted.contains("PrivateKeys"));
    assert!(!formatted.contains("ELIDED"));
}

/// Test roundtrip with elision preserves XID identity.
#[test]
fn test_xid_export_roundtrip_with_elision() {
    // Create a signed XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original XID ID
    let id_original =
        run_cli(&["xid", "id", "--verify", "inception", &signed_xid]).unwrap();

    // Export with elision (no re-signing)
    let elided =
        run_cli(&["xid", "export", "--private", "elide", &signed_xid]).unwrap();

    // Get ID of elided version (should work without re-signing)
    let id_elided =
        run_cli(&["xid", "id", "--verify", "inception", &elided]).unwrap();

    // IDs should be identical
    assert_eq!(id_original, id_elided);
}

/// Test all elide/include combinations preserve signature without re-signing.
#[test]
fn test_xid_export_elide_combinations_preserve_signature() {
    // Create a signed XID document with provenance
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--generator",
        "include",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Get the original digest
    let digest_original = run_cli(&["digest", &signed_xid]).unwrap();

    // Test all elide/include combinations (no omit or encrypt)
    let combinations = vec![
        ("include", "include"),
        ("elide", "include"),
        ("include", "elide"),
        ("elide", "elide"),
    ];

    for (private_opt, generator_opt) in combinations {
        let exported = run_cli(&[
            "xid",
            "export",
            "--private",
            private_opt,
            "--generator",
            generator_opt,
            &signed_xid,
        ])
        .unwrap();

        // Digest should be preserved
        let digest_exported = run_cli(&["digest", &exported]).unwrap();
        assert_eq!(
            digest_original, digest_exported,
            "Digest changed with private={} generator={}",
            private_opt, generator_opt
        );

        // Signature should still verify
        run_cli(&["xid", "id", "--verify", "inception", &exported])
            .unwrap_or_else(|_| {
                panic!(
                    "Signature failed with private={} generator={}",
                    private_opt, generator_opt
                )
            });
    }
}

/// Test that omit produces no ELIDED markers.
#[test]
fn test_xid_export_no_elided_when_omitted() {
    // Create a XID document (unsigned for simplicity)
    let xid =
        run_cli(&["xid", "new", ALICE_PRVKEYS, "--nickname", "Alice"]).unwrap();

    // Export with omit
    let omitted =
        run_cli(&["xid", "export", "--private", "omit", &xid]).unwrap();

    // Verify format shows no ELIDED markers and no private keys
    run_cli_expect(
        &["format", &omitted],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#},
    )
    .unwrap();
}

/// Test method add command uses output options correctly.
#[test]
fn test_xid_method_add_with_output_options() {
    // Create and sign a XID document
    let signed_xid = run_cli(&[
        "xid",
        "new",
        ALICE_PRVKEYS,
        "--nickname",
        "Alice",
        "--sign",
        "inception",
    ])
    .unwrap();

    // Add a method with elided output (requires re-sign since it modifies doc)
    let elided_xid = run_cli(&[
        "xid",
        "method",
        "add",
        "https://example.org",
        "--private",
        "elide",
        "--sign",
        "inception",
        &signed_xid,
    ])
    .unwrap();

    // Verify signature works
    run_cli(&["xid", "id", "--verify", "inception", &elided_xid]).unwrap();

    // Verify elision happened and endpoint was added
    let formatted = run_cli(&["format", &elided_xid]).unwrap();
    assert!(formatted.contains("ELIDED"));
    assert!(formatted.contains("https://example.org"));
}

/// Test key add with output options.
#[test]
fn test_xid_key_add_with_output_options() {
    // Create a XID document
    let xid =
        run_cli(&["xid", "new", ALICE_PRVKEYS, "--nickname", "Alice"]).unwrap();

    // Add a key with private keys elided
    let with_key = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--private",
        "elide",
        &xid,
    ])
    .unwrap();

    // Verify the document has both keys, with private keys elided
    let formatted = run_cli(&["format", &with_key]).unwrap();
    assert!(formatted.contains("ELIDED"));
    // Should have 2 PublicKeys entries (Alice and Bob)
    assert_eq!(formatted.matches("PublicKeys").count(), 2);
}

/// Test that export with elision preserves multiple keys.
#[test]
fn test_xid_export_preserves_multiple_keys() {
    // Create a XID document
    let xid =
        run_cli(&["xid", "new", ALICE_PRVKEYS, "--nickname", "Alice"]).unwrap();

    // Add multiple keys
    let with_bob = run_cli(&["xid", "key", "add", BOB_PUBKEYS, &xid]).unwrap();
    let with_carol =
        run_cli(&["xid", "key", "add", CAROL_PUBKEYS, &with_bob]).unwrap();

    // Sign it
    let signed = run_cli(&[
        "xid",
        "key",
        "add",
        DAVE_PUBKEYS,
        "--sign",
        "inception",
        &with_carol,
    ])
    .unwrap();

    // Get original digest
    let digest_before = run_cli(&["digest", &signed]).unwrap();

    // Export with elision (NO re-signing needed)
    let elided =
        run_cli(&["xid", "export", "--private", "elide", &signed]).unwrap();

    // Digest should be preserved
    let digest_after = run_cli(&["digest", &elided]).unwrap();
    assert_eq!(digest_before, digest_after);

    // Verify signature works
    run_cli(&["xid", "id", "--verify", "inception", &elided]).unwrap();

    // Verify all keys are still present (count the number of PublicKeys)
    let formatted = run_cli(&["format", &elided]).unwrap();
    let public_keys_count = formatted.matches("PublicKeys").count();
    assert_eq!(
        public_keys_count, 4,
        "Should have 4 keys (Alice, Bob, Carol, Dave)"
    );

    // Should have ELIDED for Alice's private key
    assert!(formatted.contains("ELIDED"));
}
