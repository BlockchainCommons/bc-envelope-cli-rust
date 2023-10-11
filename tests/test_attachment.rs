use indoc::indoc;

mod common;
use common::*;

static SUBJECT: &str = "this-is-the-subject";
static PAYLOAD_V1: &str = "this-is-the-v1-payload";
static PAYLOAD_V2: &str = "this-is-the-v2-payload";
static VENDOR: &str = "com.example";
static CONFORMS_TO_V1: &str = "https://example.com/v1";
static CONFORMS_TO_V2: &str = "https://example.com/v2";

fn subject_envelope() -> anyhow::Result<String> {
    run_cli(&["subject", "type", "string", SUBJECT])
}

fn payload_v1_envelope() -> anyhow::Result<String> {
    run_cli(&["subject", "type", "string", PAYLOAD_V1])
}

fn payload_v2_envelope() -> anyhow::Result<String> {
    run_cli(&["subject", "type", "string", PAYLOAD_V2])
}

fn attachment_v1() -> anyhow::Result<String> {
    run_cli(&["attachment", "create", VENDOR, "--conforms-to", CONFORMS_TO_V1, &payload_v1_envelope()?])
}

fn attachment_v2() -> anyhow::Result<String> {
    run_cli(&["attachment", "create", VENDOR, "--conforms-to", CONFORMS_TO_V2, &payload_v2_envelope()?])
}

fn attachment_v1_no_conformance() -> anyhow::Result<String> {
    run_cli(&["attachment", "create", VENDOR, &payload_v1_envelope()?])
}

fn envelope_v1_v2() -> anyhow::Result<String> {
    run_cli_piped_stdin(&[
        &["attachment", "add", "envelope", &attachment_v1()?],
        &["attachment", "add", "envelope", &attachment_v2()?],
        ],
        &subject_envelope()?
    )
}

#[test]
fn test_attachment_create() -> anyhow::Result<()> {
    let attachment = attachment_v1()?;
    run_cli_expect(
        &["format", &attachment],
        indoc!(r#"
        'attachment': {
            "this-is-the-v1-payload"
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
    let attachment = attachment_v1_no_conformance()?;
    run_cli_expect(
        &["format", &attachment],
        indoc!(r#"
        'attachment': {
            "this-is-the-v1-payload"
        } [
            'vendor': "com.example"
        ]
        "#)
    )?;
    Ok(())
}

#[test]
fn test_attachment_queries() -> anyhow::Result<()> {
    let attachment = attachment_v1()?;
    let payload_env = run_cli(&["attachment", "payload", &attachment])?;
    assert_eq!(payload_env, payload_v1_envelope()?);
    let vendor = run_cli(&["attachment", "vendor", &attachment])?;
    assert_eq!(vendor, VENDOR);
    let conforms_to = run_cli(&["attachment", "conforms-to", &attachment])?;
    assert_eq!(conforms_to, CONFORMS_TO_V1);

    let attachment_no_conformance = attachment_v1_no_conformance()?;
    let conforms_to = run_cli(&["attachment", "conforms-to", &attachment_no_conformance])?;
    assert_eq!(conforms_to, "");
    Ok(())
}

#[test]
fn test_attachment_add_components() -> anyhow::Result<()> {
    run_cli_raw_piped_expect_stdin(
        &[
            &["attachment", "add", "components", VENDOR, "--conforms-to", CONFORMS_TO_V1, &payload_v1_envelope()?],
            &["attachment", "add", "components", VENDOR, "--conforms-to", CONFORMS_TO_V2, &payload_v2_envelope()?],
            &["format"]
        ],
        indoc!(r#"
        "this-is-the-subject" [
            'attachment': {
                "this-is-the-v1-payload"
            } [
                'conformsTo': "https://example.com/v1"
                'vendor': "com.example"
            ]
            'attachment': {
                "this-is-the-v2-payload"
            } [
                'conformsTo': "https://example.com/v2"
                'vendor': "com.example"
            ]
        ]
        "#),
        &subject_envelope()?,
    )?;

    Ok(())
}

#[test]
fn test_attachment_add_envelope() -> anyhow::Result<()> {
    run_cli_expect_stdin(
        &["format", &envelope_v1_v2()?],
        indoc!(r#"
        "this-is-the-subject" [
            'attachment': {
                "this-is-the-v1-payload"
            } [
                'conformsTo': "https://example.com/v1"
                'vendor': "com.example"
            ]
            'attachment': {
                "this-is-the-v2-payload"
            } [
                'conformsTo': "https://example.com/v2"
                'vendor': "com.example"
            ]
        ]
        "#),
        &subject_envelope()?,
    )?;

    Ok(())
}

#[test]
fn test_attachment_count() -> anyhow::Result<()> {
    run_cli_expect(
        &["attachment", "count", &envelope_v1_v2()?],
        "2"
    )
}

trait IntoLines {
    fn lines(&self) -> Vec<String>;
}

impl IntoLines for String {
    fn lines(&self) -> Vec<String> {
        let a = self.trim();
        if a.is_empty() {
            return vec![];
        }
        a.split('\n').map(|s| s.to_string()).collect::<Vec<_>>()
    }
}

#[test]
fn test_attachment_all() -> anyhow::Result<()> {
    let envelopes = run_cli(&["attachment", "all", &envelope_v1_v2()?])?.lines();
    assert_eq!(envelopes.len(), 2);
    assert_eq!(envelopes[0], attachment_v2()?);
    assert_eq!(envelopes[1], attachment_v1()?);
    Ok(())
}

#[test]
fn test_attachment_at() -> anyhow::Result<()> {
    run_cli_expect(
        &["attachment", "at", "0", &envelope_v1_v2()?],
        &attachment_v2()?
    )?;
    run_cli_expect(
        &["attachment", "at", "1", &envelope_v1_v2()?],
        &attachment_v1()?
    )?;
    assert!(run_cli(&["attachment", "at", "2", &envelope_v1_v2()?]).is_err());
    Ok(())
}

#[test]
fn test_attachment_find() -> anyhow::Result<()> {
    assert_eq!(run_cli(&["attachment", "find", &envelope_v1_v2()?])?.lines().len(), 2);

    assert_eq!(run_cli(&["attachment", "find", "--vendor", VENDOR, &envelope_v1_v2()?])?.lines().len(), 2);
    assert_eq!(run_cli(&["attachment", "find", "--vendor", "bar", &envelope_v1_v2()?])?.lines().len(), 0);

    assert_eq!(run_cli(&["attachment", "find", "--conforms-to", CONFORMS_TO_V1, &envelope_v1_v2()?])?.lines().len(), 1);
    assert_eq!(run_cli(&["attachment", "find", "--conforms-to", "foo", &envelope_v1_v2()?])?.lines().len(), 0);

    Ok(())
}
