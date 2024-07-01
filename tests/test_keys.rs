use bc_envelope::prelude::*;

mod common;
use common::*;

const SEED: &str = "ur:seed/oyadhdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdweocfztrd";
const PRVKEYS: &str = "ur:crypto-prvkeys/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";

#[test]
fn test_generate_random_private_key_base() {
    let prvkeys = run_cli(&["generate", "prvkeys"]).unwrap();
    assert_eq!(UR::from_ur_string(prvkeys).unwrap().ur_type_str(), "crypto-prvkeys");
}

#[test]
fn test_generate_private_key_base_from_seed() {
    let prvkeys = run_cli(&["generate", "prvkeys", "--seed", SEED]).unwrap();
    assert_eq!(prvkeys, PRVKEYS);
}

#[test]
fn test_schnorr() {
    let pubkeys = run_cli(&["generate", "pubkeys", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshfhdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejobebtdlhd";
    assert_eq!(pubkeys, expected_pubkeys);
}

#[test]
fn test_ecdsa() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ecdsa", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshflfadhdclaoayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejomecapkpr";
    assert_eq!(pubkeys, expected_pubkeys);
}

#[test]
fn test_ssh_ed25519() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-ed25519", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskshdjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafwemeydlgseseyetendndlhkgdkpeojneseckogridjyjshsflgmjnidjzgdhgiygtgdhsgwhthkemdldyjyiycxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejogoimkkkt";
    assert_eq!(pubkeys, expected_pubkeys);
}

#[test]
fn test_ssh_ecdsa_nistp256() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-ecdsa-p256", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskspdihiaiejkhsdpjkishseydpjtinjkjyjoeyecencxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkkglghhkfpfpfpfpgaidjnjzkniefdfpkkglghhkfpfpfpfwfwfwflengakpflgygoiajteyeyehgdhkimgwgeihgmgmingeiyiyhtjzjnksfxjkemfwenkphsgsksjtgyeyioeteokohsidjykpehgdihgsiyguenjsjtgojofgjlfggadyjedlgrioetgahdemjyihjpioidjtidkpgajkglgafektfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejodlztswca";
    assert_eq!(pubkeys, expected_pubkeys);
}

#[test]
fn test_ssh_ecdsa_nistp384() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-ecdsa-p384", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskstyihiaiejkhsdpjkishseydpjtinjkjyjoeoeteecxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkngwfygyfpfpfpfpgaidjnjzkniefdfpkngwfygyfpfpfpfwisfwfeeedyfgdyksjogrgmgoghhkieidindlgrisfggdjofwihehgwidjohsgsgleojlgwjlflhdhfjehkkoksecgmididkkengdjkiaflgehtemflgtdyingsguihinjojljletkkhdgyemfefwgogmksinflgaingokojliyhgkojyetjzeefggmehfleyksgaiogadyimiefxiseteyfggejkiegoehkkfgimethtiajkgtindlisjnecghdnjthsiofsfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejoltvdgula";
    assert_eq!(pubkeys, expected_pubkeys);
}

#[test]
fn test_ssh_ecdsa_nistp521() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-ecdsa-p521", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskkadaaihiaiejkhsdpjkishseydpjtinjkjyjoeceyehcxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpehgtimfefpfpfpfpgaidjnjzkniefdfpehgtimfefpfpfpfxfgfwfpfegsgmgrktisgdjpflfwfxdnjkemimhgfeisknktgweodlethkgtjeiagwiagygafgidiefphtehgaisguksjogeiehkfygtguhtjpgdidjkgdieenjsjlkpghimksgdgsiaemjehgknjnimjsglkteyiykoglfpjodydyetkpidjygyfpfgkoiygljkjsgafeiejteydninjzfdeohdjtfgglenhkksgajlihgudyidgajsghecfxfgdniykkgeehgrieiegmkpjyiskkehjtgrkkgyknimgddnjpfyeyeeihfgemghgmgogwhfghkpinhgjpeoiykpdyiydyktimgaehjygygyfsfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejofrfpkery";
    assert_eq!(pubkeys, expected_pubkeys);
}

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_dsa() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-dsa", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskkaoghjkjkisdpiejkjkcxfpfpfpfpfweoglknhsfxehjeiaeogtfpfpfpfxfwfpgdfwgretjtjphtjehdiyieksjykteygaingmhdhkiaimidemfyfghsfwjkiejzknhkjohgksghgmeyesjekkgygyfljyeyhkeneeinkseydndnfxjnfxjleeesgeesgygujofdkthgiohtgofgechffwhfgoghfyghflflksgdemksksfeguecihgsgudlhkiyiykpgmemethkhdiohtfeidfpguecjsidglidhdhkfdhkhtfehfhgiygwfdiyghjsgmfyfxgmgmjpjegageesgegoinjtflghfwglgedlgaknfydnjpihehendlfyieglgskkjnhfhtkteokpfxjyfpfpfpfpfggyfygofletgukkfeihdnfphgjtisgtjoiodlkkjykoglkpecgsktknetktfpfpfpgafwgmkogmiaehglhghthkjlgrfxfgetiegdhkgogrfghdhgfwhsjsinisgshfhsfdhsktiyisktfdjehdfpetjsisgaeskojsemgujoemhsdlkphsjnjziegmgskojlkofdfwkoecjlecgrgyfdjpguhgemiojlglfwidkkgafeenimjzfyisgweoiehfhgktfxinidjygufyfwiaksgegljyhggtgaetkoeejsiahkfgdnkpeekkjefgingmiajnfwjektgyjoihemgukngljlglfdhffxeyflfwgreoglfgjnimkniegeisfwhdgmenfyfdhkkseeecfpghieksihiofpfpfpgafefpjlidglfldlehidfpeekogseohdechtfdfyhthsgwfwetgmemjphdjohgihihgednishgfwkthsimfpfddlhgimiheoinjzehgljzjlgyihfeihgsiojpgwimjohfjnfgdldlidiagmisieiykkkofdghiygyjlgreyiokteseefgehgwiakphtghemktflkkgyeojzgdgahsjzgeehjejokngretjsdnimfygddngliagrhsgsioghinehimjsiagridglgukpfwfwjlineteyehfphdghimidhgeohkgdfghfhsenhfhdgwjtgtgmjnfwinjyjzjzfygrjoeyfyetfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejomsaadihh";
    assert_eq!(pubkeys, expected_pubkeys);
}

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_rsa_sha256() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-rsa-sha256", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyjzktinkkdyhfishgdyhgfwfxhseejkfwidksjsfyidgojyenjsiheeenjsfgjogujnececjykkglgsjsideseyhkhsjkesfgiyjejnfekpgsiakofpgseheoeyeehdiahfdygefdgeioehfpidfxiaenjtiegejkeegahshtgrjyiehgjshgksgsdnemehineydyjkktinhtdlfgjngakoktgafefyfdgwgwgwhtjpidendygueyimhgfddngmhkgoesjlgedydnkseohkfygskkhkinhgjyesfxkogwiajogtisihkphdetehgtgtdyflhkjpfdjzjyiogoisisesktjtengrfgjyjeknjshsjnjlgaeninglfgjsgthfdyecglgegegreeimfygygahdjkiejldyjlfyetfygrimgagdihkohgjtguiofdisiofyjtfgidgmeednfxkphkendygtjoidesgogseoineoktflfykniyjnflingejtjeeokkioknidgoetjzesehengojeetgoihkoidfyhtecgojojokkeshdhdjnehiofefwdnfygyiejzkshthtgeknjeiyisfygufpdlksenghgdjojygteyioesgsetfdgugmeogejneeetjkgtecdnenjkjoehhdgsfwjeideyfdcxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejotpgdosen";
    assert_eq!(pubkeys, expected_pubkeys);
}

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_rsa_sha512() {
    let pubkeys = run_cli(&["generate", "pubkeys", "--type", "ssh-rsa-sha512", "--comment", "comment", PRVKEYS]).unwrap();
    let expected_pubkeys = "ur:crypto-pubkeys/lftanshftanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyfdgukteygrfliegtfwiyhshfglgoimhfiodliejogsjlgaeyfyeogljnfdingrdlgsingwfwhdkogwjthkfydnksgsgdgakokkhtgwghkpeskshtfldlgsktknfpdyjkgrghghjsgsjnfyemimksgwhkfegufgfgenhkemeyetfwetjnjsenfgglflkkgekkgshgimfeemkneoieiyhkgwkkgtehgrghktgrfxfwiminhfemhtfwgmiyflfphfgljodydyglgogmeciaglhteofykndlgdknjnfwihdyjejpflgtgdeokpdykkgaiyjnfygrhtgufedliyeygrecgdkofygrdyjsfyknguhffdflgugrdyhgkpgokniegdjzidfdhsehdneyecidksfgdlhshsehhsidiyfyemgwiyhsiefegrgdhgihguiekokkhkfeeoeyfeknfygodlimgofdghjnemiajliakkemgmeeghgtesdnfefeiyeekkjzfgidinhkjphsjtjtfpgwfljykpfefgesgyfdenkojokkdyhtjkgagtesksiyjpjeioeejefpioiodnidjkimetgsdlgdeefwjsetguecgdececdlkojsgtisioiehkhtfxiajyjegoksgyeygljkgwkpihhsfyjkiohtihgahfcxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejovyotplpd";
    assert_eq!(pubkeys, expected_pubkeys);
}
