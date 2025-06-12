use anyhow::Result;
use bc_envelope::prelude::*;

mod common;
use common::*;

#[test]
fn test_generate_arid() -> Result<()> {
    let output1 = run_cli(&["generate", "arid"])?;
    let output2 = run_cli(&["generate", "arid"])?;
    assert_ne!(output1, output2);
    Ok(())
}

#[test]
fn test_generate_digest_arg() -> Result<()> {
    run_cli_expect(
        &["generate", "digest", "Hello"],
        "ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp",
    )
}

#[test]
fn test_generate_digest_stdin() -> Result<()> {
    run_cli_expect_stdin(
        &["generate", "digest"],
        "ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp",
        "Hello",
    )
}

#[test]
fn test_generate_key() -> Result<()> {
    bc_envelope::register_tags();

    let output1 = run_cli(&["generate", "key"])?;
    let key1 = bc_components::SymmetricKey::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "key"])?;
    let key2 = bc_components::SymmetricKey::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_nonce() -> Result<()> {
    bc_envelope::register_tags();

    let output1 = run_cli(&["generate", "nonce"])?;
    let nonce1 = bc_components::Nonce::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "nonce"])?;
    let nonce2 = bc_components::Nonce::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(nonce1, nonce2);
    Ok(())
}

#[test]
fn test_generate_seed() -> Result<()> {
    bc_envelope::register_tags();

    let output1 = run_cli(&["generate", "seed"])?;
    let seed1 = bc_components::Seed::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "seed"])?;
    let seed2 = bc_components::Seed::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(seed1, seed2);
    Ok(())
}

#[test]
fn test_generate_seed_with_count() -> Result<()> {
    bc_envelope::register_tags();

    let output = run_cli(&["generate", "seed", "--count", "32"])?;
    let seed = bc_components::Seed::from_ur_string(output.trim())?;
    assert_eq!(seed.as_bytes().len(), 32);
    Ok(())
}

#[test]
fn test_generate_seed_with_bad_count() -> Result<()> {
    assert!(run_cli(&["generate", "seed", "--count", "15"]).is_err());
    assert!(run_cli(&["generate", "seed", "--count", "257"]).is_err());
    Ok(())
}

#[test]
fn test_generate_seed_with_hex() -> Result<()> {
    bc_envelope::register_tags();

    let output = run_cli(&[
        "generate",
        "seed",
        "--hex",
        "7e31b2b14b895e75cdb82c22b013527c",
    ])?;

    assert_eq!(
        output,
        "ur:seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt"
    );

    let seed = bc_components::Seed::from_ur_string(output)?;
    assert_eq!(seed.as_bytes().len(), 16);
    assert_eq!(
        seed.as_bytes(),
        &hex::decode("7e31b2b14b895e75cdb82c22b013527c")?
    );
    Ok(())
}

#[test]
fn test_generate_prvkeys() -> Result<()> {
    bc_envelope::register_tags();

    let output1 = run_cli(&["generate", "prvkeys"])?;
    let key1 = bc_components::PrivateKeyBase::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "prvkeys"])?;
    let key2 = bc_components::PrivateKeyBase::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_prvkeys_from_seed() -> Result<()> {
    run_cli_expect(
        &[
            "generate",
            "prvkeys",
            "--seed",
            "ur:seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt",
        ],
        "ur:crypto-prvkey-base/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam",
    )
}

#[test]
fn test_generate_pubkeys() -> Result<()> {
    run_cli_expect(
        &[
            "generate",
            "pubkeys",
            "ur:crypto-prvkey-base/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam",
        ],
        "ur:crypto-pubkeys/lftanshfhdcxfpfwzcparpckfhvlidynjepsltsgjlprostpcmgehsmedtlbcktajodispgsfroytansgrhdcxenrytyrlpknyosfnfwlrwkdwsknduogwlyhdrfdrftflnnksbzsaierhbdrnrfbbfdvlwsca",
    )
}
