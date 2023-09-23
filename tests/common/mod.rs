#![allow(dead_code)]

use anyhow::bail;
use assert_cmd::Command;

pub fn run_cli_raw_stdin(args: &[&str], stdin: &str) -> anyhow::Result<String> {
    let output = Command::cargo_bin("envelope").unwrap()
        .args(args)
        .write_stdin(stdin)
        .assert();

    if output.get_output().status.success() {
        Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
    } else {
        bail!("Command failed: {:?}", String::from_utf8(output.get_output().stderr.to_vec()).unwrap());
    }
}

pub fn run_cli_raw(args: &[&str]) -> anyhow::Result<String> {
    run_cli_raw_stdin(args, "")
}

pub fn run_cli_raw_expect(args: &[&str], expected: &str) -> anyhow::Result<()> {
    let output = run_cli_raw(args)?;
    assert_eq!(expected, output);
    Ok(())
}

pub fn run_cli_stdin(args: &[&str], stdin: &str) -> anyhow::Result<String> {
    run_cli_raw_stdin(args, stdin).map(|s| s.trim().to_string())
}

pub fn run_cli(args: &[&str]) -> anyhow::Result<String> {
    run_cli_stdin(args, "")
}

pub fn run_cli_expect_stdin(args: &[&str], expected: &str, stdin: &str) -> anyhow::Result<()> {
    let output = run_cli_stdin(args, stdin)?;
    assert_eq!(expected, output);
    Ok(())
}

pub fn run_cli_expect(args: &[&str], expected: &str) -> anyhow::Result<()> {
    run_cli_expect_stdin(args, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_raw_piped_expect_stdin(cmds: &[&[&str]], expected: &str, stdin: &str) -> anyhow::Result<()> {
    let mut output = stdin.to_string();
    for cmd in cmds {
        output = run_cli_raw_stdin(cmd, &output)?;
    }
    assert_eq!(expected, output);
    Ok(())
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_piped_expect_stdin(cmds: &[&[&str]], expected: &str, stdin: &str) -> anyhow::Result<()> {
    let mut output = stdin.to_string();
    for cmd in cmds {
        output = run_cli_stdin(cmd, &output)?;
    }
    assert_eq!(expected, output);
    Ok(())
}

pub fn run_cli_piped_expect(cmds: &[&[&str]], expected: &str) -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(cmds, expected, "")
}
