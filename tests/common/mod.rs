use anyhow::bail;
use assert_cmd::Command;

pub fn run_cli_raw(args: &[&str], stdin: Option<&str>) -> anyhow::Result<String> {
    let output = Command::cargo_bin("envelope").unwrap()
        .args(args)
        .write_stdin(stdin.unwrap_or_default())
        .assert();

    if output.get_output().status.success() {
        Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
    } else {
        bail!("Command failed: {:?}", String::from_utf8(output.get_output().stderr.to_vec()).unwrap());
    }
}

pub fn run_cli(args: &[&str], stdin: Option<&str>) -> anyhow::Result<String> {
    run_cli_raw(args, stdin).map(|s| s.trim().to_string())
}

#[allow(dead_code)]
pub fn run_cli_expect_stdin(args: &[&str], expected: &str, stdin: Option<&str>) -> anyhow::Result<()> {
    let output = run_cli(args, stdin)?;
    assert_eq!(output, expected);
    Ok(())
}

pub fn run_cli_expect(args: &[&str], expected: &str) -> anyhow::Result<()> {
    run_cli_expect_stdin(args, expected, None)
}
