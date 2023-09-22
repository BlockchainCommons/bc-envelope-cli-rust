use bc_ur::URDecodable;
mod common;
use common::*;

#[test]
fn test_generate_arid() -> anyhow::Result<()> {
    let output1 = run_cli(&["generate", "arid"], None)?;
    let output2 = run_cli(&["generate", "arid"], None)?;
    assert_ne!(output1, output2);
    Ok(())
}

#[test]
fn test_generate_digest_arg() -> anyhow::Result<()> {
    run_cli_expect(
        &["generate", "digest", "Hello"],
        None,
        "ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp"
    )
}

#[test]
fn test_generate_digest_stdin() -> anyhow::Result<()> {
    run_cli_expect(
        &["generate", "digest"],
        Some("Hello"),
        "ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp"
    )
}

#[test]
fn test_generate_key() -> anyhow::Result<()> {
    let output1 = run_cli(&["generate", "key"], None)?;
    let key1 = bc_components::SymmetricKey::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "key"], None)?;
    let key2 = bc_components::SymmetricKey::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_nonce() -> anyhow::Result<()> {
    let output1 = run_cli(&["generate", "nonce"], None)?;
    let nonce1 = bc_components::Nonce::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "nonce"], None)?;
    let nonce2 = bc_components::Nonce::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(nonce1, nonce2);
    Ok(())
}

#[test]
fn test_generate_seed() -> anyhow::Result<()> {
    let output1 = run_cli(&["generate", "seed"], None)?;
    let seed1 = bc_components::Seed::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "seed"], None)?;
    let seed2 = bc_components::Seed::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(seed1, seed2);
    Ok(())
}

#[test]
fn test_generate_seed_with_count() -> anyhow::Result<()> {
    let output = run_cli(&["generate", "seed", "--count", "32"], None)?;
    let seed = bc_components::Seed::from_ur_string(output.trim())?;
    assert_eq!(seed.data().len(), 32);
    Ok(())
}


#[test]
fn test_generate_seed_with_bad_count() -> anyhow::Result<()> {
    assert!(run_cli(&["generate", "seed", "--count", "15"], None).is_err());
    assert!(run_cli(&["generate", "seed", "--count", "257"], None).is_err());
    Ok(())
}

#[test]
fn test_generate_seed_with_hex() -> anyhow::Result<()> {
    let output = run_cli(
        &["generate", "seed", "--hex", "7e31b2b14b895e75cdb82c22b013527c"],
        None
    )?;

    assert_eq!(output, "ur:crypto-seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt");

    let seed = bc_components::Seed::from_ur_string(output)?;
    assert_eq!(seed.data().len(), 16);
    assert_eq!(
        seed.data(),
        &hex::decode("7e31b2b14b895e75cdb82c22b013527c")?
    );
    Ok(())
}

#[test]
fn test_generate_prvkeys() -> anyhow::Result<()> {
    let output1 = run_cli(&["generate", "prvkeys"], None)?;
    let key1 = bc_components::PrivateKeyBase::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "prvkeys"], None)?;
    let key2 = bc_components::PrivateKeyBase::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_prvkeys_from_seed() -> anyhow::Result<()> {
    run_cli_expect(
        &["generate", "prvkeys", "--seed", "ur:crypto-seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt"],
        None,
        "ur:crypto-prvkeys/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam"
    )
}

#[test]
fn test_generate_pubkeys() -> anyhow::Result<()> {
    run_cli_expect(
        &["generate", "pubkeys", "ur:crypto-prvkeys/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam"],
        None,
        "ur:crypto-pubkeys/lftanshfhdcxfpfwzcparpckfhvlidynjepsltsgjlprostpcmgehsmedtlbcktajodispgsfroytansgrhdcxenrytyrlpknyosfnfwlrwkdwsknduogwlyhdrfdrftflnnksbzsaierhbdrnrfbbfdvlwsca"
    )
}
