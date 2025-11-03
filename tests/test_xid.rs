use indoc::indoc;
mod common;
use common::*;

const XID_DOC: &str = "ur:xid/tpsplftpsotanshdhdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnoyaylftpsotansgylftanshfhdcxhslkfzemaylrwttynsdlghrydpmdfzvdglndloimaahykorefddtsguogmvlahqztansgrhdcxetlewzvlwyfdtobeytidosbamkswaomwwfyabakssakggegychesmerkcatekpcxoycsfncsfggmplgshd";

#[test]
fn test_xid_format() {
    // Anywhere in `envelope` that accepts a `ur:envelope` can also accept any
    // other UR type, including XID documents.

    // $ envelope format $XID_DOC

    #[rustfmt::skip]
    run_cli_expect(&["format", XID_DOC], indoc! {r#"
        XID(71274df1) [
            'key': PublicKeys(eb9b1cae, SigningPublicKey(71274df1, SchnorrPublicKey(9022010e)), EncapsulationPublicKey(b4f7059a, X25519PublicKey(b4f7059a))) [
                'allow': 'All'
            ]
        ]
    "#}.trim()).unwrap();

    // Note that this does not validate the XID document (or any other
    // envelope-containing UR), it just reads the UR’s envelope, meaning you can
    // manipulate it like any other envelope.

    // $ envelope assertion at 0 $XID_DOC | \
    // envelope format

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["assertion", "at", "0", XID_DOC],
            &["format"]
        ],
        indoc! {r#"
            'key': PublicKeys(eb9b1cae, SigningPublicKey(71274df1, SchnorrPublicKey(9022010e)), EncapsulationPublicKey(b4f7059a, X25519PublicKey(b4f7059a))) [
                'allow': 'All'
            ]
        "#}.trim()
    ).unwrap();

    // $ envelope assertion at 0 $XID_DOC | \
    // envelope extract object | \
    // envelope assertion at 0 | \
    // envelope format

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["assertion", "at", "0", XID_DOC],
            &["extract", "object"],
            &["assertion", "at", "0"],
            &["format"]
        ],
        indoc! {r#"
            'allow': 'All'
        "#}.trim()
    ).unwrap();

    // XID Documents always have the XID CBOR object as their subject. So you
    // can extract the baer XID of a XID document using the `extract xid`
    // subcommand.

    let bare_xid = run_cli(&["extract", "xid", XID_DOC]).unwrap();
    assert_eq!(
        bare_xid,
        "ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw"
    );

    // Bare XID URs, although they do not contain an envelope (they are just
    // CBOR) are also internally imported into an empty XID document and then
    // turned into an envelope, with just the XID as its subject.

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &bare_xid],
        indoc! {r#"
            XID(71274df1)
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_id() {
    // Unlike the technique of simply extracting the subject above, this
    // subcommand validates the entire XID document.

    let xid_id = run_cli(&["xid", "id", XID_DOC]).unwrap();
    assert_eq!(
        xid_id,
        "ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw"
    );

    // Extracting the bare XID from a bare XID UR is idempotent.

    run_cli_expect(&["xid", "id", &xid_id], &xid_id).unwrap();

    // Several output formats are supported. `ur` is the default and is
    // machine-readable, while the others are human-readable.

    #[rustfmt::skip]
    run_cli_expect(
        &["xid", "id",
            "--format", "ur",
            "--format", "hex",
            "--format", "bytewords",
            "--format", "bytemoji",
            &xid_id],
        indoc! {r#"
            ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
            XID(71274df1)
            🅧 JUGS DELI GIFT WHEN
            🅧 🌊 😹 🌽 🐞
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_new() {
    // The `xid new` subcommand converts a `PrivateKeyBase` or `PublicKeys`
    // into a XID Document with the provided key as the inception key.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PUBKEYS],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                ]
            ]
        "#}.trim()
    ).unwrap();

    // A XID document returned by the `xid new` subcommand is returned as a
    // `ur:xid`.

    run_cli_expect(
        &["xid", "new", ALICE_PUBKEYS],
        "ur:xid/tpsplftpsotanshdhdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfessoyaylftpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgzckbfwes"
    ).unwrap();

    // If a `PrivateKeyBase` is provided, by default the salted private key
    // itself will be included.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PRVKEY_BASE],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    {
                        'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
                    } [
                        'salt': Salt
                    ]
                    'allow': 'All'
                ]
            ]
        "#}.trim()
    ).unwrap();

    // The private key can be omitted using the `--private omit` option, or
    // elided using `--private elide`.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PRVKEY_BASE, "--private", "omit"],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                ]
            ]
        "#}.trim()
    ).unwrap();

    // $ envelope xid new $ALICE_PRVKEY_BASE --private elide | envelope format

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PRVKEY_BASE, "--private", "elide"],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    ELIDED
                ]
            ]
        "#}.trim()
    ).unwrap();

    // One or more endpoint URIs may be added to the inception key.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PUBKEYS,
                "--endpoint", "https://endpoint.example.com/",
                "--endpoint", "btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6"],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'endpoint': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
                    'endpoint': URI(https://endpoint.example.com/)
                ]
            ]
        "#}.trim()
    ).unwrap();

    // One or more permissions may be specified for the inception key. These
    // replace the default `'All'` permission.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PUBKEYS,
                "--allow", "encrypt",
                "--allow", "sign"],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
            ]
        "#}.trim()
    ).unwrap();

    // The key may be given a user-assigned name ("nickname") using the
    // `--nickname` option.

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "new", ALICE_PUBKEYS,
                "--nickname", "Alice's Key"],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice's Key"
                ]
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_key_add() {
    // All the same options as `xid new` are available. The same key may not be
    // added twice.

    // $ XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    // $ envelope xid key add --nickname 'Bob' $BOB_PUBKEYS $XID_DOC | envelope
    // format

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "key", "add", "--nickname", "Bob", BOB_PUBKEYS, &xid_doc],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                    'allow': 'All'
                    'nickname': "Bob"
                ]
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_key_update() {
    // All the same options as `xid new` are available. The key must already
    // exist in the XID document.

    // $ XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS | envelope
    // xid key add --nickname 'Bob' $BOB_PUBKEYS` $ envelope format $XID_DOC

    // XID(93a4d4e7) [
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    //     'key': PublicKeys(e2c18423,
    // SigningPublicKey(SchnorrPublicKey(f0638394)),
    // EncapsulationPublicKey(X25519PublicKey(4af6be52))) [         'allow':
    // 'All'         'nickname': "Bob"
    //     ]
    // ]

    // All the same options as `xid new` are available. The key must already
    // exist in the XID document.

    let xid_doc = run_cli_piped(&[
        &["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS],
        &["xid", "key", "add", "--nickname", "Bob", BOB_PUBKEYS],
    ])
    .unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                    'allow': 'All'
                    'nickname': "Bob"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // $ XID_DOC_UPDATED=`envelope xid key update $BOB_PUBKEYS \
    //     --allow 'encrypt' \
    //     --allow 'sign' \
    //     $XID_DOC`

    let xid_doc_updated = run_cli(&[
        "xid",
        "key",
        "update",
        BOB_PUBKEYS,
        "--allow",
        "encrypt",
        "--allow",
        "sign",
        &xid_doc,
    ])
    .unwrap();

    // $ envelope format $XID_DOC_UPDATED
    // println!("xid_doc_updated: {}", xid_doc_updated);

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc_updated],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                    'nickname': "Bob"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid key count`: Count the Number of Keys in a XID Document
    //
    // ```
    // $ envelope xid key count $XID_DOC_UPDATED
    //
    // 2
    // ```

    run_cli_expect(&["xid", "key", "count", &xid_doc_updated], "2").unwrap();

    // #### `xid key at`: Returns the Key at the Specified Index

    // The indexes are zero-based, and in the order the key assertions appear in
    // the XID document's Gordian Envelope, which is not necessarily the order
    // they appear via `envelope format`.

    // ```
    // $ envelope xid key at 0 $XID_DOC_UPDATED | envelope format

    // PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [     'allow':
    // 'All'     'nickname': "Alice"
    // ]

    // $ envelope xid key at 1 $XID_DOC_UPDATED | envelope format

    // PublicKeys(e2c18423, SigningPublicKey(SchnorrPublicKey(f0638394)),
    // EncapsulationPublicKey(X25519PublicKey(4af6be52))) [     'allow':
    // 'Encrypt'     'allow': 'Sign'
    //     'nickname': "Bob"
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "key", "at", "0", &xid_doc_updated],
            &["format"]
        ],
        indoc! {r#"
            PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
        "#}.trim()
    ).unwrap();

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "key", "at", "1", &xid_doc_updated],
            &["format"]
        ],
        indoc! {r#"
            PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                'allow': 'Encrypt'
                'allow': 'Sign'
                'nickname': "Bob"
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid key all`: Returns All Keys in a XID Document
    //
    // The keys envelopes separated by newlines.
    //
    // ```
    // $ envelope xid key all $XID_DOC_UPDATED
    //
    // ur:envelope/lstpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgoycscstpsoihfpjziniaihqdkobsbw
    // ur:envelope/lrtpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfdoycsfncsgaoycscstpsoiafwjlidkpjkotey
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["xid", "key", "all", &xid_doc_updated],
        indoc! {r#"
            ur:envelope/lstpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgoycscstpsoihfpjziniaihqdkobsbw
            ur:envelope/lrtpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfdoycsfncsgaoycscstpsoiafwjlidkpjkotey
        "#}.trim()
    ).unwrap();
}

const XID_DOC_UPDATED: &str = "ur:xid/tpsplstpsotanshdhdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfessoyaylstpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgoycscstpsoihfpjziniaihoyaylrtpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfdoycsfncsgaoycscstpsoiafwjlidbeglldte";

#[test]
fn test_xid_key_find() {
    // ##### `xid key find name`: Find a Key by the Given Name
    //
    // May return multiple key envelopes.
    //
    // ```
    // $ envelope xid key find name 'Alice' $XID_DOC_UPDATED | envelope format
    //
    // PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //     'allow': 'All'
    //     'nickname': "Alice"
    // ]

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "key", "find", "name", "Alice", XID_DOC_UPDATED],
            &["format"]
        ],
        indoc! {r#"
            PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
        "#}.trim()
    ).unwrap();

    // $ envelope xid key find name 'Wolf' $XID_DOC_UPDATED
    //
    // (nothing returned)
    // ```

    run_cli_expect(
        &["xid", "key", "find", "name", "Wolf", XID_DOC_UPDATED],
        "",
    )
    .unwrap();

    // ##### `xid key find inception`: Find the Document's Inception Key
    //
    // Returns at most one key envelope.
    //
    // ```
    // $ envelope xid key find inception $XID_DOC_UPDATED | envelope format
    //
    // PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //     'allow': 'All'
    //     'nickname': "Alice"
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "key", "find", "inception", XID_DOC_UPDATED],
            &["format"]
        ],
        indoc! {r#"
            PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                'allow': 'All'
                'nickname': "Alice"
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_key_remove() {
    // #### `xid key remove`: Remove a Given Key
    //
    // ```
    // $ XID_DOC_REMOVED=`envelope xid key remove $ALICE_PUBKEYS $XID_DOC_UPDATED`
    // $ envelope format $XID_DOC_REMOVED
    //
    // XID(93a4d4e7) [
    //     'key': PublicKeys(e2c18423, SigningPublicKey(SchnorrPublicKey(f0638394)), EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //         'nickname': "Bob"
    //     ]
    // ]

    let xid_doc_removed =
        run_cli(&["xid", "key", "remove", ALICE_PUBKEYS, XID_DOC_UPDATED])
            .unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc_removed],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                    'nickname': "Bob"
                ]
            ]
        "#}.trim()
    ).unwrap();

    //
    // $ envelope xid key find inception $XID_DOC_REMOVED
    //
    // (nothing returned)
    // ```

    run_cli_expect(&["xid", "key", "find", "inception", &xid_doc_removed], "")
        .unwrap();
}

#[test]
fn test_xid_method() {
    // ### `xid method`: Work with Resolution Methods
    //
    // Resolution methods are URIs that describe how to resolve a XID. They are
    // used to find the complete, most up-to-date version of a XID document.
    //
    // ```
    // $ envelope xid method --help
    // ```
    //
    // #### `xid method add`: Add a Resolution Method to a XID Document
    //
    // ```
    // $ XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`

    let xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    // $ XID_DOC_WITH_RESOLVERS=`envelope xid method add 'https://resolver.example.com/' $XID_DOC | \
    //     envelope xid method add
    // 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6'`

    let xid_doc_with_resolvers = run_cli_piped(
        &[
            &["xid", "method", "add", "https://resolver.example.com/", &xid_doc],
            &[
                "xid",
                "method",
                "add",
                "btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6",
            ],
        ]
    ).unwrap();

    // $ envelope format $XID_DOC_WITH_RESOLVERS
    //
    // XID(93a4d4e7) [
    //     'dereferenceVia':
    // URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)     'dereferenceVia': URI(https://resolver.example.com/)
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &xid_doc_with_resolvers],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
                'dereferenceVia': URI(https://resolver.example.com/)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    //
    // #### `xid method count`: Count the Number of Resolution Methods in a XID
    // Document
    //
    // ```
    // $ envelope xid method count $XID_DOC_WITH_RESOLVERS
    //
    // 2
    // ```

    run_cli_expect(&["xid", "method", "count", &xid_doc_with_resolvers], "2")
        .unwrap();

    //
    // #### `xid method at`: Return the Resolution Method at the Specified Index
    //
    // The indexes are zero-based, and in the order the resolution methods
    // appear in the XID document's Gordian Envelope, which is not necessarily
    // the order they appear via `envelope format`.
    //
    // ```
    // $ envelope xid method at 0 $XID_DOC_WITH_RESOLVERS
    //
    // https://resolver.example.com/

    run_cli_expect(
        &["xid", "method", "at", "0", &xid_doc_with_resolvers],
        "https://resolver.example.com/",
    )
    .unwrap();

    //
    // $ envelope xid method at 1 $XID_DOC_WITH_RESOLVERS
    //
    // btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
    // ```

    run_cli_expect(
        &["xid", "method", "at", "1", &xid_doc_with_resolvers],
        "btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6",
    )
    .unwrap();

    //
    // #### `xid method all`: List All Resolution Methods in a XID Document
    //
    // ```
    // $ envelope xid method all $XID_DOC_WITH_RESOLVERS
    //
    // https://resolver.example.com/
    // btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["xid", "method", "all", &xid_doc_with_resolvers],
        indoc! {r#"
            https://resolver.example.com/
            btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
        "#}.trim()
    ).unwrap();

    //
    // #### `xid method remove`: Remove a Resolution Method from a XID Document
    //
    // ```
    // $ envelope xid method remove 'https://resolver.example.com/' $XID_DOC_WITH_RESOLVERS | envelope format
    //
    // XID(93a4d4e7) [
    //     'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
    //     'key': PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //         'allow': 'All'
    //         'nickname': "Alice"
    //     ]
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "method", "remove", "https://resolver.example.com/", &xid_doc_with_resolvers],
            &["format"]
        ],
        indoc! {r#"
            XID(93a4d4e7) [
                'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_delegate() {
    // ### `xid delegate`: Work with Delegates
    //
    // A *delegate* is XID document that is authorized to act on behalf of the
    // *principal* XID document. A delegate can be granted any permissions, but
    // its *effective* permissions will be a subset of the permissions of the
    // principal XID document.
    //
    // ```
    // $ envelope xid delegate --help
    // ```
    //
    // #### `xid delegate add`: Add a Delegate to a XID Document
    //
    // This example:
    //
    // - creates a XID documents for Alice, Bob, Carol, and Dave,
    // - grants Carol all permissions on behalf of Alice,
    // - grants Bob the ability to sign and encrypt on behalf of Alice,
    // - grants Dave the ability to elide data on behalf of Alice,
    //     - but only add's Dave's XID identifier to the XID document, which
    //       means it will have to be resolved to be used.
    //
    // ```
    //
    // $ ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`

    let alice_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    // $ BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`

    let bob_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Bob", BOB_PUBKEYS]).unwrap();

    // $ CAROL_XID_DOC=`envelope xid new --nickname 'Carol' $CAROL_PUBKEYS`

    let carol_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Carol", CAROL_PUBKEYS]).unwrap();

    // $ DAVE_XID_DOC=`envelope xid new --nickname 'Dave' $DAVE_PUBKEYS`

    let dave_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Dave", DAVE_PUBKEYS]).unwrap();

    // $ DAVE_XID=`envelope xid id $DAVE_XID_DOC`

    let dave_xid = run_cli(&["xid", "id", &dave_xid_doc]).unwrap();

    // $ ALICE_XID_DOC=`envelope xid delegate add --allow 'all' $CAROL_XID_DOC
    // $ALICE_XID_DOC`

    let alice_xid_doc = run_cli(&[
        "xid",
        "delegate",
        "add",
        "--allow",
        "all",
        &carol_xid_doc,
        &alice_xid_doc,
    ])
    .unwrap();

    // $ ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow
    // 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`

    let alice_xid_doc = run_cli(&[
        "xid",
        "delegate",
        "add",
        "--allow",
        "sign",
        "--allow",
        "encrypt",
        &bob_xid_doc,
        &alice_xid_doc,
    ])
    .unwrap();

    // $ ALICE_XID_DOC=`envelope xid delegate add --allow 'elide' $DAVE_XID
    // $ALICE_XID_DOC`

    let alice_xid_doc = run_cli(&[
        "xid",
        "delegate",
        "add",
        "--allow",
        "elide",
        &dave_xid,
        &alice_xid_doc,
    ])
    .unwrap();

    // $ envelope format $ALICE_XID_DOC
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(3636003e)
    //     } [
    //         'allow': 'Elide'
    //     ]
    //     'delegate': {
    //         XID(61b1f3c7) [
    //             'key': PublicKeys(eebd4add,
    // SigningPublicKey(SchnorrPublicKey(8684e3e4)),
    // EncapsulationPublicKey(X25519PublicKey(0995c476))) [
    // 'allow': 'All'                 'nickname': "Carol"
    //             ]
    //         ]
    //     } [
    //         'allow': 'All'
    //     ]
    //     'delegate': {
    //         XID(f1199a75) [
    //             'key': PublicKeys(e2c18423,
    // SigningPublicKey(SchnorrPublicKey(f0638394)),
    // EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    // 'allow': 'All'                 'nickname': "Bob"
    //             ]
    //         ]
    //     } [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(3636003e)
                } [
                    'allow': 'Elide'
                ]
                'delegate': {
                    XID(61b1f3c7) [
                        'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
                            'allow': 'All'
                            'nickname': "Carol"
                        ]
                    ]
                } [
                    'allow': 'All'
                ]
                'delegate': {
                    XID(f1199a75) [
                        'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                            'allow': 'All'
                            'nickname': "Bob"
                        ]
                    ]
                } [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid delegate count`: Count the Number of Delegates in a XID
    // Document
    //
    // ```
    // $ envelope xid delegate count $ALICE_XID_DOC
    //
    // 3
    // ```

    run_cli_expect(&["xid", "delegate", "count", &alice_xid_doc], "3").unwrap();

    // #### `xid delegate at`: Return the Delegate at the Specified Index
    //
    // The indexes are zero-based, and in the order the delegate assertions
    // appear in the XID document's Gordian Envelope, which is not necessarily
    // the order they appear via `envelope format`.
    //
    // ```
    // $ envelope xid delegate at 1 $ALICE_XID_DOC | envelope format
    //
    // {
    //     XID(f1199a75) [
    //         'key': PublicKeys(e2c18423, SigningPublicKey(SchnorrPublicKey(f0638394)), EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    //             'allow': 'All'
    //             'nickname': "Bob"
    //         ]
    //     ]
    // } [
    //     'allow': 'Encrypt'
    //     'allow': 'Sign'
    // ]

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "delegate", "at", "1", &alice_xid_doc],
            &["format"]
        ],
        indoc! {r#"
            {
                XID(f1199a75) [
                    'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                        'allow': 'All'
                        'nickname': "Bob"
                    ]
                ]
            } [
                'allow': 'Encrypt'
                'allow': 'Sign'
            ]
        "#}.trim()
    ).unwrap();

    // $ envelope xid delegate at 0 $ALICE_XID_DOC | envelope format
    //
    // {
    //     XID(61b1f3c7) [
    //         'key': PublicKeys(eebd4add,
    // SigningPublicKey(SchnorrPublicKey(8684e3e4)),
    // EncapsulationPublicKey(X25519PublicKey(0995c476))) [
    // 'allow': 'All'             'nickname': "Carol"
    //         ]
    //     ]
    // } [
    //     'allow': 'All'
    // ]

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "delegate", "at", "0", &alice_xid_doc],
            &["format"]
        ],
        indoc! {r#"
            {
                XID(61b1f3c7) [
                    'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
                        'allow': 'All'
                        'nickname': "Carol"
                    ]
                ]
            } [
                'allow': 'All'
            ]
        "#}.trim()
    ).unwrap();

    // $ envelope xid delegate at 2 $ALICE_XID_DOC | envelope format
    //
    // {
    //     XID(3636003e)
    // } [
    //     'allow': 'Elide'
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "delegate", "at", "2", &alice_xid_doc],
            &["format"]
        ],
        indoc! {r#"
            {
                XID(3636003e)
            } [
                'allow': 'Elide'
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid delegate all`: List All Delegates in a XID Document
    //
    // ```
    // $ envelope xid delegate all $ALICE_XID_DOC
    //
    // ur:envelope/lstpsplftpsotanshdhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloyaylstpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoybdtpsoiafwjlidoycsfncsfgoycsfncsfdoycsfncsgauyzsurla
    // ur:envelope/lftpsplftpsotanshdhdcxhspawfstecswotwpbsweiowlsrmyfpwpskmeonrtjsrhetsrhnaxfwylvtvsuorkoyaylstpsotansgylftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwoycsfncsfgoybdtpsoihfxhsjpjljzoycsfncsfgzsiddlec
    // ur:envelope/lftpsptpsotanshdhdcxenenaefmosgecksalokgmnrhgrsemhhfnlfssroxbytkvllrvsrhgtgscpvswfveoycsfncsgegtgtyljt
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["xid", "delegate", "all", &alice_xid_doc],
        indoc! {r#"
            ur:envelope/lftpsplftpsotanshdhdcxhspawfstecswotwpbsweiowlsrmyfpwpskmeonrtjsrhetsrhnaxfwylvtvsuorkoyaylstpsotansgylftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwoycsfncsfgoycscstpsoihfxhsjpjljzoycsfncsfgknhpttwe
            ur:envelope/lstpsplftpsotanshdhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloyaylstpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfgoycscstpsoiafwjlidoycsfncsfdoycsfncsgawnftoeoy
            ur:envelope/lftpsptpsotanshdhdcxenenaefmosgecksalokgmnrhgrsemhhfnlfssroxbytkvllrvsrhgtgscpvswfveoycsfncsgegtgtyljt
        "#}.trim()
    ).unwrap();

    // #### `xid delegate find`: Find a Delegate by its XID Identifier
    //
    // ```
    // $ envelope xid delegate find $DAVE_XID $ALICE_XID_DOC | envelope format
    //
    // {
    //     XID(3636003e)
    // } [
    //     'allow': 'Elide'
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_piped_expect(
        &[
            &["xid", "delegate", "find", &dave_xid, &alice_xid_doc],
            &["format"]
        ],
        indoc! {r#"
            {
                XID(3636003e)
            } [
                'allow': 'Elide'
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid delegate update`: Update an Existing Delegate in an Existing
    // XID Document
    //
    // - Replaces the existing delegate with the one provided, which must
    //   already exist in the XID document.
    // - Replaces the permissions of the existing delegate with the ones
    //   provided.
    //
    // In this example:
    // - Carol's XID document is replaced with her bare XID, and
    // - her permissions are reduced.
    //
    // ```
    // $ CAROL_XID=`envelope xid id $CAROL_XID_DOC`

    let carol_xid = run_cli(&["xid", "id", &carol_xid_doc]).unwrap();

    // $ ALICE_XID_DOC_UPDATED=`envelope xid delegate update --allow 'auth'
    // --allow 'encrypt' --allow 'sign' $CAROL_XID $ALICE_XID_DOC`

    let alice_xid_doc_updated = run_cli(&[
        "xid",
        "delegate",
        "update",
        "--allow",
        "auth",
        "--allow",
        "encrypt",
        "--allow",
        "sign",
        &carol_xid,
        &alice_xid_doc,
    ])
    .unwrap();

    // $ envelope format $ALICE_XID_DOC_UPDATED
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(3636003e)
    //     } [
    //         'allow': 'Elide'
    //     ]
    //     'delegate': {
    //         XID(61b1f3c7)
    //     } [
    //         'allow': 'Auth'
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'delegate': {
    //         XID(f1199a75) [
    //             'key': PublicKeys(e2c18423,
    // SigningPublicKey(SchnorrPublicKey(f0638394)),
    // EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    // 'allow': 'All'                 'nickname': "Bob"
    //             ]
    //         ]
    //     } [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc_updated],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(3636003e)
                } [
                    'allow': 'Elide'
                ]
                'delegate': {
                    XID(61b1f3c7)
                } [
                    'allow': 'Auth'
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'delegate': {
                    XID(f1199a75) [
                        'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                            'allow': 'All'
                            'nickname': "Bob"
                        ]
                    ]
                } [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // #### `xid delegate remove`: Remove a Delegate from a XID Document
    //
    // ```
    // $ BOB_XID=`envelope xid id $BOB_XID_DOC`

    let bob_xid = run_cli(&["xid", "id", &bob_xid_doc]).unwrap();

    // $ ALICE_XID_DOC_UPDATED=`envelope xid delegate remove $BOB_XID
    // $ALICE_XID_DOC_UPDATED`

    let alice_xid_doc_updated = run_cli(&[
        "xid",
        "delegate",
        "remove",
        &bob_xid,
        &alice_xid_doc_updated,
    ])
    .unwrap();

    // $ envelope format $ALICE_XID_DOC_UPDATED
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(3636003e)
    //     } [
    //         'allow': 'Elide'
    //     ]
    //     'delegate': {
    //         XID(61b1f3c7)
    //     } [
    //         'allow': 'Auth'
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    // ]
    // ```

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc_updated],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(3636003e)
                } [
                    'allow': 'Elide'
                ]
                'delegate': {
                    XID(61b1f3c7)
                } [
                    'allow': 'Auth'
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_service() {
    bc_envelope::register_tags();

    // ### `xid service`: Work with Services
    //
    // ```
    // $ envelope xid service --help
    // ```
    //
    // Services are URI endpoints along with the keys, delegates, and
    // permissions that are allowed to use them.
    //
    // The keys and delegates in a Service declaration are references to keys
    // and delegates that must already exist in the XID document.
    //
    // ```
    // $ ALICE_PRVKEY_BASE=ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct
    // $ ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEY_BASE`
    // $ BOB_PRVKEY_BASE=ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva
    // $ BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEY_BASE`
    // $ CAROL_PRVKEY_BASE="ur:crypto-prvkey-base/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos"
    // $ CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEY_BASE`
    // ```
    //
    // Alice creates a basic XID document.
    //
    // ```
    // $ ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
    // $ envelope format $ALICE_XID_DOC
    //
    // XID(93a4d4e7) [
    //     'key': PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //         'allow': 'All'
    //         'nickname': "Alice"
    //     ]
    // ]
    // ```

    let alice_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Alice", ALICE_PUBKEYS]).unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Alice adds Bob as a delegate.
    //
    // ```
    // $ BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`
    // $ ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
    // $ envelope format $ALICE_XID_DOC
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(f1199a75) [
    //             'key': PublicKeys(e2c18423, SigningPublicKey(SchnorrPublicKey(f0638394)), EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    //                 'allow': 'All'
    //                 'nickname': "Bob"
    //             ]
    //         ]
    //     } [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //         'allow': 'All'
    //         'nickname': "Alice"
    //     ]
    // ]
    // ```

    let bob_xid_doc =
        run_cli(&["xid", "new", "--nickname", "Bob", BOB_PUBKEYS]).unwrap();

    let alice_xid_doc = run_cli(&[
        "xid",
        "delegate",
        "add",
        "--allow",
        "sign",
        "--allow",
        "encrypt",
        &bob_xid_doc,
        &alice_xid_doc,
    ])
    .unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(f1199a75) [
                        'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                            'allow': 'All'
                            'nickname': "Bob"
                        ]
                    ]
                } [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Alice adds a secure messaging service.
    //
    // ```
    // $ ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    //     --name 'Messaging' \
    //     --capability 'com.example.messaging' \
    //     --allow 'sign' \
    //     --allow 'encrypt' \
    //     --key $ALICE_PUBKEYS \
    //     --delegate $BOB_XID_DOC \
    //     "https://messaging.example.com" \
    //     $ALICE_XID_DOC`
    // ```

    let alice_xid_doc = run_cli(&[
        "xid",
        "service",
        "add",
        "--name",
        "Messaging",
        "--capability",
        "com.example.messaging",
        "--allow",
        "sign",
        "--allow",
        "encrypt",
        "--key",
        ALICE_PUBKEYS,
        "--delegate",
        &bob_xid_doc,
        "https://messaging.example.com",
        &alice_xid_doc,
    ])
    .unwrap();

    // $ envelope format $ALICE_XID_DOC_WITH_SERVICE
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(f1199a75) [
    //             'key': PublicKeys(e2c18423,
    // SigningPublicKey(SchnorrPublicKey(f0638394)),
    // EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    // 'allow': 'All'                 'nickname': "Bob"
    //             ]
    //         ]
    //     } [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0,
    // SigningPublicKey(SchnorrPublicKey(26712894)),
    // EncapsulationPublicKey(X25519PublicKey(00b42db3))) [         'allow':
    // 'All'         'nickname': "Alice"
    //     ]
    //     'service': URI(https://messaging.example.com) [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //         'capability': "com.example.messaging"
    //         'delegate': Reference(f1199a75)
    //         'key': Reference(cab108a0)
    //         'name': "Messaging"
    //     ]
    // ]

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(f1199a75) [
                        'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                            'allow': 'All'
                            'nickname': "Bob"
                        ]
                    ]
                } [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'service': URI(https://messaging.example.com) [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                    'capability': "com.example.messaging"
                    'delegate': Reference(f1199a75)
                    'key': Reference(cab108a0)
                    'name': "Messaging"
                ]
            ]
        "#}.trim()
    ).unwrap();

    // Alice adds a second service for retrieving her status.
    //
    // ```
    // $ ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    //     --name 'Status' \
    //     --capability 'com.example.status' \
    //     --allow 'sign' \
    //     --key $ALICE_PUBKEYS \
    //     "https://status.example.com/alice" \
    //     $ALICE_XID_DOC_WITH_SERVICE`
    //
    // $ envelope format $ALICE_XID_DOC_WITH_SERVICE
    //
    // XID(93a4d4e7) [
    //     'delegate': {
    //         XID(f1199a75) [
    //             'key': PublicKeys(e2c18423, SigningPublicKey(SchnorrPublicKey(f0638394)), EncapsulationPublicKey(X25519PublicKey(4af6be52))) [
    //                 'allow': 'All'
    //                 'nickname': "Bob"
    //             ]
    //         ]
    //     } [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //     ]
    //     'key': PublicKeys(cab108a0, SigningPublicKey(SchnorrPublicKey(26712894)), EncapsulationPublicKey(X25519PublicKey(00b42db3))) [
    //         'allow': 'All'
    //         'nickname': "Alice"
    //     ]
    //     'service': URI(https://messaging.example.com) [
    //         'allow': 'Encrypt'
    //         'allow': 'Sign'
    //         'capability': "com.example.messaging"
    //         'delegate': Reference(f1199a75)
    //         'key': Reference(cab108a0)
    //         'name': "Messaging"
    //     ]
    //     'service': URI(https://status.example.com/alice) [
    //         'allow': 'Sign'
    //         'capability': "com.example.status"
    //         'key': Reference(cab108a0)
    //         'name': "Status"
    //     ]
    // ]
    // ```

    let alice_xid_doc = run_cli(&[
        "xid",
        "service",
        "add",
        "--name",
        "Status",
        "--capability",
        "com.example.status",
        "--allow",
        "sign",
        "--key",
        ALICE_PUBKEYS,
        "https://status.example.com/alice",
        &alice_xid_doc,
    ])
    .unwrap();

    #[rustfmt::skip]
    run_cli_expect(
        &["format", &alice_xid_doc],
        indoc! {r#"
            XID(93a4d4e7) [
                'delegate': {
                    XID(f1199a75) [
                        'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
                            'allow': 'All'
                            'nickname': "Bob"
                        ]
                    ]
                } [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                ]
                'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
                    'allow': 'All'
                    'nickname': "Alice"
                ]
                'service': URI(https://messaging.example.com) [
                    'allow': 'Encrypt'
                    'allow': 'Sign'
                    'capability': "com.example.messaging"
                    'delegate': Reference(f1199a75)
                    'key': Reference(cab108a0)
                    'name': "Messaging"
                ]
                'service': URI(https://status.example.com/alice) [
                    'allow': 'Sign'
                    'capability': "com.example.status"
                    'key': Reference(cab108a0)
                    'name': "Status"
                ]
            ]
        "#}.trim()
    ).unwrap();
}

#[test]
fn test_xid_encrypted_keys_preserved() {
    // Test that encrypted private keys are preserved when modifying XID
    // documents without providing a password.

    // Generate private keys and create XID with encrypted private keys
    // $ envelope generate prvkeys | \
    // envelope xid new --private encrypt --encrypt-password "secret"

    let xid_encrypted = run_cli_piped(&[
        &["generate", "prvkeys"],
        &[
            "xid",
            "new",
            "--private",
            "encrypt",
            "--encrypt-password",
            "secret",
        ],
    ])
    .unwrap();

    // Verify it contains ENCRYPTED by formatting it
    let formatted = run_cli_stdin(&["format"], &xid_encrypted).unwrap();
    assert!(formatted.contains("ENCRYPTED"));
    assert!(formatted.contains("hasSecret"));

    // Add a resolution method WITHOUT providing password
    // $ envelope xid method add https://resolver.example.com <<< $XID_ENCRYPTED

    let xid_with_method = run_cli_stdin(
        &["xid", "method", "add", "https://resolver.example.com"],
        &xid_encrypted,
    )
    .unwrap();

    // Should still have encrypted keys
    let formatted = run_cli_stdin(&["format"], &xid_with_method).unwrap();
    assert!(formatted.contains("ENCRYPTED"));
    assert!(formatted.contains("hasSecret"));
    assert!(formatted.contains("dereference"));

    // Verify we can still decrypt with the password by adding another key
    // $ envelope generate prvkeys | envelope xid key add --password "secret" \
    //   --private encrypt --encrypt-password "secret" <<< $XID_WITH_METHOD

    let prvkeys_for_new_key = run_cli(&["generate", "prvkeys"]).unwrap();

    let xid_final = run_cli_piped_stdin(
        &[&[
            "xid",
            "key",
            "add",
            "--password",
            "secret",
            "--private",
            "encrypt",
            "--encrypt-password",
            "secret",
        ]],
        &format!("{}\n{}", prvkeys_for_new_key, xid_with_method),
    )
    .unwrap();

    // Should successfully decrypt, add the new key, and re-encrypt
    // Both keys should now be encrypted
    let formatted = run_cli_stdin(&["format"], &xid_final).unwrap();
    assert!(formatted.contains("ENCRYPTED"));
    assert!(formatted.contains("hasSecret"));
    // Should have 2 keys now (inception key + newly added key)
    assert_eq!(formatted.matches("'key':").count(), 2);
}

#[test]
fn test_xid_key_private_flag() {
    // Test the --private flag on key retrieval commands

    // Create XID with encrypted private key
    let prvkey = run_cli(&["generate", "prvkeys"]).unwrap();
    let xid_encrypted = run_cli(&[
        "xid",
        "new",
        &prvkey,
        "--private",
        "encrypt",
        "--encrypt-password",
        "secret",
        "--nickname",
        "TestKey",
    ])
    .unwrap();

    // Test 1: xid key all without --private (returns public keys)
    let public_keys = run_cli(&["xid", "key", "all", &xid_encrypted]).unwrap();
    assert!(public_keys.starts_with("ur:envelope/"));
    let formatted_public = run_cli_stdin(&["format"], &public_keys).unwrap();
    assert!(formatted_public.contains("PublicKeys"));
    assert!(formatted_public.contains("ENCRYPTED")); // Public key envelope includes encrypted assertion

    // Test 2: xid key all --private without password (returns encrypted
    // envelope)
    let encrypted =
        run_cli(&["xid", "key", "all", "--private", &xid_encrypted]).unwrap();
    let formatted_encrypted = run_cli_stdin(&["format"], &encrypted).unwrap();
    assert!(formatted_encrypted.contains("ENCRYPTED"));
    assert!(formatted_encrypted.contains("hasSecret"));

    // Test 3: xid key all --private with correct password (returns
    // ur:crypto-prvkeys)
    let decrypted = run_cli(&[
        "xid",
        "key",
        "all",
        "--private",
        "--password",
        "secret",
        &xid_encrypted,
    ])
    .unwrap();
    // Should return ur:crypto-prvkeys directly, not an envelope
    assert!(decrypted.starts_with("ur:crypto-prvkeys/"));

    // Test 4: xid key all --private with wrong password (should error)
    let result = run_cli(&[
        "xid",
        "key",
        "all",
        "--private",
        "--password",
        "wrong",
        &xid_encrypted,
    ]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid password"));

    // Test 5: xid key at 0 --private with password (returns ur:crypto-prvkeys)
    let decrypted_at = run_cli(&[
        "xid",
        "key",
        "at",
        "0",
        "--private",
        "--password",
        "secret",
        &xid_encrypted,
    ])
    .unwrap();
    assert!(decrypted_at.starts_with("ur:crypto-prvkeys/"));

    // Test 6: xid key find inception --private
    let inception_encrypted = run_cli(&[
        "xid",
        "key",
        "find",
        "inception",
        "--private",
        &xid_encrypted,
    ])
    .unwrap();
    let formatted_inception =
        run_cli_stdin(&["format"], &inception_encrypted).unwrap();
    assert!(formatted_inception.contains("ENCRYPTED"));

    // Test 7: xid key find name --private with password (returns
    // ur:crypto-prvkeys)
    let found_by_name = run_cli(&[
        "xid",
        "key",
        "find",
        "name",
        "TestKey",
        "--private",
        "--password",
        "secret",
        &xid_encrypted,
    ])
    .unwrap();
    assert!(found_by_name.starts_with("ur:crypto-prvkeys/"));

    // Test 8: Unencrypted key with --private (returns ur:crypto-prvkeys)
    let xid_unencrypted = run_cli(&["xid", "new", &prvkey]).unwrap();
    let unencrypted_private =
        run_cli(&["xid", "key", "all", "--private", &xid_unencrypted]).unwrap();
    assert!(unencrypted_private.starts_with("ur:crypto-prvkeys/"));
}
