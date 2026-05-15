mod common;

use common::*;

const XID_DOC: &str = "ur:xid/tpsplftpsotanshdhdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnoyaylftpsotansgylftanshfhdcxhslkfzemaylrwttynsdlghrydpmdfzvdglndloimaahykorefddtsguogmvlahqztansgrhdcxetlewzvlwyfdtobeytidosbamkswaomwwfyabakssakggegychesmerkcatekpcxoycsfncsfggmplgshd";
const XID_DOC_ID: &str = "ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw";

#[test]
fn issue_10_full_xid_doc_can_be_added_as_ur_object() {
    let endorsement =
        run_cli(&["subject", "type", "string", "Endorsement"]).unwrap();
    let with_xid = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "endorsementTarget",
        "ur",
        XID_DOC,
        &endorsement,
    ])
    .unwrap();

    let formatted = run_cli(&["format", &with_xid]).unwrap();

    assert!(!formatted.contains("<error:"), "{formatted}");
    assert!(
        formatted.contains(r#""endorsementTarget": XID("#),
        "{formatted}"
    );
}

#[test]
fn issue_20_xid_doc_can_be_added_as_envelope_object() {
    let overview =
        run_cli(&["subject", "type", "string", "bradovc8XIDFiles"]).unwrap();
    let with_xid = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "xid",
        "envelope",
        XID_DOC,
        &overview,
    ])
    .unwrap();

    let formatted = run_cli(&["format", &with_xid]).unwrap();

    assert!(!formatted.contains("<error:"), "{formatted}");
    assert!(formatted.contains(r#""xid": XID("#), "{formatted}");
}

#[test]
fn issue_11_xid_id_accepts_xid_document_with_custom_assertions() {
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        XID_DOC,
    ])
    .unwrap();

    run_cli_expect(&["xid", "id", &custom], XID_DOC_ID).unwrap();
}

#[test]
fn issue_13_xid_key_count_accepts_custom_assertions() {
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        XID_DOC,
    ])
    .unwrap();

    run_cli_expect(&["xid", "key", "count", &custom], "1").unwrap();
}

#[test]
fn issue_21_xid_key_commands_accept_elided_key_material() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "elide",
        "--nickname",
        "Alice",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();

    run_cli_expect(&["xid", "key", "count", &xid], "1").unwrap();

    let keys = run_cli(&["xid", "key", "all", &xid]).unwrap();
    let formatted = run_cli(&["format", &keys]).unwrap();
    assert!(formatted.contains("PublicKeys"), "{formatted}");
    assert!(formatted.contains("ELIDED"), "{formatted}");
}

#[test]
fn issue_14_xid_provenance_get_accepts_custom_assertions() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "include",
        "--generator",
        "include",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();
    let custom = run_cli(&[
        "assertion",
        "add",
        "pred-obj",
        "string",
        "customField",
        "string",
        "customValue",
        &xid,
    ])
    .unwrap();

    let provenance = run_cli(&["xid", "provenance", "get", &custom]).unwrap();

    assert!(provenance.starts_with("ur:provenance/"), "{provenance}");
}

#[test]
fn issue_16_wrong_password_reports_decryption_failure() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "encrypt",
        "--encrypt-password",
        "correct password",
        "--generator",
        "encrypt",
        "--sign",
        "inception",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();

    let result = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://example.com/xid.txt",
        "--verify",
        "inception",
        "--password",
        "wrong password",
        "--sign",
        "inception",
        "--private",
        "encrypt",
        "--generator",
        "encrypt",
        "--encrypt-password",
        "correct password",
        &xid,
    ]);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("could not decrypt the inception key"), "{err}");
}

#[test]
fn issue_17_xid_export_signs_when_requested() {
    let unsigned = run_cli(&[
        "xid",
        "new",
        "--private",
        "include",
        "--generator",
        "include",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();

    let exported = run_cli(&[
        "xid",
        "export",
        "--private",
        "elide",
        "--generator",
        "elide",
        "--sign",
        "inception",
        &unsigned,
    ])
    .unwrap();

    run_cli(&["xid", "id", "--verify", "inception", &exported]).unwrap();
}

#[test]
fn issue_18_xid_resolution_all_reads_signed_xid_subject() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "include",
        "--generator",
        "include",
        "--sign",
        "inception",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();
    let with_resolution = run_cli(&[
        "xid",
        "resolution",
        "add",
        "https://example.com/xid.txt",
        "--sign",
        "inception",
        &xid,
    ])
    .unwrap();

    run_cli_expect(
        &["xid", "resolution", "all", &with_resolution],
        "https://example.com/xid.txt",
    )
    .unwrap();
}

#[test]
fn issue_19_assertion_find_object_does_not_panic_on_elided_assertions() {
    let xid = run_cli(&[
        "xid",
        "new",
        "--private",
        "elide",
        "--nickname",
        "contract-key",
        ALICE_PRVKEY_BASE,
    ])
    .unwrap();
    let key = run_cli(&["xid", "key", "all", &xid]).unwrap();

    // expected-text-output-rubric:
    run_cli_piped_expect(
        &[
            &[
                "assertion",
                "find",
                "object",
                "string",
                "contract-key",
                &key,
            ],
            &["format"],
        ],
        "'nickname': \"contract-key\"",
    )
    .unwrap();
}
