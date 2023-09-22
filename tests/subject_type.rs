mod common;
use common::*;

#[test]
fn test_subject_type_arid_ur() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"],
        "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"
    )
}

#[test]
fn test_subject_type_arid_hex() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "08ec470b9af6f832f5c41343151bf1f806b123380fc2cfb1c391b8c8e48b69ca"],
        "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"
    )
}

#[test]
fn test_subject_type_cbor() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "cbor", "83010203"],
        "ur:envelope/tpcslsadaoaxgedmotks"
    )
}

#[test]
fn test_subject_type_data() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "data", "010203"],
        "ur:envelope/tpcsfxadaoaxfniagtkb"
    )
}

#[test]
fn test_subject_type_date() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30T07:16:11Z"],
        "ur:envelope/tpcssecyiabtrhfrpafdbzdy"
    )
}

#[test]
fn test_subject_type_date_no_time() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30"],
        "ur:envelope/tpcssecyiabtguaeoxtdvdjp"
    )
}

#[test]
fn test_subject_type_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "digest", "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"],
        "ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas"
    )
}

#[test]
fn test_subject_type_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "envelope", "ur:envelope/tpcsfyadaoaxaatitospwz"],
        "ur:envelope/tpcsfyadaoaxaatitospwz"
    )
}

#[test]
fn test_subject_type_known_int() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "1"],
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_known_name() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "isA"],
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_number_float() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "number", "3.14"],
        "ur:envelope/tpcszofzasckrogywmlpctfggoreee"
    )
}

#[test]
fn test_subject_type_number_int() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "number", "42"],
        "ur:envelope/tpcscsdrldehwedp"
    )
}

#[test]
fn test_subject_type_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "string", "Hello"],
        "ur:envelope/tpcsihfdihjzjzjllgcllact"
    )
}

#[test]
fn test_subject_type_uri() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uri", "https://example.com"],
        "ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse"
    )
}

#[test]
fn test_subject_type_uuid() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uuid", "492ACBF4-13DC-4872-8A3B-4BF65C6BDF7C"],
        "ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt"
    )
}

#[test]
fn test_subject_type_wrapped() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "wrapped", "ur:envelope/tpcslsadaoaxgedmotks"],
        "ur:envelope/tpsptpcslsadaoaxqzsshsyl"
    )
}
