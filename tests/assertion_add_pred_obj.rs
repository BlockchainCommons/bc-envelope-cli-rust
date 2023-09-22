mod common;
use common::*;

#[test]
fn test_assertion_add_pred_obj() -> anyhow::Result<()> {
    let subject = run_cli(
        &["subject", "type", "string", "Hello"],
        None
    )?;
    run_cli_expect(
        &["assertion", "add", "pred-obj", "known", "note", "string", "This is the note.", &subject],
        "ur:envelope/lftpcsihfdihjzjzjloyaatpcsjsghisinjkcxinjkcxjyisihcxjtjljyihdmtshlgycm"
    )?;
    run_cli_expect_stdin(
        &["assertion", "add", "pred-obj", "known", "note", "string", "This is the note."],
        "ur:envelope/lftpcsihfdihjzjzjloyaatpcsjsghisinjkcxinjkcxjyisihcxjtjljyihdmtshlgycm",
        Some(&subject)
    )
}
