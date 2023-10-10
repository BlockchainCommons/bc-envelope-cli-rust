use indoc::indoc;

mod common;
use common::*;

static PAYLOAD: &str = "this-is-the-payload";
static VENDOR: &str = "com.example";
static CONFORMS_TO: &str = "https://example.com/v1";

fn payload_envelope() -> anyhow::Result<String> {
    run_cli(&["subject", "type", "string", PAYLOAD])
}

fn attachment() -> anyhow::Result<String> {
    run_cli(&["attachment", "create", VENDOR, "--conforms-to", CONFORMS_TO, &payload_envelope()?])
}

fn attachment_no_conformance() -> anyhow::Result<String> {
    run_cli(&["attachment", "create", VENDOR, &payload_envelope()?])
}

#[test]
fn test_attachment_create() -> anyhow::Result<()> {
    let attachment = attachment()?;
    run_cli_expect(
        &["format", &attachment],
        indoc!(r#"
        'attachment': {
            "this-is-the-payload"
        } [
            'conformsTo': "https://example.com/v1"
            'vendor': "com.example"
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_attachment_create_no_conformance() -> anyhow::Result<()> {
    let attachment = attachment_no_conformance()?;
    run_cli_expect(
        &["format", &attachment],
        indoc!(r#"
        'attachment': {
            "this-is-the-payload"
        } [
            'vendor': "com.example"
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_attachment_queries() -> anyhow::Result<()> {
    let attachment = attachment()?;
    let payload_env = run_cli(&["attachment", "payload", &attachment])?;
    assert_eq!(payload_env, payload_envelope()?);
    let vendor = run_cli(&["attachment", "vendor", &attachment])?;
    assert_eq!(vendor, VENDOR);
    let conforms_to = run_cli(&["attachment", "conforms-to", &attachment])?;
    assert_eq!(conforms_to, CONFORMS_TO);

    let attachment_no_conformance = attachment_no_conformance()?;
    let conforms_to = run_cli(&["attachment", "conforms-to", &attachment_no_conformance])?;
    assert_eq!(conforms_to, "");
    Ok(())
}
