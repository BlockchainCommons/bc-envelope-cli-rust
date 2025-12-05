#![allow(dead_code)]

use anyhow::{Result, bail};

/// A macro to assert that two values are equal, printing them if they are not,
/// including newlines and indentation they may contain. This macro is useful
/// for debugging tests where you want to see the actual and expected values
/// when they do not match.
#[macro_export]
macro_rules! assert_actual_expected {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val);
                }
            }
        }
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
                }
            }
        }
    };
}

pub const HELLO_STR: &str = "Hello.";
pub const HELLO_ENVELOPE_UR: &str = "ur:envelope/tpsoiyfdihjzjzjldmksbaoede";
pub const ARID_HEX: &str =
    "dec7e82893c32f7a4fcec633c02c0ec32a4361ca3ee3bc8758ae07742e940550";
pub const ARID: &str = "ur:arid/hdcxuestvsdemusrdlkngwtosweortdwbasrdrfxhssgfmvlrflthdplatjydmmwahgdwlflguqz";
pub const DATE_EXAMPLE: &str = "2022-08-30T07:16:11Z";
pub const DIGEST_EXAMPLE: &str = "ur:digest/hdcxdplutstarkhelprdiefhadbetlbnreamoyzefxnnkonycpgdehmuwdhnfgrkltylrovyeeck";
pub const SEED_UR_EXAMPLE: &str =
    "ur:seed/oyadgdaawzwplrbdhdpabgrnvokorolnrtemksayyadmut";
pub const UUID_EXAMPLE: &str = "eb377e65-5774-410a-b9cb-510bfc73e6d9";
pub const ALICE_KNOWS_BOB_EXAMPLE: &str =
    "ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns";
pub const CREDENTIAL_EXAMPLE: &str = "ur:envelope/lstpspmntpsotansgshdcxfgkoiahtjthnissawsfhzcmyyldsutfzcttefpaxjtmobsbwimcaleykvsdtgajnoytpsojsiaihjpjyiniyiniahsjyihglkpjnidihjptpsojeeheyeodpeeecendpemetesoytpsojtihksjoinjphsjyinjljtfyhsjyihtpsosecyjncscxaeoytpsoisjzhsjkjyglhsjnihtpsoiogthsksktihjzjzoytpsoininjkjkkpihfyhsjyihtpsosecyhybdvyaeoyadtpsokscffxihjpjyiniyiniahsjyihcxjliycxfxjljnjojzihjyinjljtoytpsoihjoisjljyjltpsoksckghisinjkcxinjkcxgehsjnihjkcxgthsksktihjzjzdijkcxjoisjljyjldmoytpsokscejojpjliyihjkjkinjljthsjzfyihkoihjzjljojnihjtjyfdjlkpjpjktpsobsoytpsoiniyinjpjkjyglhsjnihtpsoihgehsjnihjkoytpsoiyjyjljoiniajktpsolfingukpidimihiajycxehingukpidimihiajycxeyoytpsokscsiajljtjyinjtkpinjtiofeiekpiahsjyinjljtgojtinjyjktpsoadoyattpsoksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieoytpsoiojkkpidimihiajytpsokscegmfgcxhsjtiecxgtiniajpjlkthskoihcxfejtioinjtihihjpinjtiooybttpsoksdkfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpieoyaxtpsotansghhdfzdlmunbknwymowslbwfkidawyastikibksfhdosgslulecpwktysphprdheingyckvlrtjlrdhswnkbdereotdryapyhddpmnahcsmymnlsmtpdadsptyptmdbyosdllooyaatpsoksdmguiniojtihiecxidkkcxfekshsjnjojzihcxfejzihiajyjpiniahsjzcxfejtioinjtihihjpinjtiocxfwjlhsjpietdeoahrf";
pub const KEY_EXAMPLE: &str = "ur:crypto-key/hdcxmszmjlfsgssrbzehsslphdlgtbwesofnlpehlftldwotpaiyfwbtzsykwttomsbatnzswlqd";

pub const ALICE_ARID: &str = "ur:arid/hdcxtygshybkzcecfhflpfdlhdonotoentnydmzsidmkindlldjztdmoeyishknybtbswtgwwpdi";
pub const ALICE_SEED: &str =
    "ur:seed/oyadgdlfwfdwlphlfsghcphfcsaybekkkbaejkhphdfndy";
pub const ALICE_PRVKEY_BASE: &str =
    "ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct";
pub const ALICE_PRVKEYS: &str = "ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls";
pub const ALICE_PUBKEYS: &str = "ur:crypto-pubkeys/lftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnwksorlbd";

pub const BOB_ARID: &str = "ur:arid/hdcxdkreprfslewefgdwhtfnaosfgajpehhyrlcyjzheurrtamfsvolnaxwkioplgansesiabtdr";
pub const BOB_SEED: &str =
    "ur:seed/oyadgdcsknhkjkswgtecnslsjtrdfgimfyuykglfsfwtso";
pub const BOB_PRVKEY_BASE: &str =
    "ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva";
pub const BOB_PUBKEYS: &str = "ur:crypto-pubkeys/lftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwphejzwnmhhf";

pub const CAROL_ARID: &str = "ur:arid/hdcxamstktdsdlplurgaoxfxdijyjysertlpehwstkwkskmnnsqdpfgwlbsertvatbbtcaryrdta";
pub const CAROL_SEED: &str =
    "ur:seed/oyadgdlpjypepycsvodtihcecwvsyljlzevwcnmepllulo";
pub const CAROL_PRVKEY_BASE: &str =
    "ur:crypto-prvkey-base/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos";
pub const CAROL_PRVKEYS: &str = "ur:crypto-prvkeys/lftansgohdcxmorsytadihzswmckyltauyolecmevychhlwmtylbhsmdptfdrtuewnjtdkmnmkretansgehdcxhentsejphsfwclylihbwroaoisptaskegrimyldebecsdrrtbdlrrslazeursspmldtkmdds";
pub const CAROL_PUBKEYS: &str = "ur:crypto-pubkeys/lftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwsbkbotbs";

pub const DAVE_PRVKEY_BASE: &str = "ur:crypto-prvkey-base/hdcxjtgrwefxlpihpmvtzoprdpfrbaghgmfmdyjsiafzaewlenmktesweocpluwepekgdyutaejy";
pub const DAVE_PUBKEYS: &str = "ur:crypto-pubkeys/lftanshfhdcxbwbdwmehecntwdwdfgeyotrhplcejyglaacpotqzbtjslfoybdpyhpdpbasrytpatansgrhdcxptsnuebzqzwdhtlanbhyweprpytkpfntvyfpmomykkasfeltwyceuoieaysngrjtjndrescf";

pub fn run_cli_raw_stdin(args: &[&str], stdin: &str) -> Result<String> {
    let output = assert_cmd::cargo::cargo_bin_cmd!("envelope")
        .args(args)
        .write_stdin(stdin)
        .assert();

    if output.get_output().status.success() {
        Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
    } else {
        bail!(
            "Command failed: {:?}",
            String::from_utf8(output.get_output().stderr.to_vec()).unwrap()
        );
    }
}

pub fn run_cli_raw(args: &[&str]) -> Result<String> {
    run_cli_raw_stdin(args, "")
}

pub fn run_cli_raw_expect(args: &[&str], expected: &str) -> Result<()> {
    let output = run_cli_raw(args)?;
    if output != expected.trim() {
        bail!(
            "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
            expected,
            output
        );
    }
    assert_eq!(expected.trim(), output);
    Ok(())
}

pub fn run_cli_stdin(args: &[&str], stdin: &str) -> Result<String> {
    run_cli_raw_stdin(args, stdin).map(|s| s.trim().to_string())
}

pub fn run_cli(args: &[&str]) -> Result<String> { run_cli_stdin(args, "") }

pub fn run_cli_expect_stdin(
    args: &[&str],
    expected: &str,
    stdin: &str,
) -> Result<()> {
    let output = run_cli_stdin(args, stdin)?;
    if output != expected.trim() {
        bail!(
            "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
            expected,
            output
        );
    }
    assert_eq!(expected.trim(), output);
    Ok(())
}

pub fn run_cli_expect(args: &[&str], expected: &str) -> Result<()> {
    run_cli_expect_stdin(args, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_raw_piped_stdin(
    cmds: &[&[&str]],
    stdin: &str,
) -> Result<String> {
    let mut output = stdin.to_string();
    for cmd in cmds {
        output = run_cli_raw_stdin(cmd, &output)?;
    }
    Ok(output)
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_piped_stdin(cmds: &[&[&str]], stdin: &str) -> Result<String> {
    run_cli_raw_piped_stdin(cmds, stdin).map(|s| s.trim().to_string())
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_raw_piped_expect_stdin(
    cmds: &[&[&str]],
    expected: &str,
    stdin: &str,
) -> Result<()> {
    let output = run_cli_raw_piped_stdin(cmds, stdin)?;
    if output.trim() != expected.trim() {
        bail!(
            "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
            expected,
            output
        );
    }
    assert_eq!(expected.trim(), output.trim());
    Ok(())
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_piped_expect_stdin(
    cmds: &[&[&str]],
    expected: &str,
    stdin: &str,
) -> Result<()> {
    // run_cli_piped_stdin(cmds, stdin).map(|s| assert_eq!(expected, s))
    let output = run_cli_piped_stdin(cmds, stdin)?;
    if output != expected.trim() {
        bail!(
            "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
            expected,
            output
        );
    }
    assert_eq!(expected.trim(), output.trim());
    Ok(())
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_piped_expect(cmds: &[&[&str]], expected: &str) -> Result<()> {
    run_cli_piped_expect_stdin(cmds, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_raw_piped_expect(
    cmds: &[&[&str]],
    expected: &str,
) -> Result<()> {
    run_cli_raw_piped_expect_stdin(cmds, expected, "")
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_piped(cmds: &[&[&str]]) -> Result<String> {
    run_cli_piped_stdin(cmds, "")
}

/// Run each command in sequence, piping the output of the previous command to
/// the next command.
pub fn run_cli_raw_piped(cmds: &[&[&str]]) -> Result<String> {
    run_cli_raw_piped_stdin(cmds, "")
}
