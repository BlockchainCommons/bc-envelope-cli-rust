mod common;

use common::*;
use indoc::indoc;

const BOB_PRVKEYS: &str = "ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk";

#[test]
fn issue_12_xid_key_add_preserves_custom_assertions() {
    let xid =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        &xid,
    ])
    .unwrap();

    let updated = run_cli(&[
        "xid",
        "key",
        "add",
        "--nickname",
        "Bob",
        BOB_PUBKEYS,
        &custom,
    ])
    .unwrap();

    let formatted = run_cli(&["format", &updated]).unwrap();

    // expected-text-output-rubric:
    #[rustfmt::skip]
    assert_actual_expected!(formatted, indoc! {r#"
        XID(93a4d4e7) [
            "customField": "customValue"
            'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
            'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                'allow': 'All'
                'nickname': "Bob"
            ]
        ]
    "#}.trim());
}

#[test]
fn issue_12_xid_key_remove_preserves_custom_assertions() {
    let xid =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();
    let with_bob =
        run_cli(&["xid", "key", "add", "--nickname", "Bob", BOB_PUBKEYS, &xid])
            .unwrap();
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        &with_bob,
    ])
    .unwrap();

    let updated =
        run_cli(&["xid", "key", "remove", BOB_PUBKEYS, &custom]).unwrap();
    let formatted = run_cli(&["format", &updated]).unwrap();

    // expected-text-output-rubric:
    #[rustfmt::skip]
    assert_actual_expected!(formatted, indoc! {r#"
        XID(93a4d4e7) [
            "customField": "customValue"
            'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
        ]
    "#}.trim());
}

#[test]
fn issue_14_xid_provenance_next_preserves_custom_assertions() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "include",
        "--generator",
        "include",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        &xid,
    ])
    .unwrap();

    let updated = run_cli(&[
        "xid",
        "provenance",
        "next",
        "--date",
        "2024-01-15",
        &custom,
    ])
    .unwrap();
    let formatted = run_cli(&["format", &updated]).unwrap();

    assert!(formatted.contains(r#""customField": "customValue""#));
    assert!(formatted.contains(r#""next-seq": 2"#));
}

#[test]
#[ignore = "open feature request #15; separate from the bc-xid custom assertion preservation fix"]
fn issue_15_xid_service_add_embeds_encrypted_private_keys() {
    let xid =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    let updated = run_cli(&[
        "xid",
        "service",
        "add",
        "--name",
        "GitHub",
        "--prvkeys",
        BOB_PRVKEYS,
        "--private",
        "encrypt",
        "--encrypt-password",
        "service password",
        "--key-nickname",
        "GitHub signing",
        "--allow",
        "sign",
        "https://github.com/alice",
        &xid,
    ])
    .unwrap();
    let formatted = run_cli(&["format", &updated]).unwrap();

    // expected-text-output-rubric:
    #[rustfmt::skip]
    assert_actual_expected!(formatted, indoc! {r#"
        XID(93a4d4e7) [
            'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
            'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                {
                    'privateKey': ENCRYPTED [
                        'hasSecret': EncryptedKey(Argon2id)
                    ]
                } [
                    'salt': Salt
                ]
                'allow': 'All'
                'nickname': "GitHub signing"
            ]
            'service': URI(https://github.com/alice) [
                'allow': 'Sign'
                'key': Reference(e2c18423)
                'name': "GitHub"
            ]
        ]
    "#}.trim());
}
