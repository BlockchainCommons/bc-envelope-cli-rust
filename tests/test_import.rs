use indoc::indoc;

mod common;
use common::*;

#[rustfmt::skip]
const ED25519_PRIVATE_KEY_SSH_ENCRYPTED: &str = indoc! {"
    -----BEGIN OPENSSH PRIVATE KEY-----
    b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABAAFiGpGp
    tFlJLG9vpkh+AcAAAAGAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iC
    nLS7wO5GKTzybWCBkZWnO2d4KoBgAAAAoOtDEwxXcRHJWAxcYY5iJVdBCl5UGfLYYPK+Gb
    ybsn7Oz1WlEL4RVorR854HqXRwch5BQ5d3KXYm5vEj5kiu4cHLOHqkFoSRrwY7F7yOwgYr
    fNPS6xZvrhxx2spEtB95QROjGbgjEa1tNI4vXYArmK70tlpaEgsFMLfuXVZmlUZZS2M2eh
    2L7leSuWLZDPVlVSsNqEXD/bVVGHGw3c1Tf8Y=
    -----END OPENSSH PRIVATE KEY-----
"};

#[rustfmt::skip]
const ED25519_PRIVATE_KEY_SSH: &str = indoc! {"
    -----BEGIN OPENSSH PRIVATE KEY-----
    b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
    QyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYAAAAKAv5jjOL+Y4
    zgAAAAtzc2gtZWQyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYA
    AAAEA7//5KYvv6Fojiwq+KEhIxRmAdkxk5gMXL4spqzBgIM1uMSVOimmADR7iCnLS7wO5G
    KTzybWCBkZWnO2d4KoBgAAAAHHdvbGZAV29sZnMtTWFjQm9vay1Qcm8ubG9jYWwB
    -----END OPENSSH PRIVATE KEY-----
"};

const ED25519_PRIVATE_KEY_UR: &str =
    "ur:signing-private-key/tanehnkkadotdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpfpfpfpgrfpkoecimimgwgsdnhkeebkkniofpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpbkfpfpfpfefpemdldlecgrhkkokoenfgjliminktjsdngrfeisgaksgmjnfpiejeksjeeciogthdgseejkjojsknfwiogagtehkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflbkgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiofpfpfpfpfdfdiekoidflhtfphfeyesjkhtjtgtjyghhgfgimgyjneskohskkehgyiajnetkpidflesimhkhgktfwbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkgtctcyrd";

const ED25519_PUBLIC_KEY_SSH: &str =
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iCnLS7wO5GKTzybWCBkZWnO2d4KoBg wolf@Wolfs-MacBook-Pro.local";

const ED25519_PUBLIC_KEY_UR: &str =
    "ur:signing-public-key/tanehsksjnjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafgkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiocxktjljziyfzhgjljziyjkdpgthsiafwjljljedpgdjpjldmjzjliahsjzmybngyfs";

#[rustfmt::skip]
const ED25519_SIGNATURE_SSH: &str = indoc! {"
    -----BEGIN SSH SIGNATURE-----
    U1NIU0lHAAAAAQAAADMAAAALc3NoLWVkMjU1MTkAAAAgW4xJU6KaYANHuIKctLvA7kYpPP
    JtYIGRlac7Z3gqgGAAAAAEZmlsZQAAAAAAAAAGc2hhNTEyAAAAUwAAAAtzc2gtZWQyNTUx
    OQAAAEA6Nx4GYTzs3JL/E//ruJf2AobDfsngRhZ8QT/BaZnLofdrRAUIMPb4K9toCCITjG
    SgCuQsV0/hUUrcj7vBCgYK
    -----END SSH SIGNATURE-----
"};

const ED25519_SIGNATURE_UR: &str =
    "ur:signature/taneidkkaddsdpdpdpdpdpfwfeflgaglcxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkgoehglgagodyjzfdfpfpfpfpfpgyfpfpfpfygtfpfpfpfpgsiaeogljlgshghfjegtimgoehgtghjefpfpfpfpiohgeeksgegoengrhshkfpglfdkpgagriajygskofpemjehkjogdgdbkgejyhkgaflgmjzhsiaemhteoiojsioflfpfpfpfpfpfehtjnjzjkhtgyfpfpfpfpfpfpfpfpfpfliaeyisisglghfekkfpfpfpfpgoktfpfpfpfpjykniaeyiojyhthggykkglghgoksbkgwgyfpfpfpfefpenglkseeflhkghknjkeogegsdlfedldljpkpgeiyeyfpjlidfyiyjkjtiogmishtetgyghdlfwhshtjtgsjliyiejpgmfpgogagtgdideegresjyjlfxfxgaghimflbkguiofxkpgyjkhfdydlisgogojpiaimemkofwfxiohkgrbkdpdpdpdpdpfeglfycxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkcllebeje";

#[test]
fn test_import_export_signing_private_key() {
    let imported = run_cli_stdin(
        &["import", "--password", "test"],
        ED25519_PRIVATE_KEY_SSH_ENCRYPTED
    ).unwrap();
    assert_eq!(imported, ED25519_PRIVATE_KEY_UR);
    let exported = run_cli(&["export", ED25519_PRIVATE_KEY_UR]).unwrap();
    assert_eq!(exported, ED25519_PRIVATE_KEY_SSH.trim());
    let re_imported = run_cli(&["import", "--", ED25519_PRIVATE_KEY_SSH]).unwrap();
    assert_eq!(re_imported, ED25519_PRIVATE_KEY_UR);
}

#[test]
fn test_import_export_signing_public_key() {
    let imported = run_cli(&["import", ED25519_PUBLIC_KEY_SSH]).unwrap();
    assert_eq!(imported, ED25519_PUBLIC_KEY_UR);
    let exported = run_cli(&["export", ED25519_PUBLIC_KEY_UR]).unwrap();
    assert_eq!(exported, ED25519_PUBLIC_KEY_SSH);
}

#[test]
fn test_import_export_signature() {
    let imported = run_cli(&["import", "--", ED25519_SIGNATURE_SSH]).unwrap();
    assert_eq!(imported, ED25519_SIGNATURE_UR);
    let exported = run_cli(&["export", ED25519_SIGNATURE_UR]).unwrap();
    assert_eq!(exported, ED25519_SIGNATURE_SSH.trim());
}
