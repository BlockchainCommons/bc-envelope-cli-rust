use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_encrypt() {
    let encrypted = run_cli(&[
        "encrypt",
        "--key",
        KEY_EXAMPLE,
        ALICE_KNOWS_BOB_EXAMPLE
    ]).unwrap();
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
        ENCRYPTED [
            "knows": "Bob"
        ]
        "#)
    ).unwrap();
    let decrypted = run_cli(&[
        "decrypt",
        "--key",
        KEY_EXAMPLE,
        &encrypted,
    ]).unwrap();
    assert_eq!(decrypted, ALICE_KNOWS_BOB_EXAMPLE);
}
