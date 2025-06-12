use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_encrypt_key() {
    let encrypted =
        run_cli(&["encrypt", "--key", KEY_EXAMPLE, ALICE_KNOWS_BOB_EXAMPLE])
            .unwrap();
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
            ENCRYPTED [
                "knows": "Bob"
            ]
        "#)
    ).unwrap();
    let decrypted =
        run_cli(&["decrypt", "--key", KEY_EXAMPLE, &encrypted]).unwrap();
    assert_eq!(decrypted, ALICE_KNOWS_BOB_EXAMPLE);
}

#[test]
fn test_encrypt_password() {
    let encrypted = run_cli_piped(&[
        &["subject", "type", "wrapped", ALICE_KNOWS_BOB_EXAMPLE],
        &["encrypt", "--password", "password"],
    ])
    .unwrap();
    #[rustfmt::skip]
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
            ENCRYPTED [
                'hasSecret': EncryptedKey(Argon2id)
            ]
        "#)
    ).unwrap();
    let decrypted = run_cli_piped(&[
        &["decrypt", "--password", "password", &encrypted],
        &["extract", "wrapped"],
    ])
    .unwrap();
    assert_eq!(decrypted, ALICE_KNOWS_BOB_EXAMPLE);
}
