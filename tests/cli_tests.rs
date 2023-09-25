use anyhow::Ok;
use indoc::indoc;

use bc_envelope::prelude::*;

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
fn test_cbor_subject() -> anyhow::Result<()> {
    let cbor_array_example = vec![1, 2, 3].cbor().hex();
    let e = run_cli(&["subject", "type", "cbor", &cbor_array_example])?;
    assert_eq!(e, "ur:envelope/tpcslsadaoaxgedmotks");
    run_cli_expect(&["format", &e], "[1, 2, 3]")?;
    run_cli_expect(&["extract", "cbor", &e], "83010203")?;
    run_cli_expect(&["subject", "type", "cbor", &cbor_array_example], &e)
}

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

#[test]
fn test_date_subject() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "type", "date", DATE_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/tpcssecyiabtrhfrpafdbzdy");
    run_cli_expect(&["format", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "date", &e], DATE_EXAMPLE)?;
    run_cli_expect(&["extract", "cbor", &e], "c11a630db93b")?;
    Ok(())
}

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

#[test]
fn test_assertion() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "assertion", "string", "Alpha", "string", "Beta"])?;
    assert_eq!(e, "ur:envelope/oytpcsihfpjzjoishstpcsiefwihjyhsptyngldp");
    run_cli_expect(&["format", &e], r#""Alpha": "Beta""#)?;
    Ok(())
}

#[test]
fn test_assertion_2() -> anyhow::Result<()> {
    let e = run_cli(&["subject", "assertion", "number", "1", "number", "2"])?;
    assert_eq!(e, "ur:envelope/oytpcsadtpcsaolpkbrsfs");
    run_cli_expect(&["format", &e], "1: 2")?;
    Ok(())
}

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

#[test]
fn test_assertion_count() -> anyhow::Result<()> {
    run_cli_expect(&["assertion", "count", ALICE_KNOWS_BOB_EXAMPLE], "1")
}

#[test]
fn test_assertion_count_2() -> anyhow::Result<()> {
    run_cli_expect(&["assertion", "count", CREDENTIAL_EXAMPLE], "2")
}

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

#[test]
fn test_assertion_at() -> anyhow::Result<()> {
    let e = run_cli(&["assertion", "at", "0", ALICE_KNOWS_BOB_EXAMPLE])?;
    assert_eq!(e, "ur:envelope/oytpcsihjejtjlktjktpcsiafwjlidmhaxgwio");
    run_cli_expect(&["format", &e], r#""knows": "Bob""#)?;
    Ok(())
}

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

#[test]
fn test_generate_private_keys_1() -> anyhow::Result<()> {
    let prvkeys = run_cli(&["generate", "prvkeys"])?;
    assert_eq!(UR::from_ur_string(prvkeys)?.ur_type(), "crypto-prvkeys");
    Ok(())
}

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

#[test]
fn test_sskr_1() -> anyhow::Result<()> {
    let result = run_cli(&["sskr", "split", ALICE_KNOWS_BOB_EXAMPLE])?;
    run_cli_expect(
        &["format", &result],
        indoc!(r#"
        ENCRYPTED [
            'sskrShare': SSKRShare
        ]
        "#)
    )?;
    let restored = run_cli(&["sskr", "join", &result])?;
    assert_eq!(restored, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}

#[test]
fn test_sskr_2() -> anyhow::Result<()> {
    let result = run_cli(&[
        "sskr",
        "split",
        "-t",
        "2",
        "-g",
        "2-of-3",
        "-g",
        "2-of-3",
        ALICE_KNOWS_BOB_EXAMPLE,
    ])?;
    let shares = result.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
    let indexes = [0, 1, 4, 5];
    let recovered_shares = indexes.iter().map(|i| shares[*i].clone()).collect::<Vec<_>>();

    let mut args1 = vec!["sskr", "join"];
    args1.extend(recovered_shares.iter().map(|s| s.as_str()));
    let restored1 = run_cli(&args1)?;
    assert_eq!(restored1, ALICE_KNOWS_BOB_EXAMPLE);

    let restored2 = run_cli_stdin(&["sskr", "join"], &recovered_shares.join("\n"))?;
    assert_eq!(restored2, ALICE_KNOWS_BOB_EXAMPLE);
    Ok(())
}
