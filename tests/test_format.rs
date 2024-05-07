use indoc::indoc;
use anyhow::Result;

mod common;
use common::*;

const ENVELOPE: &str = "ur:envelope/lftpsplntpcstansgshdcxchfdfwwdsrzofytsyndsvetsndkbbelbtdmuskhfdtyntbcprocktyatktaxaosphdcxfnstoxfwdaglhlmywzpafwmnfdzezmkisgfhtaetihtibemedpnsuevswtcngwpaoybtlstpcstansgshdcxaaenfsheytmseorfbsbzktrdrdfybkwntkeegetaveghzstattdertbswsihahvsoyaatpcsksckgajkjkkpihiecxidkkcxjyisihcxgujyhsjyihcxjliycxfekshsjnjojzihoyastpcstpcxksheisjyjyjojkftdldlihkshsjnjojzihjzihieioihjpdmiajljndlhsjpiniedldyeeeoeneoieeciyiyesesemeoeoidiadyiyehecememidhsidhseeeedyhsiyehiaiyeoeeeehsieesiheeeceeiyhsieesieeheyetiadydyiyihiyenecdyecihetoybalktpcstansgshdcxksrfdyaeflkootmhhpsfrhronbeytkjpdwwdwtrkzocygawdwfcshepyhdaysguohdcxbwjsinwkcmahnefdmsfdgtltkpdppdbdwnfdhhwfjyptvddimucwrycavameetssoytpcsimiyhsjninjzkkglhsjnihtpcsihgugtgaghfdoytpcsinioinkoihjtglhsjnihtpcsiegegwfdglhdcxhdcamnzeftfppdwzpmjojluypebnbeplzeptzesfkgfholssdtkgveimonnlsosehdcxjscnletijkdssosnvljpbklrhpihrpjtfwtnwecflolsolfmkbnlndosmdadztsboytpcsihinjnhsioihlstpcsjpgejlisjtcxgujninjyiscxjkjninjzinjtiooyastpcskshsisjyjyjojkftdldlihkshsjnjojzihjzihieioihjpdmiajljndlieinioihjkjydleoenidiheodyemeyenidihiyideneciahseheoideheoenhsiheyesieetdyetehiyeneeemeseyiaeyemdyeyeeehecihidendyhsieehiaecenihieeoeoiaesesesoyaatpcsksctghisinjkcxinjkcxhsjtcxinjnhsioihcxjliycxgejlisjtcxgujninjyisdmhdcxnsmkylvtfseegsgotaammhcezebdgwhyhhyljkrhwfqzoskgeosodsgmpmhgzchhhdcxosfmfplpfxvefzoybncfwzgtfewdcapsqdkkuolagdtdltvwfdttvorflocwzegahdcxtbcffdrptpstzmmomdktssmegedwvdecgadtdsreaygtdifwmokimwaodwbyuozmhdcxvyidloaagdetmopfrnbwleidmeioftfptavtlnptprvohnfpmtcegdseamceotwyhdcxzejocerptnaxchswvossceasnehkgefyptmhndretdghwtwepymwoyrocmnntddioyadtpcsimiajpihieihjtjyinhsjzhdcxwmesosbwlupscfiopltaemchmdzmtllrgraxlnrhwnkbfmlrveadrtlobspspmmsoyaxlftpcstansghhdfzqdwtmnhlgegylkasmhvtguaadtbstohstekbolkpastlrecltasgadcwtljtnlrhvlecrplufyvacfkevacpesbkdesfpfkpoyosylwzlbvosfyldtdejnbtioprdmoxoyaatpcskscagthsieihcxidkkcxjyisihcxgujyhsjyihcxjliycxfekshsjnjojzihdmjzveesyk";

#[test]
fn test_format() -> Result<()> {
    let expected_output = r#""Hello.""#;
    run_cli_expect(&["format", HELLO_ENVELOPE_UR], expected_output)?;
    run_cli_expect_stdin(&["format"], expected_output, HELLO_ENVELOPE_UR)
}

#[test]
fn test_format_envelope() -> Result<()> {
    let output = run_cli_raw(&["format", ENVELOPE])?;
    assert_eq!(
        output,
        indoc! {r#"
        {
            ARID(174842ea) [
                'isA': "credential"
                'holder': ARID(78bc3000) [
                    "familyName": "SMITH"
                    "givenName": "JOHN"
                    "image": "John Smith smiling" [
                        'dereferenceVia': "https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999"
                        'note': "This is an image of John Smith."
                    ]
                    ELIDED (8)
                ]
                'issuer': ARID(04363d5f) [
                    'dereferenceVia': URI(https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8)
                    'note': "Issued by the State of Example"
                ]
                ELIDED (2)
            ]
        } [
            'verifiedBy': Signature [
                'note': "Made by the State of Example."
            ]
        ]
        "#}
    );
    Ok(())
}

#[test]
fn test_format_cbor() -> Result<()> {
    run_cli_expect(
        &["format", "--type", "cbor", ENVELOPE],
        "d8c882d8c886d8c9d99c4c5820174842eac3fb44d7f626e4d79b7e107fd293c55629f6d622b81ed407770302c858203cc7a442254e5d8ff2b1428e48feff7dca3fd93865d010912d9cdee8f0234fb1a10d83d8c9d99c4c582004363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8a104d8c9781e49737375656420627920746865205374617465206f66204578616d706c65a109d8c9d820785f68747470733a2f2f6578616d706c656c65646765722e636f6d2f617269642f30343336336435666639393733336263306631353737626162613434306166316366333434616439653435346661643964313238633030666566363530356538a10e8cd8c9d99c4c582078bc30004776a3905bccb9b8a032cf722ceaf0bbfb1a49eaf3185fab5808cadc5820137169f416059f4897484d87752da80bf1485cf374a9e727931bbd1de69138c4a1d8c96a66616d696c794e616d65d8c965534d495448a1d8c969676976656e4e616d65d8c9644a4f484e5820581d8efe3a41a8f2ad706fdbaf0c10aefea9fecc7b3fa6c4297be46aa599c9c1582071238ad07326c9cde3720a845b65b66e42daed198883a63e7e999ba79501fccba1d8c965696d61676583d8c9724a6f686e20536d69746820736d696c696e67a109d8c9786168747470733a2f2f6578616d706c656c65646765722e636f6d2f6469676573742f33366265333037323662656662363563613133623133366165323964383038316636343739326332373032343135656236306164316335366564333363393939a104d8c9781f5468697320697320616e20696d616765206f66204a6f686e20536d6974682e58209c98f7e03d344c55d906901cfe0b4f5e5cf773b9f3b4a77b33c92652ad57fd5c5820a73e418543e440a10c19f24d45ea1dacb379dc8050d287e548d1e2bc881bfe495820d61948b6d8c7ff929577c4914a2ce735492926b5084d2742927d94022c11dcff5820e1628804503892b0be138a6291673a41d9e086a9b2e26041961c50c1061ca3ee5820fe701cb6da0317c6e2c41c099f594a44a9909bb5d254f0edab94a1b8169ed227a101d8c96a63726564656e7469616c5820eb39a7138bac1967aed9371795ffd5844b0386b9f17e3e84e401c0880facad97a10382d8c9d99c545840b3f08e5d4a518c0990e05304290fce61d37ea67509d5b521d9ca011bd56e99b9e335b68b44e6197ce622390a28ccb075a1a7f7f27fe2ccf729286d0d67b22ea4a104d8c9781d4d61646520627920746865205374617465206f66204578616d706c652e"
    )
}

#[test]
fn test_format_diag() -> Result<()> {
    let output: String = run_cli_raw(&["format", "--type", "diag", ENVELOPE])?;
    assert_eq!(
        output,
        indoc! {r#"
        200(   / envelope /
           [
              200(   / envelope /
                 [
                    201(   / leaf /
                       40012(   / arid /
                          h'174842eac3fb44d7f626e4d79b7e107fd293c55629f6d622b81ed407770302c8'
                       )
                    ),
                    h'3cc7a442254e5d8ff2b1428e48feff7dca3fd93865d010912d9cdee8f0234fb1',
                    {
                       13:
                       [
                          201(   / leaf /
                             40012(   / arid /
                                h'04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8'
                             )
                          ),
                          {
                             4:
                             201(   / leaf /
                                "Issued by the State of Example"
                             )
                          },
                          {
                             9:
                             201(   / leaf /
                                32(
                                   "https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8"
                                )
                             )
                          }
                       ]
                    },
                    {
                       14:
                       [
                          201(   / leaf /
                             40012(   / arid /
                                h'78bc30004776a3905bccb9b8a032cf722ceaf0bbfb1a49eaf3185fab5808cadc'
                             )
                          ),
                          h'137169f416059f4897484d87752da80bf1485cf374a9e727931bbd1de69138c4',
                          {
                             201("familyName"):   / leaf /
                             201("SMITH")   / leaf /
                          },
                          {
                             201("givenName"):   / leaf /
                             201("JOHN")   / leaf /
                          },
                          h'581d8efe3a41a8f2ad706fdbaf0c10aefea9fecc7b3fa6c4297be46aa599c9c1',
                          h'71238ad07326c9cde3720a845b65b66e42daed198883a63e7e999ba79501fccb',
                          {
                             201("image"):   / leaf /
                             [
                                201("John Smith smiling"),   / leaf /
                                {
                                   9:
                                   201(   / leaf /
                                      "https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999"
                                   )
                                },
                                {
                                   4:
                                   201(   / leaf /
                                      "This is an image of John Smith."
                                   )
                                }
                             ]
                          },
                          h'9c98f7e03d344c55d906901cfe0b4f5e5cf773b9f3b4a77b33c92652ad57fd5c',
                          h'a73e418543e440a10c19f24d45ea1dacb379dc8050d287e548d1e2bc881bfe49',
                          h'd61948b6d8c7ff929577c4914a2ce735492926b5084d2742927d94022c11dcff',
                          h'e1628804503892b0be138a6291673a41d9e086a9b2e26041961c50c1061ca3ee',
                          h'fe701cb6da0317c6e2c41c099f594a44a9909bb5d254f0edab94a1b8169ed227'
                       ]
                    },
                    {
                       1:
                       201("credential")   / leaf /
                    },
                    h'eb39a7138bac1967aed9371795ffd5844b0386b9f17e3e84e401c0880facad97'
                 ]
              ),
              {
                 3:
                 [
                    201(   / leaf /
                       40020(   / signature /
                          h'b3f08e5d4a518c0990e05304290fce61d37ea67509d5b521d9ca011bd56e99b9e335b68b44e6197ce622390a28ccb075a1a7f7f27fe2ccf729286d0d67b22ea4'
                       )
                    ),
                    {
                       4:
                       201(   / leaf /
                          "Made by the State of Example."
                       )
                    }
                 ]
              }
           ]
        )
        "#}
    );
    Ok(())
}

#[test]
fn test_format_tree() -> Result<()> {
    let output: String = run_cli_raw(&["format", "--type", "tree", ENVELOPE])?;
    assert_eq!(
        output,
        indoc! {r#"
        7da760aa NODE
            2f50e5e7 subj WRAPPED
                ee1bfc78 subj NODE
                    6c1c5596 subj ARID(174842ea)
                    3cc7a442 ELIDED
                    728e7274 ASSERTION
                        6dd16ba3 pred 'issuer'
                        33257537 obj NODE
                            cf8241fe subj ARID(04363d5f)
                            4be120e3 ASSERTION
                                0fcd6a39 pred 'note'
                                c6e07baa obj "Issued by the State of Example"
                            f451ae8e ASSERTION
                                cdb6a696 pred 'dereferenceVia'
                                d5cb18e7 obj URI(https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8)
                    b02071bd ASSERTION
                        9a7ea0eb pred 'holder'
                        95ce7a1a obj NODE
                            db53cadb subj ARID(78bc3000)
                            137169f4 ELIDED
                            1e1b5a40 ASSERTION
                                a4760522 pred "familyName"
                                e9a5913e obj "SMITH"
                            460df727 ASSERTION
                                b771d812 pred "givenName"
                                f3e7ec3d obj "JOHN"
                            581d8efe ELIDED
                            71238ad0 ELIDED
                            746ca150 ASSERTION
                                763303e5 pred "image"
                                8ed5acce obj NODE
                                    28252e90 subj "John Smith smiling"
                                    2822e493 ASSERTION
                                        cdb6a696 pred 'dereferenceVia'
                                        21b4b63e obj "https://exampleledger.com/digest/36be307…"
                                    ef16f1af ASSERTION
                                        0fcd6a39 pred 'note'
                                        6ad445db obj "This is an image of John Smith."
                            9c98f7e0 ELIDED
                            a73e4185 ELIDED
                            d61948b6 ELIDED
                            e1628804 ELIDED
                            fe701cb6 ELIDED
                    be100e9e ASSERTION
                        2be2d79b pred 'isA'
                        c2e5cb01 obj "credential"
                    eb39a713 ELIDED
            34ceffc8 ASSERTION
                d0e39e78 pred 'verifiedBy'
                a89b685b obj NODE
                    bf52495c subj Signature
                    f763da80 ASSERTION
                        0fcd6a39 pred 'note'
                        ae039855 obj "Made by the State of Example."
        "#}
    );
    Ok(())
}
