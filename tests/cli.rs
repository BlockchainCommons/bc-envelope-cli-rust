use assert_cmd::prelude::*;
use std::process::Command;
use indoc::indoc;

#[test]
fn test_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envelope")?;
    cmd.arg("ur:envelope/tpcsihfdihjzjzjllgcllact");
    // let output = String::from_utf8(cmd.unwrap().stdout)?;
    // println!("{}", output);
    cmd.assert()
        .success()
        .stdout(indoc! {r#"
        "Hello"
        "#});

    Ok(())
}
