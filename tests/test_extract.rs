mod common;
use common::*;

use bc_envelope::prelude::*;

const ALICE_KNOWS_BOB: &str = "ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns";

#[test]
fn test_extract_assertion() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_object() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_predicate() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_arid() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "arid", "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"],
        "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"
    )
}

#[test]
fn test_extract_cbor() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "cbor", "ur:envelope/tpcslsadaoaxgedmotks"],
        "83010203"
    )
}

#[test]
fn test_extract_data() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "data", "ur:envelope/tpcsfxadaoaxfniagtkb"],
        "010203"
    )
}

#[test]
fn test_extract_date() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "date", "ur:envelope/tpcssecyiabtrhfrpafdbzdy"],
        "2022-08-30T07:16:11Z"
    )?;
    run_cli_expect(
        &["extract", "date", "ur:envelope/tpcssecyiabtguaeoxtdvdjp"],
        "2022-08-30T00:00:00Z"
    )
}

#[test]
fn test_extract_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "digest", "ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas"],
        "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"
    )
}

#[test]
fn test_extract_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "envelope", ALICE_KNOWS_BOB],
        "ur:envelope/tpsoihfpjziniaihmebdmodl"
    )
}

#[test]
fn test_extract_known() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "known", "ur:envelope/adonahurcw"],
        "'isA'"
    )?;
    run_cli_expect(
        &["extract", "known", "ur:envelope/cfdyfyfwfpwzms"],
        "'12356'"
    )
}

#[test]
fn test_extract_number() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "number", "ur:envelope/tpcszofzasckrogywmlpctfggoreee"],
        "3.14"
    )?;
    run_cli_expect(
        &["extract", "number", "ur:envelope/tpcscsdrldehwedp"],
        "42"
    )
}

#[test]
fn test_extract_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "string", ALICE_KNOWS_BOB],
        "Alice"
    )
}

#[test]
fn test_extract_ur() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_uri() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "uri", "ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse"],
        "https://example.com"
    )
}

#[test]
fn test_extract_uuid() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "uuid", "ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt"],
        "492acbf4-13dc-4872-8a3b-4bf65c6bdf7c"
    )
}

#[test]
fn test_extract_wrapped() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "wrapped", "ur:envelope/tpsptpcslsadaoaxqzsshsyl"],
        "ur:envelope/tpsolsadaoaxzerkykme"
    )
}

#[test]
fn test_extract_assertion_subject() -> anyhow::Result<()> {
    let e = Envelope::new_assertion(known_values::NOTE, "This is a note.");
    let ur = e.ur_string();

    let predicate_envelope = "ur:envelope/aatljldnmw";
    let object_envelope = "ur:envelope/tpsojlghisinjkcxinjkcxhscxjtjljyihdmkkqdzops";
    let pred_obj_envelope = [predicate_envelope, object_envelope].join("\n");

    run_cli_expect(&["extract", "assertion", &ur], &pred_obj_envelope)?;
    run_cli_expect(&["extract", "predicate", &ur], predicate_envelope)?;
    run_cli_expect(&["extract", "object", &ur], object_envelope)
}
