use indoc::indoc;

mod common;
use common::run_cli;

#[test]
fn test_subject_type_arid_ur() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "arid", "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_arid_hex() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "arid", "08ec470b9af6f832f5c41343151bf1f806b123380fc2cfb1c391b8c8e48b69ca"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_cbor() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "cbor", "83010203"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcslsadaoaxgedmotks
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_data() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "data", "010203"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsfxadaoaxfniagtkb
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_date() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "date", "2022-08-30T07:16:11Z"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcssecyiabtrhfrpafdbzdy
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_date_no_time() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "date", "2022-08-30"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcssecyiabtguaeoxtdvdjp
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_digest() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "digest", "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_envelope() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "envelope", "ur:envelope/tpcsfyadaoaxaatitospwz"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsfyadaoaxaatitospwz
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_known_int() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "known", "1"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/adonahurcw
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_known_name() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "known", "isA"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/adonahurcw
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_number_float() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "number", "3.14"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcszofzasckrogywmlpctfggoreee
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_number_int() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "number", "42"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcscsdrldehwedp
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_string() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "string", "Hello"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsihfdihjzjzjllgcllact
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_uri() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "uri", "https://example.com"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_uuid() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "uuid", "492ACBF4-13DC-4872-8A3B-4BF65C6BDF7C"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_type_wrapped() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "type", "wrapped", "ur:envelope/tpcslsadaoaxgedmotks"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpsptpcslsadaoaxqzsshsyl
        "#}
    );
    Ok(())
}
