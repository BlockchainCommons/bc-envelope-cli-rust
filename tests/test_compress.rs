use indoc::indoc;
use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_compress_1() -> Result<()> {
    let compressed = run_cli(&["compress", "--subject", ALICE_KNOWS_BOB_EXAMPLE])?;

    run_cli_expect(
        &["format", &compressed],
        indoc!(r#"
        COMPRESSED [
            "knows": "Bob"
        ]
        "#)
    )?;
    let decompressed = run_cli(&["uncompress", "--subject", &compressed])?;
    assert_eq!(decompressed, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

#[test]
fn test_compress_2() -> Result<()> {
    let compressed = run_cli(&["compress", CREDENTIAL_EXAMPLE])?;

    println!("{} {}", CREDENTIAL_EXAMPLE.len(), compressed.len());
    assert_eq!(CREDENTIAL_EXAMPLE.len(), 1210);
    assert_eq!(compressed.len(), 1036);

    run_cli_expect(
        &["format", &compressed],
        "COMPRESSED"
    )?;
    let decompressed = run_cli(&["uncompress", &compressed])?;
    assert_eq!(decompressed, CREDENTIAL_EXAMPLE);
    Ok(())
}
