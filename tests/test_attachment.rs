use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_attachment_create() -> anyhow::Result<()> {
    let payload = run_cli(&["subject", "type", "string", "this-is-the-payload"])?;
    let attachment = run_cli(&["attachment", "create", "com.example", "--conforms-to", "https://example.com/v1", &payload])?;
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
