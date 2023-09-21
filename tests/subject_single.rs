use indoc::indoc;

mod common;
use common::run_cli;

#[test]
fn test_subject_single_arid_ur() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "arid", "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_arid_hex() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "arid", "08ec470b9af6f832f5c41343151bf1f806b123380fc2cfb1c391b8c8e48b69ca"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_cbor() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "cbor", "83010203"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcslsadaoaxgedmotks
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_data() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "data", "010203"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsfxadaoaxfniagtkb
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_date() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "date", "2022-08-30T07:16:11Z"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcssecyiabtrhfrpafdbzdy
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_date_no_time() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "date", "2022-08-30"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcssecyiabtguaeoxtdvdjp
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_digest() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "digest", "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_envelope() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "envelope", "ur:envelope/tpcsfyadaoaxaatitospwz"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsfyadaoaxaatitospwz
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_known_int() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "known", "1"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/adonahurcw
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_known_name() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "known", "isA"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/adonahurcw
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_number_float() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "number", "3.14"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcszofzasckrogywmlpctfggoreee
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_number_int() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "number", "42"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcscsdrldehwedp
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_string() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "string", "Hello"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcsihfdihjzjzjllgcllact
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_uri() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "uri", "https://example.com"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_uuid() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "uuid", "492ACBF4-13DC-4872-8A3B-4BF65C6BDF7C"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt
        "#}
    );
    Ok(())
}

#[test]
fn test_subject_single_wrapped() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli(&["subject", "single", "wrapped", "ur:envelope/tpcslsadaoaxgedmotks"], None)?;
    assert_eq!(
        output,
        indoc! {r#"
        ur:envelope/tpsptpcslsadaoaxqzsshsyl
        "#}
    );
    Ok(())
}
