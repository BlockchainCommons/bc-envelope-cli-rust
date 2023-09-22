mod common;
use common::*;

#[test]
fn test_subject_type_arid_ur() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"],
        None,
        "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"
    )
}

#[test]
fn test_subject_type_arid_hex() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "08ec470b9af6f832f5c41343151bf1f806b123380fc2cfb1c391b8c8e48b69ca"],
        None,
        "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"
    )
}

#[test]
fn test_subject_type_cbor() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "cbor", "83010203"],
        None,
        "ur:envelope/tpcslsadaoaxgedmotks"
    )
}

#[test]
fn test_subject_type_data() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "data", "010203"],
        None,
        "ur:envelope/tpcsfxadaoaxfniagtkb"
    )
}

#[test]
fn test_subject_type_date() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30T07:16:11Z"],
        None,
        "ur:envelope/tpcssecyiabtrhfrpafdbzdy"
    )
}

#[test]
fn test_subject_type_date_no_time() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30"],
        None,
        "ur:envelope/tpcssecyiabtguaeoxtdvdjp"
    )
}

#[test]
fn test_subject_type_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "digest", "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"],
        None,
        "ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas"
    )
}

#[test]
fn test_subject_type_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "envelope", "ur:envelope/tpcsfyadaoaxaatitospwz"],
        None,
        "ur:envelope/tpcsfyadaoaxaatitospwz"
    )
}

#[test]
fn test_subject_type_known_int() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "1"],
        None,
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_known_name() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "isA"],
        None,
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_number_float() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "number", "3.14"],
        None,
        "ur:envelope/tpcszofzasckrogywmlpctfggoreee"
    )
}

#[test]
fn test_subject_type_number_int() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "number", "42"],
        None,
        "ur:envelope/tpcscsdrldehwedp"
    )
}

#[test]
fn test_subject_type_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "string", "Hello"],
        None,
        "ur:envelope/tpcsihfdihjzjzjllgcllact"
    )
}

#[test]
fn test_subject_type_uri() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uri", "https://example.com"],
        None,
        "ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse"
    )
}

#[test]
fn test_subject_type_uuid() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uuid", "492ACBF4-13DC-4872-8A3B-4BF65C6BDF7C"],
        None,
        "ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt"
    )
}

#[test]
fn test_subject_type_wrapped() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "wrapped", "ur:envelope/tpcslsadaoaxgedmotks"],
        None,
        "ur:envelope/tpsptpcslsadaoaxqzsshsyl"
    )
}
