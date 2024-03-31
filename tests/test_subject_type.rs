use indoc::indoc;

mod common;
use common::*;

use bc_envelope::prelude::*;

#[test]
fn test_subject_type_arid_1() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"],
        "ur:envelope/tpsotansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgfejejkyk"
    )
}

#[test]
fn test_subject_type_arid_2() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "arid", "08ec470b9af6f832f5c41343151bf1f806b123380fc2cfb1c391b8c8e48b69ca"],
        "ur:envelope/tpsotansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgfejejkyk"
    )
}

#[test]
fn test_subject_type_cbor() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "cbor", "83010203"],
        "ur:envelope/tpsolsadaoaxzerkykme"
    )
}

#[test]
fn test_subject_type_data() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "data", "010203"],
        "ur:envelope/tpsofxadaoaxloyncwms"
    )
}

#[test]
fn test_subject_type_date_1() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30T07:16:11Z"],
        "ur:envelope/tpsosecyiabtrhfrrfztcase"
    )
}

#[test]
fn test_subject_type_date_2() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "date", "2022-08-30"],
        "ur:envelope/tpsosecyiabtguaeptiywsls"
    )
}

#[test]
fn test_subject_type_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "digest", "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"],
        "ur:envelope/tpsotansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasplnecbehy"
    )
}

#[test]
fn test_subject_type_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "envelope", "ur:envelope/tpcsfyadaoaxaatitospwz"],
        "ur:envelope/tpsofyadaoaxaaaspsatks"
    )
}

#[test]
fn test_subject_type_know_1() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "1"],
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_known_2() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "known", "isA"],
        "ur:envelope/adonahurcw"
    )
}

#[test]
fn test_subject_type_number_1() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "number", "3.14"],
        "ur:envelope/tpsozofzasckrogywmlpctynlngyfx"
    )
}

#[test]
fn test_subject_type_number_2() -> anyhow::Result<()> {
        run_cli_expect(
        &["subject", "type", "number", "42"],
        "ur:envelope/tpsocsdrahknprdr"
    )
}

#[test]
fn test_subject_type_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "string", "Hello"],
        "ur:envelope/tpsoihfdihjzjzjllamdlowy"
    )
}

#[test]
fn test_subject_type_uri() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uri", "https://example.com"],
        "ur:envelope/tpsotpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljnrlsrpsas"
    )
}

#[test]
fn test_subject_type_uuid() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "uuid", "492ACBF4-13DC-4872-8A3B-4BF65C6BDF7C"],
        "ur:envelope/tpsotpdagdgadrsbwkbwuofdjplefrgrynhhjeurkeflkgehwt"
    )
}

#[test]
fn test_subject_type_wrapped() -> anyhow::Result<()> {
    run_cli_expect(
        &["subject", "type", "wrapped", "ur:envelope/tpcslsadaoaxgedmotks"],
        "ur:envelope/tpsptpsolsadaoaxaegyemck"
    )
}

#[test]
fn test_cbor_subject() -> anyhow::Result<()> {
    let cbor_array_example = vec![1, 2, 3].cbor().hex();
    let e = run_cli(&["subject", "type", "cbor", &cbor_array_example])?;
    assert_eq!(e, "ur:envelope/tpsolsadaoaxzerkykme");
    run_cli_expect(&["format", &e], "[1, 2, 3]")?;
    run_cli_expect(&["extract", "cbor", &e], "83010203")?;
    run_cli_expect(&["subject", "type", "cbor", &cbor_array_example], &e)
}

#[test]
fn test_arid_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "arid", ARID_HEX])?;
    assert_eq!(e, "ur:envelope/tpsotansgshdcxuestvsdemusrdlkngwtosweortdwbasrdrfxhssgfmvlrflthdplatjydmmwahgdaabzoswy");
    run_cli_expect(
        &["format", &e],
        &format!("ARID({})", ARID_HEX.get(..8).unwrap()),
    )?;
    run_cli_expect(&["extract", "arid", &e], ARID)?;
    run_cli_expect(&["extract", "arid-hex", &e], ARID_HEX)?;
    run_cli_expect(
        &["extract", "cbor", &e],
        "d99c4c5820dec7e82893c32f7a4fcec633c02c0ec32a4361ca3ee3bc8758ae07742e940550",
    )?;

    let e2 = run_cli(&["subject", "type", "arid", ARID])?;
    assert_eq!(e, e2);
    Ok(())
}

#[test]
fn test_bool_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "bool", "true"])?;
    assert_eq!(e, "ur:envelope/tpsoykpyeetsba");
    run_cli_expect(&["format", &e], "true")?;
    run_cli_expect(&["extract", "bool", &e], "true")?;
    run_cli_expect(&["extract", "cbor", &e], "f5")?;
    Ok(())
}

#[test]
fn test_wrapped_envelope_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "wrapped", HELLO_ENVELOPE_UR])?;
    assert_eq!(e, "ur:envelope/tpsptpsoiyfdihjzjzjldmdnjyfzse");
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        {
            "Hello."
        }
        "#)
    );
    run_cli_expect(&["extract", "wrapped", &e], HELLO_ENVELOPE_UR)?;
    run_cli_expect(&["extract", "cbor", &e], "d8c96648656c6c6f2e")?;
    run_cli_expect(&["extract", "ur", &e], HELLO_ENVELOPE_UR)?;
    Ok(())
}

#[test]
fn test_data_subject() -> anyhow::Result<()> {
    let value = "cafebabe";
    let e = run_cli(&["subject", "type", "data", value])?;
    assert_eq!(e, "ur:envelope/tpsofysgzerdrnbklgpypd");
    run_cli_expect(&["format", &e], "Bytes(4)")?;
    run_cli_expect(&["extract", "data", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "44cafebabe")?;
    Ok(())
}

#[test]
fn test_date_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "date", DATE_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/tpsosecyiabtrhfrrfztcase");
    run_cli_expect(&["format", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "date", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "cbor", &e], "c11a630db93b")?;
    Ok(())
}

#[test]
fn test_digest_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "digest", DIGEST_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/tpsotansfphdcxdplutstarkhelprdiefhadbetlbnreamoyzefxnnkonycpgdehmuwdhnfgrkltyltpwzdadn");
    run_cli_expect(&["format", &e], "Digest(2d8bd7d9)")?;
    run_cli_expect(&["extract", "digest", &e], DIGEST_EXAMPLE)?;
    run_cli_expect(
        &["extract", "cbor", &e],
        "d99c4158202d8bd7d9bb5f85ba643f0110d50cb506a1fe439e769a22503193ea6046bb87f7",
    )?;
    Ok(())
}

#[test]
fn test_float_subject() -> anyhow::Result<()> {
    let value = "42.5";
    let e = run_cli(&["subject", "type", "number", value])?;
    assert_eq!(e, "ur:envelope/tpsoytgygdamfnchrl");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "f95150")?;
    Ok(())
}

#[test]
fn test_int_subject() -> anyhow::Result<()> {
    let value = "42";
    let e = run_cli(&["subject", "type", "number", value])?;
    assert_eq!(e, "ur:envelope/tpsocsdrahknprdr");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "182a")?;
    Ok(())
}

#[test]
fn test_negative_int_subject() -> anyhow::Result<()> {
    // https://github.com/apple/swift-argument-parser/issues/31#issuecomment-593563022
    let value = "-42";
    let e = run_cli(&["subject", "type", "number", "--", value])?;
    assert_eq!(e, "ur:envelope/tpsoetdtasylstey");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "3829")?;
    Ok(())
}

#[test]
fn test_known_value_subject() -> anyhow::Result<()> {
    let value = "note";
    let e = run_cli(&["subject", "type", "known", value])?;
    assert_eq!(e, "ur:envelope/aatljldnmw");
    run_cli_expect(&["format", &e], "'note'")?;
    run_cli_expect(&["extract", "known", &e], "'note'")?;
    run_cli_expect(&["extract", "cbor", &e], "d99c4004")?;
    Ok(())
}

#[test]
fn test_string_subject() -> anyhow::Result<()> {
    run_cli_expect(&["subject", "type", "string", HELLO_STR], HELLO_ENVELOPE_UR)?;
    run_cli_expect(&["extract", "string", HELLO_ENVELOPE_UR], HELLO_STR)?;
    run_cli_expect(&["extract", "cbor", HELLO_ENVELOPE_UR], "6648656c6c6f2e")?;
    run_cli_piped_expect(
        &[
            &["subject", "type", "string", HELLO_STR],
            &["extract", "string"],
        ],
        HELLO_STR,
    )
}

#[test]
fn test_envelope_ur_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "ur", HELLO_ENVELOPE_UR])?;
    assert_eq!(e, "ur:envelope/tpsptpsoiyfdihjzjzjldmdnjyfzse");
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        {
            "Hello."
        }
        "#)
    );
    run_cli_expect(&["extract", "ur", &e], HELLO_ENVELOPE_UR)?;
    run_cli_expect(&["extract", "wrapped", &e], HELLO_ENVELOPE_UR)?;
    Ok(())
}

#[test]
fn test_known_ur_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "ur", SEED_UR_EXAMPLE])?;
    assert_eq!(
        e,
        "ur:envelope/tpsotantjzoyadgdaawzwplrbdhdpabgrnvokorolnrtemkslgdpfebs"
    );
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        seed(Map)
        "#)
    );
    run_cli_expect(&["extract", "ur", &e], SEED_UR_EXAMPLE)?;
    Ok(())
}

#[test]
fn test_unknown_ur_subject() -> anyhow::Result<()> {
    let unknown_ur = "ur:unknown/oyadgdjlssmkcklgoskseodnyteofwwfylkiftjzamgrht";
    let e = run_cli(&["subject", "type", "ur", "--ur-tag", "555", unknown_ur])?;
    assert_eq!(
        e,
        "ur:envelope/tpsotaaodnoyadgdjlssmkcklgoskseodnyteofwwfylkiftaydpdsjz"
    );
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        555(Map)
        "#)
    );
    run_cli_expect(&["extract", "ur", &e, "--ur-type", "unknown"], unknown_ur)?;
    Ok(())
}

#[test]
fn test_uuid_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "uuid", UUID_EXAMPLE])?;
    assert_eq!(
        e,
        "ur:envelope/tpsotpdagdwmemkbihhgjyfpbkrhsbgybdztjkvatabwmnltwl"
    );
    run_cli_expect(&["format", &e], &format!("UUID({})", UUID_EXAMPLE))?;
    run_cli_expect(&["extract", "uuid", &e], UUID_EXAMPLE)?;
    run_cli_expect(
        &["extract", "cbor", &e],
        "d82550eb377e655774410ab9cb510bfc73e6d9",
    )?;
    Ok(())
}
