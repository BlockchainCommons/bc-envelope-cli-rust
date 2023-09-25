mod common;
use common::*;

#[test]
fn test_envelope_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["digest", ALICE_KNOWS_BOB_EXAMPLE],
        "ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds"
    )
}

#[test]
fn test_envelope_digest_hex() -> anyhow::Result<()> {
    run_cli_expect(
        &["digest", "--hex", ALICE_KNOWS_BOB_EXAMPLE],
        "8955db5e016affb133df56c11fe6c5c82fa3036263d651286d134c7e56c0e9f2"
    )
}
