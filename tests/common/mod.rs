use assert_cmd::Command;

pub fn run_cli(args: &[&str], stdin: Option<&str>) -> Result<String, String> {
    let output = Command::cargo_bin("envelope").unwrap()
        .args(args)
        .write_stdin(stdin.unwrap_or_default())
        .assert();

    if output.get_output().status.success() {
        Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
    } else {
        Err(String::from_utf8(output.get_output().stderr.to_vec()).unwrap())
    }
}
