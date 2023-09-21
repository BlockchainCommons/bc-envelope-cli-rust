use bc_ur::URDecodable;
use indoc::indoc;

mod common;
use common::run_cli;

#[test]
fn test_generate_arid() -> Result<(), Box<dyn std::error::Error>> {
    let output1 = run_cli(&["generate", "arid"], None)?;
    let output2 = run_cli(&["generate", "arid"], None)?;
    assert_ne!(output1, output2);
    Ok(())
}

#[test]
fn test_generate_digest_arg() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["generate", "digest", "Hello"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp
        "#}
    );
    Ok(())
}

#[test]
fn test_generate_digest_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["generate", "digest"], Some("Hello"))?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:digest/hdcxcshelgqdcpjszedaykhsolztmuludmdsfxamwpdygltngylaatttkofddsetcfinrkcltpsp
        "#}
    );
    Ok(())
}

#[test]
fn test_generate_key() -> Result<(), Box<dyn std::error::Error>> {
    let output1 = run_cli(&["generate", "key"], None)?;
    let key1 = bc_components::SymmetricKey::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "key"], None)?;
    let key2 = bc_components::SymmetricKey::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_nonce() -> Result<(), Box<dyn std::error::Error>> {
    let output1 = run_cli(&["generate", "nonce"], None)?;
    let nonce1 = bc_components::Nonce::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "nonce"], None)?;
    let nonce2 = bc_components::Nonce::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(nonce1, nonce2);
    Ok(())
}

#[test]
fn test_generate_seed() -> Result<(), Box<dyn std::error::Error>> {
    let output1 = run_cli(&["generate", "seed"], None)?;
    let seed1 = bc_components::Seed::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "seed"], None)?;
    let seed2 = bc_components::Seed::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(seed1, seed2);
    Ok(())
}

#[test]
fn test_generate_seed_with_count() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["generate", "seed", "--count", "32"], None)?;
    let seed = bc_components::Seed::from_ur_string(output.trim())?;
    assert_eq!(seed.data().len(), 32);
    Ok(())
}


#[test]
fn test_generate_seed_with_bad_count() -> Result<(), Box<dyn std::error::Error>> {
    assert!(run_cli(&["generate", "seed", "--count", "15"], None).is_err());
    assert!(run_cli(&["generate", "seed", "--count", "257"], None).is_err());
    Ok(())
}

#[test]
fn test_generate_seed_with_hex() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(
        &["generate", "seed", "--hex", "7e31b2b14b895e75cdb82c22b013527c"],
        None,
    )?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:crypto-seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt
        "#}
    );
    let seed = bc_components::Seed::from_ur_string(output.trim())?;
    assert_eq!(seed.data().len(), 16);
    assert_eq!(
        seed.data(),
        &hex::decode("7e31b2b14b895e75cdb82c22b013527c")?
    );
    Ok(())
}

#[test]
fn test_generate_prvkeys() -> Result<(), Box<dyn std::error::Error>> {
    let output1 = run_cli(&["generate", "prvkeys"], None)?;
    let key1 = bc_components::PrivateKeyBase::from_ur_string(output1.trim())?;
    let output2 = run_cli(&["generate", "prvkeys"], None)?;
    let key2 = bc_components::PrivateKeyBase::from_ur_string(output2.trim())?;

    assert_ne!(output1, output2);
    assert_ne!(key1, key2);
    Ok(())
}

#[test]
fn test_generate_prvkeys_from_seed() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(
        &["generate", "prvkeys", "--seed", "ur:crypto-seed/oyadgdkbehprpagrldhykpsnrodwcppfbwgmkemtaolbdt"],
        None,
    )?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:crypto-prvkeys/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam
        "#}
    );
    Ok(())
}

#[test]
fn test_generate_pubkeys() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(
        &["generate", "pubkeys", "ur:crypto-prvkeys/gdkbehprpagrldhykpsnrodwcppfbwgmkeadrturam"],
        None,
    )?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:crypto-pubkeys/lftanshfhdcxfpfwzcparpckfhvlidynjepsltsgjlprostpcmgehsmedtlbcktajodispgsfroytansgrhdcxenrytyrlpknyosfnfwlrwkdwsknduogwlyhdrfdrftflnnksbzsaierhbdrnrfbbfdvlwsca
        "#}
    );
    Ok(())
}
