#![allow(dead_code)]

use anyhow::bail;
use assert_cmd::Command;

pub const HELLO_STR: &str = "Hello.";
pub const HELLO_ENVELOPE_UR: &str = "ur:envelope/tpcsiyfdihjzjzjldmprrhtypk";
pub const ARID_HEX: &str = "dec7e82893c32f7a4fcec633c02c0ec32a4361ca3ee3bc8758ae07742e940550";
pub const ARID: &str = "ur:arid/hdcxuestvsdemusrdlkngwtosweortdwbasrdrfxhssgfmvlrflthdplatjydmmwahgdwlflguqz";
pub const DATE_EXAMPLE: &str = "2022-08-30T07:16:11Z";
pub const DIGEST_EXAMPLE: &str = "ur:digest/hdcxdplutstarkhelprdiefhadbetlbnreamoyzefxnnkonycpgdehmuwdhnfgrkltylrovyeeck";
pub const SEED_UR_EXAMPLE: &str = "ur:crypto-seed/oyadgdaawzwplrbdhdpabgrnvokorolnrtemksayyadmut";
pub const UUID_EXAMPLE: &str = "eb377e65-5774-410a-b9cb-510bfc73e6d9";
pub const ALICE_KNOWS_BOB_EXAMPLE: &str = "ur:envelope/lftpcsihfpjziniaihoytpcsihjejtjlktjktpcsiafwjliddssngwct";
pub const CREDENTIAL_EXAMPLE: &str = "ur:envelope/lstpspmntpcstansgshdcxfgkoiahtjthnissawsfhzcmyyldsutfzcttefpaxjtmobsbwimcaleykvsdtgajnoytpcsjsiaihjpjyiniyiniahsjyihglkpjnidihjptpcsjeeheyeodpeeecendpemetesoytpcsjtihksjoinjphsjyinjljtfyhsjyihtpcssecyjncscxaeoytpcsisjzhsjkjyglhsjnihtpcsiogthsksktihjzjzoytpcsininjkjkkpihfyhsjyihtpcssecyhybdvyaeoyadtpcskscffxihjpjyiniyiniahsjyihcxjliycxfxjljnjojzihjyinjljtoytpcsihjoisjljyjltpcsksckghisinjkcxinjkcxgehsjnihjkcxgthsksktihjzjzdijkcxjoisjljyjldmoytpcskscejojpjliyihjkjkinjljthsjzfyihkoihjzjljojnihjtjyfdjlkpjpjktpcsbsoytpcsiniyinjpjkjyglhsjnihtpcsihgehsjnihjkoytpcsiyjyjljoiniajktpcslfingukpidimihiajycxehingukpidimihiajycxeyoytpcskscsiajljtjyinjtkpinjtiofeiekpiahsjyinjljtgojtinjyjktpcsadoyattpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieoytpcsiojkkpidimihiajytpcskscegmfgcxhsjtiecxgtiniajpjlkthskoihcxfejtioinjtihihjpinjtiooybttpcsksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieoyaxtpcstansghhdfzdlmunbknwymowslbwfkidawyastikibksfhdosgslulecpwktysphprdheingyckvlrtjlrdhswnkbdereotdryapyhddpmnahcsmymnlsmtpdadsptyptmdbyosdllooyaatpcsksdmguiniojtihiecxidkkcxfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpielabtrdda";

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
    assert_eq!(expected.trim(), output);
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
    assert_eq!(expected.trim(), output);
    Ok(())
}

pub fn run_cli_expect(args: &[&str], expected: &str) -> anyhow::Result<()> {
    run_cli_expect_stdin(args, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_raw_piped_stdin(cmds: &[&[&str]], stdin: &str) -> anyhow::Result<String> {
    let mut output = stdin.to_string();
    for cmd in cmds {
        output = run_cli_raw_stdin(cmd, &output)?;
    }
    Ok(output)
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_piped_stdin(cmds: &[&[&str]], stdin: &str) -> anyhow::Result<String> {
    run_cli_raw_piped_stdin(cmds, stdin).map(|s| s.trim().to_string())
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_raw_piped_expect_stdin(cmds: &[&[&str]], expected: &str, stdin: &str) -> anyhow::Result<()> {
    run_cli_raw_piped_stdin(cmds, stdin).map(|s| assert_eq!(expected, s))
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_piped_expect_stdin(cmds: &[&[&str]], expected: &str, stdin: &str) -> anyhow::Result<()> {
    run_cli_piped_stdin(cmds, stdin).map(|s| assert_eq!(expected, s))
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_piped_expect(cmds: &[&[&str]], expected: &str) -> anyhow::Result<()> {
    run_cli_piped_expect_stdin(cmds, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_raw_piped_expect(cmds: &[&[&str]], expected: &str) -> anyhow::Result<()> {
    run_cli_raw_piped_expect_stdin(cmds, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_piped(cmds: &[&[&str]]) -> anyhow::Result<String> {
    run_cli_piped_stdin(cmds, "")
}

/// Run each command in sequence, piping the output of the previous command to the next command.
pub fn run_cli_raw_piped(cmds: &[&[&str]]) -> anyhow::Result<String> {
    run_cli_raw_piped_stdin(cmds, "")
}
