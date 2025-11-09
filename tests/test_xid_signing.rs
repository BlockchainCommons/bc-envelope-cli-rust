use indoc::indoc;
mod common;
use common::*;

#[test]
fn test_xid_verify_signature() {
    // Create a new XID document with inception key that can sign
    let xid_unsigned = run_cli(&["xid", "new", ALICE_PRVKEYS]).unwrap();

    // Verify that reading with --verify none works (default)
    let id_none = run_cli(&["xid", "id", &xid_unsigned]).unwrap();
    assert!(id_none.starts_with("ur:xid/"));

    let id_none_explicit =
        run_cli(&["xid", "id", "--verify", "none", &xid_unsigned]).unwrap();
    assert_eq!(id_none, id_none_explicit);

    // Attempting to verify inception signature on unsigned document should fail
    let result =
        run_cli_raw(&["xid", "id", "--verify", "inception", &xid_unsigned]);
    assert!(result.is_err());
}

#[test]
fn test_xid_sign_inception() {
    // Create a new XID document with inception key
    let xid = run_cli(&["xid", "new", ALICE_PRVKEYS]).unwrap();

    // Sign it with the inception key when adding a key
    let signed_xid = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--sign",
        "inception",
        &xid,
    ])
    .unwrap();

    // Verify the signature
    let verified_id =
        run_cli(&["xid", "id", "--verify", "inception", &signed_xid]).unwrap();
    assert!(verified_id.starts_with("ur:xid/"));

    // Verify the document has both keys
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &signed_xid],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Verify reading without verification still works
    let id_no_verify = run_cli(&["xid", "id", &signed_xid]).unwrap();
    assert_eq!(verified_id, id_no_verify);

    // Attempting to verify with wrong method should fail
    let result = run_cli_raw(&["xid", "id", "--verify", "none", &signed_xid]);
    // This actually succeeds because --verify none doesn't verify
    assert!(result.is_ok());
}

#[test]
fn test_xid_sign_with_external_key() {
    // Create a new XID document
    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Sign it with an external signing key
    let signed_xid = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        CAROL_PRVKEYS,
        &xid,
    ])
    .unwrap();

    // The document should now have a signature
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &signed_xid],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Can't verify as inception (Carol's key is not the inception key)
    let result =
        run_cli_raw(&["xid", "id", "--verify", "inception", &signed_xid]);
    assert!(result.is_err());
}

#[test]
fn test_xid_sign_service_operations() {
    // Create a new XID document with Alice's keys
    let xid = run_cli(&["xid", "new", ALICE_PRVKEYS]).unwrap();

    // Add a service with signing (use Alice's public keys for the service)
    let with_service = run_cli(&[
        "xid",
        "service",
        "add",
        "https://example.com/service",
        "--key",
        ALICE_PUBKEYS,
        "--sign",
        "inception",
        &xid,
    ])
    .unwrap();

    // Verify the signature
    run_cli(&["xid", "id", "--verify", "inception", &with_service]).unwrap();

    // Verify the service was added and document is signed
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &with_service],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                    'service': URI(https://example.com/service) [
                        'allow': 'All'
                        'key': Reference(cab108a0)
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Remove a service with signing
    let without_service = run_cli(&[
        "xid",
        "service",
        "remove",
        "https://example.com/service",
        "--sign",
        "inception",
        &with_service,
    ])
    .unwrap();

    // Verify the signature on the modified document
    run_cli(&["xid", "id", "--verify", "inception", &without_service]).unwrap();

    // Verify the service was removed and document is still signed
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &without_service],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_new_with_signing() {
    // Create a new XID document and sign it immediately
    let signed_xid =
        run_cli(&["xid", "new", ALICE_PRVKEYS, "--sign", "inception"]).unwrap();

    // Verify the signature
    let verified_id =
        run_cli(&["xid", "id", "--verify", "inception", &signed_xid]).unwrap();
    assert!(verified_id.starts_with("ur:xid/"));

    // Check the format includes the signature
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &signed_xid],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_verify_and_sign_chaining() {
    // Create and sign a document
    let xid1 =
        run_cli(&["xid", "new", ALICE_PRVKEYS, "--sign", "inception"]).unwrap();

    // Verify and modify with new signature
    let xid2 = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--verify",
        "inception",
        "--sign",
        "inception",
        &xid1,
    ])
    .unwrap();

    // Should be verifiable
    run_cli(&["xid", "id", "--verify", "inception", &xid2]).unwrap();

    // Verify xid2 has the expected structure
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &xid2],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();

    // Add another key
    let xid3 = run_cli(&[
        "xid",
        "key",
        "add",
        CAROL_PUBKEYS,
        "--verify",
        "inception",
        "--sign",
        "inception",
        &xid2,
    ])
    .unwrap();

    // Still verifiable
    run_cli(&["xid", "id", "--verify", "inception", &xid3]).unwrap();

    // Verify xid3 has the expected structure with all three keys
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &xid3],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        {
                            'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                        } [
                            'salt': Salt
                        ]
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                    'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_sign_with_encrypted_private_keys() {
    // Create an encrypted PrivateKeys envelope
    let encrypted_keys = run_cli_piped(&[
        &["subject", "type", "ur", CAROL_PRVKEYS],
        &["encrypt", "--password", "testpass"],
    ])
    .unwrap();

    // Verify it's an envelope
    assert!(encrypted_keys.starts_with("ur:envelope/"));

    // Create a new XID document
    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Sign with the encrypted key, providing password to decrypt it
    let signed_xid = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        &encrypted_keys,
        "--password",
        "testpass",
        &xid,
    ])
    .unwrap();

    // The document should have a signature
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &signed_xid],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_sign_with_encrypted_signing_private_key() {
    // Just use Carol's full PrivateKeys for this test
    // (In practice, one might extract just the signing key, but PrivateKeys
    // works fine)
    let encrypted_key = run_cli_piped(&[
        &["subject", "type", "ur", CAROL_PRVKEYS],
        &["encrypt", "--password", "mypass"],
    ])
    .unwrap();

    assert!(encrypted_key.starts_with("ur:envelope/"));

    // Create a new XID document
    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Sign with the encrypted PrivateKeys
    let signed_xid = run_cli(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        &encrypted_key,
        "--password",
        "mypass",
        &xid,
    ])
    .unwrap();

    // Verify it has a signature
    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["format", &signed_xid],
        ],
        indoc! {r#"
            {
                XID(93a4d4e7) [
                    'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                        'allow': 'All'
                    ]
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                    ]
                ]
            } [
                'signed': Signature
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_sign_with_encrypted_key_wrong_password() {
    // Create an encrypted key
    let encrypted_keys = run_cli_piped(&[
        &["subject", "type", "ur", CAROL_PRVKEYS],
        &["encrypt", "--password", "correctpass"],
    ])
    .unwrap();

    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Try to sign with wrong password - should fail
    let result = run_cli_raw(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        &encrypted_keys,
        "--password",
        "wrongpass",
        &xid,
    ]);

    assert!(result.is_err());
}

#[test]
fn test_xid_sign_with_encrypted_key_no_password() {
    // Create an encrypted key
    let encrypted_keys = run_cli_piped(&[
        &["subject", "type", "ur", CAROL_PRVKEYS],
        &["encrypt", "--password", "testpass"],
    ])
    .unwrap();

    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Try to sign without providing password - should fail
    let result = run_cli_raw(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        &encrypted_keys,
        &xid,
    ]);

    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = format!("{}", e);
        assert!(
            err_msg.contains("Password required")
                || err_msg.contains("password")
        );
    }
}

#[test]
fn test_xid_sign_with_invalid_encrypted_content() {
    // Create an encrypted envelope that doesn't contain keys
    let not_keys = run_cli_piped(&[
        &["subject", "type", "string", "Hello"],
        &["encrypt", "--password", "testpass"],
    ])
    .unwrap();

    let xid = run_cli(&["xid", "new", ALICE_PUBKEYS]).unwrap();

    // Try to sign with it - should fail with clear error
    let result = run_cli_raw(&[
        "xid",
        "key",
        "add",
        BOB_PUBKEYS,
        "--signing-key",
        &not_keys,
        "--password",
        "testpass",
        &xid,
    ]);

    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = format!("{}", e);
        assert!(
            err_msg.contains("not contain valid signing keys")
                || err_msg.contains("Invalid")
                || err_msg.contains("expected UR type")
        );
    }
}
