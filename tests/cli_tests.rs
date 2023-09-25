use anyhow::Ok;
use indoc::indoc;

use bc_envelope::{known_values::*, prelude::*};

mod common;
use common::*;

#[test]
fn test_invalid_command() -> anyhow::Result<()> {
    assert!(run_cli(&["invalid"]).is_err());
    Ok(())
}

#[test]
fn test_invalid_data() -> anyhow::Result<()> {
    assert!(run_cli(&[
        "format",
        "ur:crypto-seed/oyadgdtokgdpwkrsonfdltvdwttsnddneonbmdbntakkss"
    ])
    .is_err());
    Ok(())
}

#[test]
fn test_format() -> anyhow::Result<()> {
    let expected_output = r#""Hello.""#;
    run_cli_expect(&["format", HELLO_ENVELOPE_UR], expected_output)?;
    run_cli_expect_stdin(&["format"], expected_output, HELLO_ENVELOPE_UR)
}

#[test]
fn test_extract_assertion_subject() -> anyhow::Result<()> {
    let e = Envelope::new_assertion(NOTE, "This is a note.");
    let ur = e.ur_string();

    let predicate_envelope = "ur:envelope/aatljldnmw";
    let object_envelope = "ur:envelope/tpcsjlghisinjkcxinjkcxhscxjtjljyihdmbamnatmn";
    let pred_obj_envelope = [predicate_envelope, object_envelope].join("\n");

    run_cli_expect(&["extract", "assertion", &ur], &pred_obj_envelope)?;
    run_cli_expect(&["extract", "predicate", &ur], predicate_envelope)?;
    run_cli_expect(&["extract", "object", &ur], object_envelope)
}

// ```swift
// func testCBORSubject() throws {
//     let cborArrayExample = CBOR.array([1, 2, 3]).cborData.hex
//     let e = try envelope("subject --cbor \(cborArrayExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcslsadaoaxgedmotks")
//     XCTAssertEqual(try envelope(e), "[1, 2, 3]")
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "83010203")

//     let e2 = try envelope("subject --cbor", inputLine: cborArrayExample)
//     XCTAssertEqual(e, e2)
// }
// ```

#[test]
fn test_cbor_subject() -> anyhow::Result<()> {
    let cbor_array_example = vec![1, 2, 3].cbor().hex();
    let e = run_cli(&["subject", "type", "cbor", &cbor_array_example])?;
    assert_eq!(e, "ur:envelope/tpcslsadaoaxgedmotks");
    run_cli_expect(&["format", &e], "[1, 2, 3]")?;
    run_cli_expect(&["extract", "cbor", &e], "83010203")?;
    run_cli_expect(&["subject", "type", "cbor", &cbor_array_example], &e)
}

// ```swift
// func testARIDSubject() throws {
//     let e = try envelope("subject --arid \(aridExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcstansgshdcxuestvsdemusrdlkngwtosweortdwbasrdrfxhssgfmvlrflthdplatjydmmwahgddrrlvarh")
//     XCTAssertEqual(try envelope(e), "ARID(\(aridExample.prefix(8)))")
//     XCTAssertEqual(try envelope("extract --arid \(e)"), aridExample)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "d99c4c5820dec7e82893c32f7a4fcec633c02c0ec32a4361ca3ee3bc8758ae07742e940550")
// }
// ```

#[test]
fn test_arid_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "arid", ARID_HEX])?;
    assert_eq!(e, "ur:envelope/tpcstansgshdcxuestvsdemusrdlkngwtosweortdwbasrdrfxhssgfmvlrflthdplatjydmmwahgddrrlvarh");
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
    assert_eq!(e, "ur:envelope/tpcsykeochcngd");
    run_cli_expect(&["format", &e], "true")?;
    run_cli_expect(&["extract", "bool", &e], "true")?;
    run_cli_expect(&["extract", "cbor", &e], "f5")?;
    Ok(())
}

// ```swift
// func testWrappedEnvelopeSubject() throws {
//     let e = try envelope("subject --wrapped \(helloEnvelopeUR)")
//     XCTAssertEqual(e, "ur:envelope/tpsptpcsiyfdihjzjzjldmvysrenfx")
//     XCTAssertEqual(try envelope(e),
//     """
//     {
//         "Hello."
//     }
//     """
//     )
//     XCTAssertEqual(try envelope("extract --wrapped \(e)"), helloEnvelopeUR)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "d8186648656c6c6f2e")
//     XCTAssertEqual(try envelope("extract --ur \(e)"), helloEnvelopeUR)
// }
// ```

#[test]
fn test_wrapped_envelope_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "wrapped", HELLO_ENVELOPE_UR])?;
    assert_eq!(e, "ur:envelope/tpsptpcsiyfdihjzjzjldmvysrenfx");
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        {
            "Hello."
        }
        "#)
    );
    run_cli_expect(&["extract", "wrapped", &e], HELLO_ENVELOPE_UR)?;
    run_cli_expect(&["extract", "cbor", &e], "d8186648656c6c6f2e")?;
    run_cli_expect(&["extract", "ur", &e], HELLO_ENVELOPE_UR)?;
    Ok(())
}

// ```swift
// func testDataSubject() throws {
//     let value = "cafebabe"
//     let e = try envelope("subject --data \(value)")
//     XCTAssertEqual(e, "ur:envelope/tpcsfysgzerdrntewsiecp")
//     XCTAssertEqual(try envelope(e), "Bytes(4)")
//     XCTAssertEqual(try envelope("extract --data \(e)"), value)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "44cafebabe")
// }
// ```

#[test]
fn test_data_subject() -> anyhow::Result<()> {
    let value = "cafebabe";
    let e = run_cli(&["subject", "type", "data", value])?;
    assert_eq!(e, "ur:envelope/tpcsfysgzerdrntewsiecp");
    run_cli_expect(&["format", &e], "Bytes(4)")?;
    run_cli_expect(&["extract", "data", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "44cafebabe")?;
    Ok(())
}

// ```swift
// func testDateSubject() throws {
//     let e = try envelope("subject --date \(dateExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcssecyiabtrhfrpafdbzdy")
//     XCTAssertEqual(try envelope(e), dateExample)
//     XCTAssertEqual(try envelope("extract --date \(e)"), dateExample)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "c11a630db93b")
// }
// ```

#[test]
fn test_date_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "date", DATE_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/tpcssecyiabtrhfrpafdbzdy");
    run_cli_expect(&["format", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "date", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "cbor", &e], "c11a630db93b")?;
    Ok(())
}

// ```swift
// func testDigestSubject() throws {
//     let e = try envelope("subject --digest \(digestExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcstansfphdcxdplutstarkhelprdiefhadbetlbnreamoyzefxnnkonycpgdehmuwdhnfgrkltylyngdieke")
//     XCTAssertEqual(try envelope(e), "Digest(2d8bd7d9)")
//     XCTAssertEqual(try envelope("extract --digest \(e)"), digestExample)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "d99c4158202d8bd7d9bb5f85ba643f0110d50cb506a1fe439e769a22503193ea6046bb87f7")
// }
// ```

#[test]
fn test_digest_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "digest", DIGEST_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/tpcstansfphdcxdplutstarkhelprdiefhadbetlbnreamoyzefxnnkonycpgdehmuwdhnfgrkltylyngdieke");
    run_cli_expect(&["format", &e], "Digest(2d8bd7d9)")?;
    run_cli_expect(&["extract", "digest", &e], DIGEST_EXAMPLE)?;
    run_cli_expect(
        &["extract", "cbor", &e],
        "d99c4158202d8bd7d9bb5f85ba643f0110d50cb506a1fe439e769a22503193ea6046bb87f7",
    )?;
    Ok(())
}

// ```swift
// func testFloatSubject() throws {
//     let value = "42.5"
//     let e = try envelope("subject --number \(value)")
//     XCTAssertEqual(e, "ur:envelope/tpcsytgygdmktysogr")
//     XCTAssertEqual(try envelope(e), value)
//     XCTAssertEqual(try envelope("extract --number \(e)"), value)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "f95150")
// }
// ```

#[test]
fn test_float_subject() -> anyhow::Result<()> {
    let value = "42.5";
    let e = run_cli(&["subject", "type", "number", value])?;
    assert_eq!(e, "ur:envelope/tpcsytgygdmktysogr");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "f95150")?;
    Ok(())
}

// ```swift
// func testIntSubject() throws {
//     let value = "42"
//     let e = try envelope("subject --number \(value)")
//     XCTAssertEqual(e, "ur:envelope/tpcscsdrldehwedp")
//     XCTAssertEqual(try envelope(e), value)
//     XCTAssertEqual(try envelope("extract --number \(e)"), value)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "182a")
// }
// ```

#[test]
fn test_int_subject() -> anyhow::Result<()> {
    let value = "42";
    let e = run_cli(&["subject", "type", "number", value])?;
    assert_eq!(e, "ur:envelope/tpcscsdrldehwedp");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "182a")?;
    Ok(())
}

// ```swift
// func testNegativeIntSubject() throws {
//     let value = "-42"
//     let e = try envelope("subject --number -- \(value)")
//     XCTAssertEqual(e, "ur:envelope/tpcsetdtlprfmkec")
//     XCTAssertEqual(try envelope(e), value)
//     XCTAssertEqual(try envelope("extract --number \(e)"), value)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "3829")
// }

#[test]
fn test_negative_int_subject() -> anyhow::Result<()> {
    // https://github.com/apple/swift-argument-parser/issues/31#issuecomment-593563022
    let value = "-42";
    let e = run_cli(&["subject", "type", "number", "--", value])?;
    assert_eq!(e, "ur:envelope/tpcsetdtlprfmkec");
    run_cli_expect(&["format", &e], value)?;
    run_cli_expect(&["extract", "number", &e], value)?;
    run_cli_expect(&["extract", "cbor", &e], "3829")?;
    Ok(())
}

// ```swift
// func testKnownValueSubject() throws {
//     let value = "note"
//     let e = try envelope("subject --known \(value)")
//     XCTAssertEqual(e, "ur:envelope/aatljldnmw")
//     XCTAssertEqual(try envelope(e), "note")
//     XCTAssertEqual(try envelope("extract --known \(e)"), "note")
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "d99c4004")
// }
// ```

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

// ```swift
// func testStringSubject() throws {
//     XCTAssertEqual(try envelope("subject Hello."), helloEnvelopeUR)
//     XCTAssertEqual(try envelope("subject --string Hello."), helloEnvelopeUR)
//     XCTAssertEqual(try envelope("extract \(helloEnvelopeUR)"), helloString)
//     XCTAssertEqual(try envelope("extract --cbor \(helloEnvelopeUR)"), "6648656c6c6f2e")

//     XCTAssertEqual(try pipe(["subject", "extract"], inputLine: helloString), helloString)
// }
// ```

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

// ```swift
// func testEnvelopeURSubject() throws {
//     let e = try envelope("subject --ur \(helloEnvelopeUR)")
//     XCTAssertEqual(e, "ur:envelope/tpsptpcsiyfdihjzjzjldmvysrenfx")
//     XCTAssertEqual(try envelope(e),
//         """
//         {
//             "Hello."
//         }
//         """
//     )
//     XCTAssertEqual(try envelope("extract --ur \(e)"), helloEnvelopeUR)
//     XCTAssertEqual(try envelope("extract --wrapped \(e)"), helloEnvelopeUR)
// }
// ```

#[test]
fn test_envelope_ur_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "ur", HELLO_ENVELOPE_UR])?;
    assert_eq!(e, "ur:envelope/tpsptpcsiyfdihjzjzjldmvysrenfx");
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

// ```swift
// func testKnownURSubject() throws {
//     let e = try envelope("subject --ur \(seedURExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcstaaddwoyadgdaawzwplrbdhdpabgrnvokorolnrtemksidtbcxgu")
//     XCTAssertEqual(try envelope(e),
//         """
//         crypto-seed(Map)
//         """
//     )
//     XCTAssertEqual(try envelope("extract --ur \(e)"), seedURExample)
// }
// ```

#[test]
fn test_known_ur_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "ur", SEED_UR_EXAMPLE])?;
    assert_eq!(
        e,
        "ur:envelope/tpcstaaddwoyadgdaawzwplrbdhdpabgrnvokorolnrtemksidtbcxgu"
    );
    assert_eq!(
        run_cli_raw(&["format", &e])?,
        indoc!(r#"
        crypto-seed(Map)
        "#)
    );
    run_cli_expect(&["extract", "ur", &e], SEED_UR_EXAMPLE)?;
    Ok(())
}

// ```swift
// func testUnknownURSubject() throws {
//     let unknownUR = "ur:unknown/oyadgdjlssmkcklgoskseodnyteofwwfylkiftjzamgrht"
//     let e = try envelope("subject --ur \(unknownUR) --tag 555")
//     XCTAssertEqual(e, "ur:envelope/tpcstaaodnoyadgdjlssmkcklgoskseodnyteofwwfylkiftnsjphsox")
//     XCTAssertEqual(try envelope(e),
//         """
//         555(Map)
//         """
//     )
//     XCTAssertEqual(try envelope("extract --ur \(e) --type unknown"), unknownUR)
// }
// ```

#[test]
fn test_unknown_ur_subject() -> anyhow::Result<()> {
    let unknown_ur = "ur:unknown/oyadgdjlssmkcklgoskseodnyteofwwfylkiftjzamgrht";
    let e = run_cli(&["subject", "type", "ur", "--ur-tag", "555", unknown_ur])?;
    assert_eq!(
        e,
        "ur:envelope/tpcstaaodnoyadgdjlssmkcklgoskseodnyteofwwfylkiftnsjphsox"
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

// ```swift
// func testUUIDSubject() throws {
//     let e = try envelope("subject --uuid \(uuidExample)")
//     XCTAssertEqual(e, "ur:envelope/tpcstpdagdwmemkbihhgjyfpbkrhsbgybdztjkvataspdsylpf")
//     XCTAssertEqual(try envelope(e), "UUID(\(uuidExample))")
//     XCTAssertEqual(try envelope("extract --uuid \(e)"), uuidExample)
//     XCTAssertEqual(try envelope("extract --cbor \(e)"), "d82550eb377e655774410ab9cb510bfc73e6d9")
// }
// ```

#[test]
fn test_uuid_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "uuid", UUID_EXAMPLE])?;
    assert_eq!(
        e,
        "ur:envelope/tpcstpdagdwmemkbihhgjyfpbkrhsbgybdztjkvataspdsylpf"
    );
    run_cli_expect(&["format", &e], &format!("UUID({})", UUID_EXAMPLE))?;
    run_cli_expect(&["extract", "uuid", &e], UUID_EXAMPLE)?;
    run_cli_expect(
        &["extract", "cbor", &e],
        "d82550eb377e655774410ab9cb510bfc73e6d9",
    )?;
    Ok(())
}

// ```swift
// func testAssertion() throws {
//     let e = try envelope("subject assertion Alpha Beta")
//     XCTAssertEqual(e, "ur:envelope/oytpcsihfpjzjoishstpcsiefwihjyhsptyngldp")
//     XCTAssertEqual(try envelope(e), #""Alpha": "Beta""#)
// }
// ```

#[test]
fn test_assertion() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "assertion", "string", "Alpha", "string", "Beta"])?;
    assert_eq!(e, "ur:envelope/oytpcsihfpjzjoishstpcsiefwihjyhsptyngldp");
    run_cli_expect(&["format", &e], r#""Alpha": "Beta""#)?;
    Ok(())
}

// ```swift
// func testAssertion2() throws {
//     let e = try envelope("subject assertion --number 1 --number 2")
//     XCTAssertEqual(e, "ur:envelope/oytpcsadtpcsaolpkbrsfs")
//     XCTAssertEqual(try envelope(e), "1: 2")
// }
// ```

#[test]
fn test_assertion_2() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "assertion", "number", "1", "number", "2"])?;
    assert_eq!(e, "ur:envelope/oytpcsadtpcsaolpkbrsfs");
    run_cli_expect(&["format", &e], "1: 2")?;
    Ok(())
}

// ```swift
// func testAssertion3() throws {
//     let e = try envelope("subject assertion --known note ThisIsANote.")
//     XCTAssertEqual(e, "ur:envelope/oyaatpcsjzghisinjkgajkfpgljljyihdmsnnbgahp")
//     XCTAssertEqual(try envelope(e), #"note: "ThisIsANote.""#)
// }
// ```

#[test]
fn test_assertion_3() -> anyhow::Result<()> {
    let e = run_cli(&[
        "subject",
        "assertion",
        "known",
        "note",
        "string",
        "ThisIsANote.",
    ])?;
    assert_eq!(e, "ur:envelope/oyaatpcsjzghisinjkgajkfpgljljyihdmsnnbgahp");
    run_cli_expect(&["format", &e], r#"'note': "ThisIsANote.""#)?;
    Ok(())
}

// ```swift
// func testAssertionAdd() throws {
//     let subject = try envelope("subject Alice")
//     let e = try envelope("assertion add knows Bob \(subject)")
//     XCTAssertEqual(e, aliceKnowsBobExample)
//     XCTAssertEqual(try envelope(e),
//         """
//         "Alice" [
//             "knows": "Bob"
//         ]
//         """
//     )
// }
// ```

#[test]
fn test_assertion_add() -> anyhow::Result<()> {
    let subject = run_cli(&["subject", "type", "string", "Alice"])?;
    run_cli_expect(
        &[
            "assertion",
            "add",
            "pred-obj",
            "string",
            "knows",
            "string",
            "Bob",
            &subject,
        ],
        ALICE_KNOWS_BOB_EXAMPLE,
    )?;
    run_cli_expect(
        &["format", ALICE_KNOWS_BOB_EXAMPLE],
        indoc!(r#"
        "Alice" [
            "knows": "Bob"
        ]
        "#),
    )?;
    Ok(())
}

// ```swift
// func testAssertionAdd2() throws {
//     let subject = try envelope("subject Alice")
//     let predicate = try envelope("subject knows")
//     let object = try envelope("subject Bob")
//     let e = try envelope("assertion --envelope \(predicate) --envelope \(object) \(subject)")
//     XCTAssertEqual(try envelope(e),
//         """
//         "Alice" [
//             "knows": "Bob"
//         ]
//         """
//     )
// }
// ```

#[test]
fn test_assertion_add_2() -> anyhow::Result<()> {
    let subject = run_cli(&["subject", "type", "string", "Alice"])?;
    let predicate = run_cli(&["subject", "type", "string", "knows"])?;
    let object = run_cli(&["subject", "type", "string", "Bob"])?;
    run_cli_expect(
        &[
            "assertion",
            "add",
            "pred-obj",
            "envelope",
            &predicate,
            "envelope",
            &object,
            &subject,
        ],
        ALICE_KNOWS_BOB_EXAMPLE,
    )?;
    run_cli_expect(
        &["format", ALICE_KNOWS_BOB_EXAMPLE],
        indoc!(r#"
        "Alice" [
            "knows": "Bob"
        ]
        "#),
    )?;
    Ok(())
}

// ```swift
// func testAssertionCount() throws {
//     let count = try envelope("assertion count \(aliceKnowsBobExample)")
//     XCTAssertEqual(count, "1")
// }
// ```

#[test]
fn test_assertion_count() -> anyhow::Result<()> {
    run_cli_expect(&["assertion", "count", ALICE_KNOWS_BOB_EXAMPLE], "1")
}

// ```swift
// func testAssertionCount2() throws {
//     let count = try envelope("assertion count \(credentialExample)")
//     XCTAssertEqual(count, "2")
// }
// ```

#[test]
fn test_assertion_count_2() -> anyhow::Result<()> {
    run_cli_expect(&["assertion", "count", CREDENTIAL_EXAMPLE], "2")
}

// ```swift
// func testAssertionCount3() throws {
//     let count = try pipe(["extract --wrapped", "assertion count"], inputLine: credentialExample)
//     XCTAssertEqual(count, "13")
// }
// ```

#[test]
fn test_assertion_count_3() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "count"]
        ],
        "13",
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionAt() throws {
//     let e = try envelope("assertion at 0 \(aliceKnowsBobExample)")
//     XCTAssertEqual(e, "ur:envelope/oytpcsihjejtjlktjktpcsiafwjlidmhaxgwio")
//     XCTAssertEqual(try envelope(e), #""knows": "Bob""#)
// }
// ```

#[test]
fn test_assertion_at() -> anyhow::Result<()> {
    let e = run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/oytpcsihjejtjlktjktpcsiafwjlidmhaxgwio");
    run_cli_expect(&["format", &e], r#""knows": "Bob""#)?;
    Ok(())
}

// ```swift
// func testAssertionAt2() throws {
//     let e = try pipe(["extract --wrapped", "assertion at 12"], inputLine: credentialExample)
//     XCTAssertEqual(try envelope(e), #"issuer: "Example Electrical Engineering Board""#)
// }
// ```

#[test]
fn test_assertion_at_2() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "at", "12"],
            &["format"],
        ],
        r#"'issuer': "Example Electrical Engineering Board""#,
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionAt3() throws {
//     let e = try pipe(["extract --wrapped", "assertion at 12", "extract --object", "extract"], inputLine: credentialExample)
//     XCTAssertEqual(e, "Example Electrical Engineering Board")
// }
// ```

#[test]
fn test_assertion_at_3() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "at", "12"],
            &["extract", "object"],
            &["extract", "string"],
        ],
        "Example Electrical Engineering Board",
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionAll() throws {
//     let assertions = try pipe(["extract --wrapped", "assertion all"], inputLine: credentialExample)
//     XCTAssertEqual(assertions,
//     """
//     ur:envelope/oytpcsjsiaihjpjyiniyiniahsjyihglkpjnidihjptpcsjeeheyeodpeeecendpemetesmtskgyzt
//     ur:envelope/oytpcsjtihksjoinjphsjyinjljtfyhsjyihtpcssecyjncscxaemupyjkaa
//     ur:envelope/oytpcsisjzhsjkjyglhsjnihtpcsiogthsksktihjzjzwshedtst
//     ur:envelope/oytpcsininjkjkkpihfyhsjyihtpcssecyhybdvyaeldwtsovs
//     ur:envelope/oyadtpcskscffxihjpjyiniyiniahsjyihcxjliycxfxjljnjojzihjyinjljtwdiyftes
//     ur:envelope/oytpcsihjoisjljyjltpcsksckghisinjkcxinjkcxgehsjnihjkcxgthsksktihjzjzdijkcxjoisjljyjldmbaghdstp
//     ur:envelope/oytpcskscejojpjliyihjkjkinjljthsjzfyihkoihjzjljojnihjtjyfdjlkpjpjktpcsbsbdjyeeby
//     ur:envelope/oytpcsiniyinjpjkjyglhsjnihtpcsihgehsjnihjklkpmjngm
//     ur:envelope/oytpcsiyjyjljoiniajktpcslfingukpidimihiajycxehingukpidimihiajycxeyhnnegwax
//     ur:envelope/oytpcskscsiajljtjyinjtkpinjtiofeiekpiahsjyinjljtgojtinjyjktpcsadbygssbue
//     ur:envelope/oyattpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpiedlmdssse
//     ur:envelope/oytpcsiojkkpidimihiajytpcskscegmfgcxhsjtiecxgtiniajpjlkthskoihcxfejtioinjtihihjpinjtiotlbdctwd
//     ur:envelope/oybttpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieasqdlbto
//     """
//     )
// }
// ```

#[test]
fn test_assertion_all() -> anyhow::Result<()> {
    run_cli_raw_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "all"],
        ],
        indoc!(r#"
        ur:envelope/oytpcsjsiaihjpjyiniyiniahsjyihglkpjnidihjptpcsjeeheyeodpeeecendpemetesmtskgyzt
        ur:envelope/oytpcsjtihksjoinjphsjyinjljtfyhsjyihtpcssecyjncscxaemupyjkaa
        ur:envelope/oytpcsisjzhsjkjyglhsjnihtpcsiogthsksktihjzjzwshedtst
        ur:envelope/oytpcsininjkjkkpihfyhsjyihtpcssecyhybdvyaeldwtsovs
        ur:envelope/oyadtpcskscffxihjpjyiniyiniahsjyihcxjliycxfxjljnjojzihjyinjljtwdiyftes
        ur:envelope/oytpcsihjoisjljyjltpcsksckghisinjkcxinjkcxgehsjnihjkcxgthsksktihjzjzdijkcxjoisjljyjldmbaghdstp
        ur:envelope/oytpcskscejojpjliyihjkjkinjljthsjzfyihkoihjzjljojnihjtjyfdjlkpjpjktpcsbsbdjyeeby
        ur:envelope/oytpcsiniyinjpjkjyglhsjnihtpcsihgehsjnihjklkpmjngm
        ur:envelope/oytpcsiyjyjljoiniajktpcslfingukpidimihiajycxehingukpidimihiajycxeyhnnegwax
        ur:envelope/oytpcskscsiajljtjyinjtkpinjtiofeiekpiahsjyinjljtgojtinjyjktpcsadbygssbue
        ur:envelope/oyattpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpiedlmdssse
        ur:envelope/oytpcsiojkkpidimihiajytpcskscegmfgcxhsjtiecxgtiniajpjlkthskoihcxfejtioinjtihihjpinjtiotlbdctwd
        ur:envelope/oybttpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieasqdlbto
        "#),
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionPredicateFind1() throws {
//     let e = try pipe(["extract --wrapped", "assertion find predicate photo"], inputLine: credentialExample)
//     XCTAssertEqual(e, "ur:envelope/oytpcsihjoisjljyjltpcsksckghisinjkcxinjkcxgehsjnihjkcxgthsksktihjzjzdijkcxjoisjljyjldmbaghdstp")
//     XCTAssertEqual(try envelope(e), #""photo": "This is James Maxwell's photo.""#)
// }
// ```

#[test]
fn test_assertion_predicate_find_1() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "find", "predicate", "string", "photo"],
            &["format"]
        ],
        r#""photo": "This is James Maxwell's photo.""#,
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionPredicateFind2() throws {
//     let e = try pipe(["extract --wrapped", "assertion find predicate --known isA"], inputLine: credentialExample)
//     XCTAssertEqual(e, "ur:envelope/oyadtpcskscffxihjpjyiniyiniahsjyihcxjliycxfxjljnjojzihjyinjljtwdiyftes")
//     XCTAssertEqual(try envelope(e), #"isA: "Certificate of Completion""#)
// }
// ```

#[test]
fn test_assertion_predicate_find_2() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "find", "predicate", "known", "isA"],
            &["format"]
        ],
        r#"'isA': "Certificate of Completion""#,
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testAssertionObjectFind1() throws {
//     let e = try pipe(["extract --wrapped", "assertion find object Maxwell"], inputLine: credentialExample)
//     XCTAssertEqual(e, "ur:envelope/oytpcsisjzhsjkjyglhsjnihtpcsiogthsksktihjzjzwshedtst")
//     XCTAssertEqual(try envelope(e), #""lastName": "Maxwell""#)
// }
// ```

#[test]
fn test_assertion_object_find_1() -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(
        &[
            &["extract", "wrapped"],
            &["assertion", "find", "object", "string", "Maxwell"],
            &["format"]
        ],
        r#""lastName": "Maxwell""#,
        CREDENTIAL_EXAMPLE,
    )
}

// ```swift
// func testEnvelopeDigest() throws {
//     let d = try envelope("digest \(aliceKnowsBobExample)")
//     XCTAssertEqual(d, "ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds")
// }
// ```

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

// ```swift
// func testElide1() throws {
//     var target: [String] = []
//     // Top level
//     target.append(try envelope("digest \(aliceKnowsBobExample)"))
//     // Subject
//     target.append(try pipe(["extract --envelope \(aliceKnowsBobExample)", "digest"]))
//     // Assertion
//     let assertion = try envelope("assertion at 0 \(aliceKnowsBobExample)")
//     target.append(try envelope("digest \(assertion)"))
//     // Object
//     let object = try envelope("extract --object \(assertion)")
//     target.append(try envelope("digest \(object)"))

//     let digests = target.joined(separator: " ")
//     let elided = try envelope("elide \(aliceKnowsBobExample) \(digests)")
//     XCTAssertEqual(elided, "ur:envelope/lftpcsihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpcsiafwjlidcyhydiwe")
//     XCTAssertEqual(try envelope(elided),
//     """
//     "Alice" [
//         ELIDED: "Bob"
//     ]
//     """
//     )
// }
// ```

#[test]
fn test_elide_1() -> anyhow::Result<()> {
    let mut target = vec![];
    // Top level
    target.push(run_cli(&["digest", ALICE_KNOWS_BOB_EXAMPLE])?);
    // Subject
    target.push(run_cli_piped(&[&["extract", "envelope", ALICE_KNOWS_BOB_EXAMPLE], &["digest"]])?);
    // Assertion
    let assertion = run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    target.push(run_cli(&["digest", &assertion])?);
    // Object
    target.push(run_cli_piped(&[&["extract", "object", &assertion], &["digest"]])?);

    let digests = target.join(" ");
    let elided = run_cli(&["elide", "revealing", &digests, ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(
        elided,
        "ur:envelope/lftpcsihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpcsiafwjlidcyhydiwe"
    );
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
        "Alice" [
            ELIDED: "Bob"
        ]
        "#)
    )?;
    Ok(())
}

// ```swift
// func testElide2() throws {
//     var target: [String] = []
//     target.append(try pipe(["subject knows", "digest"]))
//     let digests = target.joined(separator: " ")
//     let elided = try envelope("elide removing \(aliceKnowsBobExample) \(digests)")
//     XCTAssertEqual(elided, "ur:envelope/lftpcsihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpcsiafwjlidcyhydiwe")
//     XCTAssertEqual(try envelope(elided),
//     """
//     "Alice" [
//         ELIDED: "Bob"
//     ]
//     """
//     )
// }
// ```

#[test]
fn test_elide_2() -> anyhow::Result<()> {
    let target = vec![
        run_cli_piped(&[&["subject", "type", "string", "knows"], &["digest"]])?
    ];
    let digests = target.join(" ");
    let elided = run_cli(&["elide", "removing", &digests, ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(
        elided,
        "ur:envelope/lftpcsihfpjziniaihoyhdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrdtpcsiafwjlidcyhydiwe"
    );
    run_cli_expect(
        &["format", &elided],
        indoc!(r#"
        "Alice" [
            ELIDED: "Bob"
        ]
        "#)
    )?;
    Ok(())
}

/// ```swift
// func testEncrypt() throws {
//     let encrypted = try envelope("encrypt \(aliceKnowsBobExample) --key \(keyExample)")
//     XCTAssertEqual(try envelope(encrypted),
//     """
//     ENCRYPTED [
//         "knows": "Bob"
//     ]
//     """
//     )
//     let decrypted = try envelope("decrypt \(encrypted) --key \(keyExample)")
//     XCTAssertEqual(decrypted, aliceKnowsBobExample)
// }
// ```

#[test]
fn test_encrypt() -> anyhow::Result<()> {
    let encrypted = run_cli(&[
        "encrypt",
        "--key",
        KEY_EXAMPLE,
        ALICE_KNOWS_BOB_EXAMPLE
    ])?;
    run_cli_expect(
        &["format", &encrypted],
        indoc!(r#"
        ENCRYPTED [
            "knows": "Bob"
        ]
        "#)
    )?;
    let decrypted = run_cli(&[
        "decrypt",
        "--key",
        KEY_EXAMPLE,
        &encrypted,
    ])?;
    assert_eq!(decrypted, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

// ```swift
// func testGeneratePrivateKeys1() throws {
//     let prvkeys = try envelope("generate prvkeys")
//     XCTAssertEqual(try UR(urString: prvkeys).type, "crypto-prvkeys")
// }
// ```

#[test]
fn test_generate_private_keys_1() -> anyhow::Result<()> {
    let prvkeys = run_cli(&["generate", "prvkeys"])?;
    assert_eq!(UR::from_ur_string(prvkeys)?.ur_type(), "crypto-prvkeys");
    Ok(())
}

// ```swift
// func testGeneratePrivateKeys2() throws {
//     let seed = "ur:crypto-seed/oyadhdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdweocfztrd"
//     let prvkeys1 = try envelope("generate prvkeys \(seed)")
//     XCTAssertEqual(prvkeys1, "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah")
//     let prvkeys2 = try envelope("generate prvkeys \(seed)")
//     XCTAssertEqual(prvkeys1, prvkeys2)

//     let pubkeys = try envelope("generate pubkeys \(prvkeys1)")
//     XCTAssertEqual(pubkeys, "ur:crypto-pubkeys/lftanshfhdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejobebtdlhd")
// }
// ```

#[test]
fn test_generate_private_keys_2() -> anyhow::Result<()> {
    let seed = "ur:crypto-seed/oyadhdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdweocfztrd";
    let prvkeys1 = run_cli(&["generate", "prvkeys", "--seed", seed])?;
    assert_eq!(
        prvkeys1,
        "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah"
    );
    let prvkeys2 = run_cli(&["generate", "prvkeys", "--seed", seed])?;
    assert_eq!(prvkeys1, prvkeys2);

    let pubkeys = run_cli(&["generate", "pubkeys", &prvkeys1])?;
    assert_eq!(
        pubkeys,
        "ur:crypto-pubkeys/lftanshfhdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejobebtdlhd"
    );
    Ok(())
}

// ```swift
// func testSign() throws {
//     let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah"
//     let signed = try envelope("sign \(aliceKnowsBobExample) --prvkeys \(prvkeys)")
//     XCTAssertEqual(try envelope(signed),
//     """
//     "Alice" [
//         "knows": "Bob"
//         verifiedBy: Signature
//     ]
//     """
//     )

//     let pubkeys = try envelope("generate pubkeys \(prvkeys)")

//     XCTAssertNoThrow(try envelope("verify \(signed) --pubkeys \(pubkeys)"))

//     XCTAssertThrowsError(try envelope("verify \(aliceKnowsBobExample) --pubkeys \(pubkeys)"))

//     let badPubkeys = try pipe(["generate prvkeys", "generate pubkeys"])
//     XCTAssertThrowsError(try envelope("verify \(signed) --pubkeys \(badPubkeys)"))
// }
// ```

#[test]
fn test_sign() -> anyhow::Result<()> {
    let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let signed = run_cli(&[
        "sign",
        "--prvkeys",
        prvkeys,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;
    run_cli_expect(
        &["format", &signed],
        indoc!(r#"
        "Alice" [
            "knows": "Bob"
            'verifiedBy': Signature
        ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;

    run_cli(&["verify", &signed, "--pubkeys", &pubkeys])?;

    assert!(run_cli(&["verify", ALICE_KNOWS_BOB_EXAMPLE, "--pubkeys", &pubkeys]).is_err());

    let bad_prvkeys = run_cli(&["generate", "prvkeys"])?;
    let bad_pubkeys = run_cli(&["generate", "pubkeys", &bad_prvkeys])?;
    assert!(run_cli(&["verify", &signed, "--pubkeys", &bad_pubkeys, &signed]).is_err());

    Ok(())
}

// ```swift
// func testSign2() throws {
//     let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah"
//     let wrappedSigned = try pipe(["subject --wrapped \(aliceKnowsBobExample)", "sign --prvkeys \(prvkeys)"])
//     XCTAssertEqual(try envelope(wrappedSigned),
//     """
//     {
//         "Alice" [
//             "knows": "Bob"
//         ]
//     } [
//         verifiedBy: Signature
//     ]
//     """
//     )

//     let pubkeys = try envelope("generate pubkeys \(prvkeys)")
//     XCTAssertNoThrow(try envelope("verify \(wrappedSigned) --pubkeys \(pubkeys)"))
// }
// ```

#[test]
fn test_sign_2() -> anyhow::Result<()> {
    let prvkeys = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
    let wrapped_signed = run_cli_piped(&[
        &["subject", "type", "wrapped", ALICE_KNOWS_BOB_EXAMPLE],
        &["sign", "--prvkeys", prvkeys]
    ])?;
    run_cli_expect(
        &["format", &wrapped_signed],
        indoc!(r#"
        {
            "Alice" [
                "knows": "Bob"
            ]
        } [
            'verifiedBy': Signature
        ]
        "#)
    )?;

    let pubkeys = run_cli(&["generate", "pubkeys", prvkeys])?;
    run_cli(&["verify", &wrapped_signed, "--pubkeys", &pubkeys])?;
    Ok(())
}

// ```swift
// func testSign3() throws {
//     let e = try pipe(["subject \(helloString)", "sign --prvkeys \(alicePrvkeys) --prvkeys \(carolPrvkeys)"])
//     XCTAssertEqual(try envelope(e),
//     """
//     "Hello." [
//         verifiedBy: Signature
//         verifiedBy: Signature
//     ]
//     """
//     )
// }
// ```

#[test]
fn test_sign_3() -> anyhow::Result<()> {
    let e = run_cli_piped(&[
        &["subject", "type", "string", "Hello."],
        &["sign", "--prvkeys", ALICE_PRVKEYS, "--prvkeys", CAROL_PRVKEYS]
    ])?;
    run_cli_expect(
        &["format", &e],
        indoc!(r#"
        "Hello." [
            'verifiedBy': Signature
            'verifiedBy': Signature
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_compress_1() -> anyhow::Result<()> {
    let compressed = run_cli(&["compress", "--subject", ALICE_KNOWS_BOB_EXAMPLE])?;

    run_cli_expect(
        &["format", &compressed],
        indoc!(r#"
        COMPRESSED [
            "knows": "Bob"
        ]
        "#)
    )?;
    let decompressed = run_cli(&["uncompress", "--subject", &compressed])?;
    assert_eq!(decompressed, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

#[test]
fn test_compress_2() -> anyhow::Result<()> {
    let compressed = run_cli(&["compress", CREDENTIAL_EXAMPLE])?;

    println!("{} {}", CREDENTIAL_EXAMPLE.len(), compressed.len());
    assert_eq!(CREDENTIAL_EXAMPLE.len(), 1210);
    assert_eq!(compressed.len(), 1032);

    run_cli_expect(
        &["format", &compressed],
        "COMPRESSED"
    )?;
    let decompressed = run_cli(&["uncompress", &compressed])?;
    assert_eq!(decompressed, CREDENTIAL_EXAMPLE);
    Ok(())
}

#[test]
fn test_salt() -> anyhow::Result<()> {
    let salted = run_cli(&["salt", ALICE_KNOWS_BOB_EXAMPLE])?;

    run_cli_expect(
        &["format", &salted],
        indoc!(r#"
        "Alice" [
            "knows": "Bob"
            'salt': Salt
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_assertion_create() -> anyhow::Result<()> {
    let assertion = run_cli(&[
        "assertion",
        "create",
        "--salted",
        "string",
        "knows",
        "string",
        "Bob",
    ])?;

    run_cli_expect(
        &["format", &assertion],
        indoc!(r#"
        {
            "knows": "Bob"
        } [
            'salt': Salt
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_assertion_remove_envelope() -> anyhow::Result<()> {
    let assertion = run_cli(&[
        "assertion",
        "at",
        "0",
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    let removed = run_cli(&[
        "assertion",
        "remove",
        "envelope",
        &assertion,
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    run_cli_expect(
        &["format", &removed],
        indoc!(r#"
        "Alice"
        "#)
    )?;
    Ok(())
}

#[test]
fn test_assertion_remove_pred_obj() -> anyhow::Result<()> {
    let removed = run_cli(&[
        "assertion",
        "remove",
        "pred-obj",
        "string",
        "knows",
        "string",
        "Bob",
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;

    run_cli_expect(
        &["format", &removed],
        indoc!(r#"
        "Alice"
        "#)
    )?;
    Ok(())
}
