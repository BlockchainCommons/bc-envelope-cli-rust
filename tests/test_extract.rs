mod common;
use common::*;

use bc_envelope::prelude::*;

const ALICE_KNOWS_BOB: &str = "ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns";

#[test]
fn test_extract_assertion() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_object() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_predicate() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_arid() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "arid", "ur:envelope/tpcstansgshdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgjesoeyoe"],
        "ur:arid/hdcxaywpflbdnyynyaeyykssbwfxbzcwwnyaampacnetbssatkpasrmerospveluinsgpdesltpe"
    )
}

#[test]
fn test_extract_cbor() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "cbor", "ur:envelope/tpcslsadaoaxgedmotks"],
        "83010203"
    )
}

#[test]
fn test_extract_cbor_2() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "cbor", "ur:envelope/lptpsotanehnkkadotdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpfpfpfpgrfygadlkkimfxkkgdetjlbkktiofpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpbkfpfpfpfefpemdldlecgrhkkokoenfgjliminktjsdngrfeisgaksgmjnfpiejeksjeeciogthdgseejkjojsknfwiogagtehkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflbkgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiofpfpfpfpfdfdiekoidflhtfphfeyesjkhtjtgtjyghhgfgimgyjneskohskkehgyiajnetkpidflesimhkhgktfwbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkoytpsoioiajljnjnihjtjytpsokscektjljziyfzhgjljziyjkdpgthsiafwjljljedpgdjpjldmjzjliahsjzoytpsoiojeihkkguinknihtpsocfadaeoytpsojeiyinjtioihjpjojpinjtjytpsokseygufdfpeyecenftktjljtgefdhgidjnhfhkhshtkkjpkkemenhfgwdlgygtecdygdgmjsfwgridfgfwehkkeojlfwfpflgmjykphkoytpsoiejykkjoihtpsoiofefyeyececehesasgsayda"],
        "d99f607901a32d2d2d2d2d424547494e204f50454e5353482050524956415445204b45592d2d2d2d2d0a6233426c626e4e7a614331725a586b74646a45414141414142473576626d554141414145626d39755a5141414141414141414142414141414d7741414141747a633267745a570a51794e5455784f514141414342626a456c546f70706741306534677079307538447552696b38386d3167675a4756707a746e6543714159414141414b44492f796a437950386f0a776741414141747a633267745a5751794e5455784f514141414342626a456c546f70706741306534677079307538447552696b38386d3167675a4756707a746e6543714159410a4141414541372f2f354b59767636466f6a6977712b4b45684978526d41646b786b35674d584c347370717a4267494d31754d53564f696d6d4144523769436e4c5337774f35470a4b547a79625743426b5a576e4f3264344b6f4267414141414848647662475a41563239735a6e4d745457466a516d397661793151636d38756247396a595777420a2d2d2d2d2d454e44204f50454e5353482050524956415445204b45592d2d2d2d2d0a"
    )
}

#[test]
fn test_extract_data() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "data", "ur:envelope/tpcsfxadaoaxfniagtkb"],
        "010203"
    )
}

#[test]
fn test_extract_date() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "date", "ur:envelope/tpcssecyiabtrhfrpafdbzdy"],
        "2022-08-30T07:16:11Z"
    )?;
    run_cli_expect(
        &["extract", "date", "ur:envelope/tpcssecyiabtguaeoxtdvdjp"],
        "2022-08-30T00:00:00Z"
    )
}

#[test]
fn test_extract_digest() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "digest", "ur:envelope/tpcstansfphdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiasppdmsgyas"],
        "ur:digest/hdcxvlfgdmamwlsshgiaemcsnelkylfwjefdsktadpfwolgmlrlevduyontbbbpyiaspvadsadje"
    )
}

#[test]
fn test_extract_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "envelope", ALICE_KNOWS_BOB],
        "ur:envelope/tpsoihfpjziniaihmebdmodl"
    )
}

#[test]
fn test_extract_known() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "known", "ur:envelope/adonahurcw"],
        "'isA'"
    )?;
    run_cli_expect(
        &["extract", "known", "ur:envelope/cfdyfyfwfpwzms"],
        "'12356'"
    )
}

#[test]
fn test_extract_number() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "number", "ur:envelope/tpcszofzasckrogywmlpctfggoreee"],
        "3.14"
    )?;
    run_cli_expect(
        &["extract", "number", "ur:envelope/tpcscsdrldehwedp"],
        "42"
    )
}

#[test]
fn test_extract_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "string", ALICE_KNOWS_BOB],
        "Alice"
    )
}

#[test]
fn test_extract_ur() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_extract_uri() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "uri", "ur:envelope/tpcstpcxjkisjyjyjojkftdldlihkshsjnjojzihdmiajljncnnswmse"],
        "https://example.com"
    )
}

#[test]
fn test_extract_uuid() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "uuid", "ur:envelope/tpcstpdagdgadrsbwkbwuofdjplefrgrynhhjeurkenstefppt"],
        "492acbf4-13dc-4872-8a3b-4bf65c6bdf7c"
    )
}

#[test]
fn test_extract_wrapped() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "wrapped", "ur:envelope/tpsptpcslsadaoaxqzsshsyl"],
        "ur:envelope/tpsolsadaoaxzerkykme"
    )
}

#[test]
fn test_extract_assertion_subject() -> anyhow::Result<()> {
    let e = Envelope::new_assertion(known_values::NOTE, "This is a note.");
    let ur = e.ur_string();

    let predicate_envelope = "ur:envelope/aatljldnmw";
    let object_envelope = "ur:envelope/tpsojlghisinjkcxinjkcxhscxjtjljyihdmkkqdzops";
    let pred_obj_envelope = [predicate_envelope, object_envelope].join("\n");

    run_cli_expect(&["extract", "assertion", &ur], &pred_obj_envelope)?;
    run_cli_expect(&["extract", "predicate", &ur], predicate_envelope)?;
    run_cli_expect(&["extract", "object", &ur], object_envelope)
}
