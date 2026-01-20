use indoc::indoc;
mod common;
use common::*;

#[test]
fn test_xid_resolution_basic() {
    bc_envelope::register_tags();

    // ### `xid resolution`: Work with Resolution Methods (dereferenceVia)
    //
    // Resolution methods declare where a XID document can be fetched from.
    // They are stored as `'dereferenceVia'` assertions on the XID document.

    // Create a basic XID document using deterministic keys.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Verify starting with no resolution methods.
    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "0").unwrap();

    // Add a resolution method.
    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    // Verify the XID document now has the resolution method.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(https://resolver.example.com)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Verify count is now 1.
    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "1").unwrap();

    // List all resolution methods.
    run_cli_expect(
        &["xid", "resolution", "all", &xid_doc],
        "https://resolver.example.com",
    )
    .unwrap();

    // Get a resolution method by index.
    run_cli_expect(
        &["xid", "resolution", "at", "0", &xid_doc],
        "https://resolver.example.com",
    )
    .unwrap();
}

#[test]
fn test_xid_resolution_multiple() {
    bc_envelope::register_tags();

    // Create XID and add multiple resolution methods.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    let xid_doc =
        run_cli(&["xid", "resolution", "add", "btcr:01234567", &xid_doc])
            .unwrap();

    // Verify the XID document has both resolution methods.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(btcr:01234567)
                'dereferenceVia': URI(https://resolver.example.com)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Verify count.
    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "2").unwrap();

    // Verify all lists both methods (order may vary due to HashSet).
    let all_methods = run_cli(&["xid", "resolution", "all", &xid_doc]).unwrap();
    assert!(all_methods.contains("https://resolver.example.com"));
    assert!(all_methods.contains("btcr:01234567"));
}

#[test]
fn test_xid_resolution_remove() {
    bc_envelope::register_tags();

    // Create XID with two resolution methods.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    let xid_doc =
        run_cli(&["xid", "resolution", "add", "btcr:01234567", &xid_doc])
            .unwrap();

    // Verify both resolution methods are present.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(btcr:01234567)
                'dereferenceVia': URI(https://resolver.example.com)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "2").unwrap();

    // Remove one resolution method.
    let xid_doc =
        run_cli(&["xid", "resolution", "remove", "btcr:01234567", &xid_doc])
            .unwrap();

    // Verify only the first method remains.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(https://resolver.example.com)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "1").unwrap();
    run_cli_expect(
        &["xid", "resolution", "all", &xid_doc],
        "https://resolver.example.com",
    )
    .unwrap();
}

#[test]
fn test_xid_resolution_remove_not_found() {
    bc_envelope::register_tags();

    // Create XID with one resolution method.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    // Try to remove a non-existent resolution method.
    let result = run_cli(&[
        "xid",
        "resolution",
        "remove",
        "https://nonexistent.com",
        &xid_doc,
    ]);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Resolution method not found"));
    assert!(err_msg.contains("https://nonexistent.com"));
}

#[test]
fn test_xid_resolution_index_out_of_bounds() {
    bc_envelope::register_tags();

    // Create XID with one resolution method.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    // Try to access an invalid index.
    let result = run_cli(&["xid", "resolution", "at", "5", &xid_doc]);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Index out of bounds"));
}

#[test]
fn test_xid_resolution_with_signature() {
    bc_envelope::register_tags();

    // Test that resolution methods work with signed XID documents.
    //
    // Create a signed XID document with encrypted private keys.

    let xid_doc = run_cli(&[
        "xid",
        "new",
        "--nickname",
        "Alice",
        "--private",
        "encrypt",
        "--encrypt-password",
        "secret",
        "--sign",
        "inception",
        ALICE_PRVKEYS,
    ])
    .unwrap();

    // Verify initial signed structure with encrypted private key.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': ENCRYPTED [
                                'hasSecret': EncryptedKey(Argon2id)
                            ]
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                        'nickname': "Alice"
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Add a resolution method with proper verification and re-signing.
    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "--verify",
        "inception",
        "--password",
        "secret",
        "--sign",
        "inception",
        "--private",
        "encrypt",
        "--encrypt-password",
        "secret",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    // Verify the signed document now has the resolution method.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'dereferenceVia': URI(https://resolver.example.com)
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': ENCRYPTED [
                                'hasSecret': EncryptedKey(Argon2id)
                            ]
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                        'nickname': "Alice"
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Verify signature is valid.
    let result = run_cli(&["xid", "id", "--verify", "inception", &xid_doc]);
    assert!(result.is_ok());
}

#[test]
fn test_xid_resolution_empty_list() {
    bc_envelope::register_tags();

    // Create XID without any resolution methods.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    // Verify count is 0.
    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "0").unwrap();

    // Verify all returns empty string.
    run_cli_expect(&["xid", "resolution", "all", &xid_doc], "").unwrap();
}

#[test]
fn test_xid_resolution_preserved_after_other_operations() {
    bc_envelope::register_tags();

    // Test that resolution methods are preserved when doing other operations
    // on the XID document.

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    // Add a resolution method.
    let xid_doc = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://resolver.example.com",
        &xid_doc,
    ])
    .unwrap();

    // Add another key to the XID document.
    let xid_doc = run_cli(&[
        "xid",
        "key",
        "add",
        "--nickname",
        "Device2",
        BOB_PUBKEYS,
        &xid_doc,
    ])
    .unwrap();

    // Verify resolution method is still present along with both keys.
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(https://resolver.example.com)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                    'allow': 'All'
                    'nickname': "Device2"
                ]
            ]
        "#}.trim()
    ).unwrap();

    run_cli_expect(&["xid", "resolution", "count", &xid_doc], "1").unwrap();
    run_cli_expect(
        &["xid", "resolution", "all", &xid_doc],
        "https://resolver.example.com",
    )
    .unwrap();
}
