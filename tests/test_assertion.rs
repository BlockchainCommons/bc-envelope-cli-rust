use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_assertion_add_pred_obj() -> anyhow::Result<()> {
    let subject = run_cli(&["subject", "type", "string", "Hello"])?;
    run_cli_expect(
        &["assertion", "add", "pred-obj", "known", "note", "string", "This is the note.", &subject],
        "ur:envelope/lftpcsihfdihjzjzjloyaatpcsjsghisinjkcxinjkcxjyisihcxjtjljyihdmtshlgycm"
    )?;
    run_cli_expect_stdin(
        &["assertion", "add", "pred-obj", "known", "note", "string", "This is the note."],
        "ur:envelope/lftpcsihfdihjzjzjloyaatpcsjsghisinjkcxinjkcxjyisihcxjtjljyihdmtshlgycm",
        &subject
    )
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
