# `envelope` XID Support

The `envelope` tool now includes basic support for working with [XID Documents](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-010-xid.md). This includes creating, updating, and removing keys, resolution methods, and delegates. XID documents are a type of envelope that contain public keys, permissions, and other metadata. They are used to represent the identity of a person, device, or service.

## Future Work

- Working with Provenance Marks in general
- Working with Provenance Marks in XID documents
- Working with signed XID documents

## Import All Envelope URs

Anywhere in `envelope` that accepts a `ur:envelope` can also accept any other UR type, including XID documents.

👉
```
$ XID_DOC=ur:xid/tpsplftpsotanshdhdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnoyaylftpsotansgylftanshfhdcxhslkfzemaylrwttynsdlghrydpmdfzvdglndloimaahykorefddtsguogmvlahqztansgrhdcxetlewzvlwyfdtobeytidosbamkswaomwwfyabakssakggegychesmerkcatekpcxoycsfncsfggmplgshd
$ envelope format $XID_DOC

XID(71274df1) [
    'key': PublicKeys(eb9b1cae) [
        'allow': 'All'
    ]
]
```

Note that this does not validate the XID document (or any other envelope-containing UR), it just reads the UR’s envelope, meaning you can manipulate it like any other envelope.

👉
```
$ envelope assertion at 0 $XID_DOC | \
    envelope format

'key': PublicKeys(eb9b1cae) [
    'allow': 'All'
]

$ envelope assertion at 0 $XID_DOC | \
    envelope extract object | \
    envelope assertion at 0 | \
    envelope format

'allow': 'All'
```

XID Documents always have the XID CBOR object as their subject. So you can extract the bare XID of a XID document using the `extract xid` subcommand.

👉
```
$ BARE_XID=`envelope extract xid $XID_DOC`
$ echo $BARE_XID

ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Bare XID URs, although they do not contain an envelope (they are just CBOR) are also internally imported into an empty XID document and then turned into an envelope, with just the XID as its subject.

👉
```
$ envelope format $BARE_XID

XID(71274df1)
```

This means that bare XIDs can be brought in like any other envelope subject. Again, no XID Document-specific validation is done.

👉
```
$ envelope assertion add pred-obj string "knows" string "Bob" $BARE_XID | envelope format

XID(71274df1) [
    "knows": "Bob"
]
```

## `xid` Subcommand

The `xid` subcommand parses and manipulates XID documents. Invalid XID documents will be rejected. All XID documents returned by its subcommands are in `ur:xid` form.

👉
```
$ envelope xid --help
```

### `xid id`: Extract the Bare XID from a XID Document

Unlike the technique of simply extracting the subject above, this subcommand validates the entire XID document.

👉
```
$ XID_ID=`envelope xid id $XID_DOC`
$ echo $XID_ID

ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Extracting the bare XID from a bare XID UR is idempotent.

👉
```
$ envelope xid id $XID_ID

ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Several output formats are supported. `ur` is the default and is machine-readable, while the others are human-readable.

👉
```
$ envelope xid id \
    --format ur \
    --format hex \
    --format bytewords \
    --format bytemoji \
    $XID_DOC

ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
XID(71274df1)
🅧 JUGS DELI GIFT WHEN
🅧 🌊 😹 🌽 🐞
```

### `xid new`: Create New XID Documents From Public or Private Keys

The `xid new` subcommand converts a `PrivateKeyBase` or `PublicKeys` into a XID Document with the provided key as the inception key.

👉
```
$ ALICE_PRVKEY_BASE=ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct
$ ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEY_BASE`

$ envelope xid new $ALICE_PUBKEYS | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
    ]
]
```

A XID document returned by the `xid new` subcommand is returned as a `ur:xid`.

👉
```
$ envelope xid new $ALICE_PUBKEYS

ur:xid/tpsplftpsotanshdhdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfessoyaylftpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgzckbfwes
```

If a `PrivateKeyBase` is provided, by default the salted private key itself will be included.

👉
```
$ envelope xid new $ALICE_PRVKEY_BASE | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        {
            'privateKey': PrivateKeyBase
        } [
            'salt': Salt
        ]
        'allow': 'All'
    ]
]
```

The private key can be omitted using the `--private omit` option, or elided using `--private elide`.

👉
```
$ envelope xid new $ALICE_PRVKEY_BASE --private omit | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
    ]
]

$ envelope xid new $ALICE_PRVKEY_BASE --private elide | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        ELIDED
    ]
]
```

One or more endpoint URIs may be added to the inception key.

👉
```
$ envelope xid new $ALICE_PUBKEYS \
    --endpoint 'https://endpoint.example.com/' \
    --endpoint 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6' \
    | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'endpoint': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
        'endpoint': URI(https://endpoint.example.com/)
    ]
]
```

One or more permissions may be specified for the inception key. These replace the default `'All'` permission.

👉
```
$ envelope xid new $ALICE_PUBKEYS \
    --allow 'encrypt' \
    --allow 'sign' \
    | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
]
```

The key may be given a user-assigned name ("pet name") using the `--name` option.

👉
```
$ envelope xid new $ALICE_PUBKEYS \
    --name 'Alice'\''s Key' \
    | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice's Key"
    ]
]
```

### `xid key`: Work With XID Document Keys

👉
```
$ envelope xid key --help
```

#### `xid key add`: Add a New Key to an Existing XID Document

All the same options as `xid new` are available. The same key may not be added twice.

👉
```
$ XID_DOC=`envelope xid new --name 'Alice' $ALICE_PUBKEYS`

$ BOB_PRVKEY_BASE=ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva
$ BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEY_BASE`

$ envelope xid key add --name 'Bob' $BOB_PUBKEYS $XID_DOC | envelope format

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
    'key': PublicKeys(e2c18423) [
        'allow': 'All'
        'name': "Bob"
    ]
]
```

#### `xid key update`: Update an Existing Key in an Existing XID Document

All the same options as `xid new` are available. The key must already exist in the XID document.

👉
```
$ XID_DOC=`envelope xid new --name 'Alice' $ALICE_PUBKEYS | envelope xid key add --name 'Bob' $BOB_PUBKEYS`
$ envelope format $XID_DOC

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
    'key': PublicKeys(e2c18423) [
        'allow': 'All'
        'name': "Bob"
    ]
]

$ XID_DOC_UPDATED=`envelope xid key update $BOB_PUBKEYS \
    --allow 'encrypt' \
    --allow 'sign' \
    $XID_DOC`
$ envelope format $XID_DOC_UPDATED

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
    'key': PublicKeys(e2c18423) [
        'allow': 'Encrypt'
        'allow': 'Sign'
        'name': "Bob"
    ]
]
```

#### `xid key count`: Count the Number of Keys in a XID Document

👉
```
$ envelope xid key count $XID_DOC_UPDATED

2
```

#### `xid key at`: Returns the Key at the Specified Index

The indexes are zero-based, and in the order the key assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

👉
```
$ envelope xid key at 0 $XID_DOC_UPDATED | envelope format

PublicKeys(cab108a0) [
    'allow': 'All'
    'name': "Alice"
]

$ envelope xid key at 1 $XID_DOC_UPDATED | envelope format

PublicKeys(e2c18423) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'name': "Bob"
]
```

#### `xid key all`: Returns All Keys in a XID Document

The keys envelopes separated by newlines.

👉
```
$ envelope xid key all $XID_DOC_UPDATED

ur:envelope/lstpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoybdtpsoihfpjziniaihoycsfncsfgrnkedtns
ur:envelope/lrtpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoybdtpsoiafwjlidoycsfncsfdoycsfncsgafpmnvszt
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

👉
```
$ XID_KEYS=($(envelope xid key all $XID_DOC_UPDATED))

$ envelope format ${XID_KEYS[1]}

PublicKeys(cab108a0) [
    'allow': 'All'
    'name': "Alice"
]

$ envelope format ${XID_KEYS[2]}

PublicKeys(e2c18423) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'name': "Bob"
]
```

#### `xid key find`: Find a Key by the Given Criteria

##### `xid key find public`: Find a Key by the Given Public Key

Returns at most one key envelope.

👉
```
$ envelope xid key find public $BOB_PUBKEYS $XID_DOC_UPDATED | envelope format

PublicKeys(e2c18423) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'name': "Bob"
]
```

##### `xid key find name`: Find a Key by the Given Name

May return multiple key envelopes.

👉
```
$ envelope xid key find name 'Alice' $XID_DOC_UPDATED | envelope format

PublicKeys(cab108a0) [
    'allow': 'All'
    'name': "Alice"
]

$ envelope xid key find name 'Wolf' $XID_DOC_UPDATED

(nothing returned)
```

##### `xid key find inception`: Find the Document's Inception Key

Returns at most one key envelope.

👉
```
$ envelope xid key find inception $XID_DOC_UPDATED | envelope format

PublicKeys(cab108a0) [
    'allow': 'All'
    'name': "Alice"
]
```

#### `xid key remove`: Remove a Given Key

👉
```
$ XID_DOC_REMOVED=`envelope xid key remove $ALICE_PUBKEYS $XID_DOC_UPDATED`
$ envelope format $XID_DOC_REMOVED

XID(93a4d4e7) [
    'key': PublicKeys(e2c18423) [
        'allow': 'Encrypt'
        'allow': 'Sign'
        'name': "Bob"
    ]
]

$ envelope xid key find inception $XID_DOC_REMOVED

(nothing returned)
```

### `xid method`: Work with Resolution Methods

Resolution methods are URIs that describe how to resolve a XID. They are used to find the complete, most up-to-date version of a XID document.

👉
```
$ envelope xid method --help
```

#### `xid method add`: Add a Resolution Method to a XID Document

👉
```
$ XID_DOC=`envelope xid new --name 'Alice' $ALICE_PUBKEYS`
$ XID_DOC_WITH_RESOLVERS=`envelope xid method add 'https://resolver.example.com/' $XID_DOC | \
    envelope xid method add 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6'`
$ envelope format $XID_DOC_WITH_RESOLVERS

XID(93a4d4e7) [
    'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
    'dereferenceVia': URI(https://resolver.example.com/)
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

#### `xid method count`: Count the Number of Resolution Methods in a XID Document

👉
```
$ envelope xid method count $XID_DOC_WITH_RESOLVERS

2
```

#### `xid method at`: Return the Resolution Method at the Specified Index

The indexes are zero-based, and in the order the resolution methods appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

👉
```
$ envelope xid method at 0 $XID_DOC_WITH_RESOLVERS

https://resolver.example.com/

$ envelope xid method at 1 $XID_DOC_WITH_RESOLVERS

btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

#### `xid method all`: List All Resolution Methods in a XID Document

👉
```
$ envelope xid method all $XID_DOC_WITH_RESOLVERS

https://resolver.example.com/
btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

#### `xid method remove`: Remove a Resolution Method from a XID Document

👉
```
$ envelope xid method remove 'https://resolver.example.com/' $XID_DOC_WITH_RESOLVERS | envelope format

XID(93a4d4e7) [
    'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

### `xid delegate`: Work with Delegates

A *delegate* is XID document that is authorized to act on behalf of the *principal* XID document. A delegate can be granted any permissions, but its *effective* permissions will be a subset of the permissions of the principal XID document.

👉
```
$ envelope xid delegate --help
```

#### `xid delegate add`: Add a Delegate to a XID Document

This example:

- creates a XID documents for Alice, Bob, Carol, and Dave,
- grants Carol all permissions on behalf of Alice,
- grants Bob the ability to sign and encrypt on behalf of Alice,
- grants Dave the ability to elide data on behalf of Alice,
    - but only add's Dave's XID identifier to the XID document, which means it will have to be resolved to be used.

👉
```
$ ALICE_PRVKEY_BASE="ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct"
$ ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEY_BASE`
$ BOB_PRVKEY_BASE="ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva"
$ BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEY_BASE`
$ CAROL_PRVKEY_BASE="ur:crypto-prvkey-base/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos"
$ CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEY_BASE`
$ DAVE_PRVKEY_BASE="ur:crypto-prvkey-base/hdcxjtgrwefxlpihpmvtzoprdpfrbaghgmfmdyjsiafzaewlenmktesweocpluwepekgdyutaejy"
$ DAVE_PUBKEYS=`envelope generate pubkeys $DAVE_PRVKEY_BASE`

$ ALICE_XID_DOC=`envelope xid new --name 'Alice' $ALICE_PUBKEYS`
$ BOB_XID_DOC=`envelope xid new --name 'Bob' $BOB_PUBKEYS`
$ CAROL_XID_DOC=`envelope xid new --name 'Carol' $CAROL_PUBKEYS`
$ DAVE_XID_DOC=`envelope xid new --name 'Dave' $DAVE_PUBKEYS`
$ DAVE_XID=`envelope xid id $DAVE_XID_DOC`

$ ALICE_XID_DOC=`envelope xid delegate add --allow 'all' $CAROL_XID_DOC $ALICE_XID_DOC`
$ ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
$ ALICE_XID_DOC=`envelope xid delegate add --allow 'elide' $DAVE_XID $ALICE_XID_DOC`
$ envelope format $ALICE_XID_DOC

XID(93a4d4e7) [
    'delegate': {
        XID(3636003e)
    } [
        'allow': 'Elide'
    ]
    'delegate': {
        XID(61b1f3c7) [
            'key': PublicKeys(eebd4add) [
                'allow': 'All'
                'name': "Carol"
            ]
        ]
    } [
        'allow': 'All'
    ]
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

#### `xid delegate count`: Count the Number of Delegates in a XID Document

👉
```
$ envelope xid delegate count $ALICE_XID_DOC

3
```

#### `xid delegate at`: Return the Delegate at the Specified Index

The indexes are zero-based, and in the order the delegate assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

👉
```
$ envelope xid delegate at 0 $ALICE_XID_DOC | envelope format

{
    XID(f1199a75) [
        'key': PublicKeys(e2c18423) [
            'allow': 'All'
            'name': "Bob"
        ]
    ]
} [
    'allow': 'Encrypt'
    'allow': 'Sign'
]

$ envelope xid delegate at 1 $ALICE_XID_DOC | envelope format

{
    XID(61b1f3c7) [
        'key': PublicKeys(eebd4add) [
            'allow': 'All'
            'name': "Carol"
        ]
    ]
} [
    'allow': 'All'
]

$ envelope xid delegate at 2 $ALICE_XID_DOC | envelope format

{
    XID(3636003e)
} [
    'allow': 'Elide'
]
```

#### `xid delegate all`: List All Delegates in a XID Document

👉
```
$ envelope xid delegate all $ALICE_XID_DOC

ur:envelope/lstpsplftpsotanshdhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloyaylstpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoybdtpsoiafwjlidoycsfncsfgoycsfncsfdoycsfncsgauyzsurla
ur:envelope/lftpsplftpsotanshdhdcxhspawfstecswotwpbsweiowlsrmyfpwpskmeonrtjsrhetsrhnaxfwylvtvsuorkoyaylstpsotansgylftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwoycsfncsfgoybdtpsoihfxhsjpjljzoycsfncsfgzsiddlec
ur:envelope/lftpsptpsotanshdhdcxenenaefmosgecksalokgmnrhgrsemhhfnlfssroxbytkvllrvsrhgtgscpvswfveoycsfncsgegtgtyljt
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

👉
```
$ XID_DELEGATES=($(envelope xid delegate all $ALICE_XID_DOC))
$ envelope format ${XID_DELEGATES[1]}

{
    XID(f1199a75) [
        'key': PublicKeys(e2c18423) [
            'allow': 'All'
            'name': "Bob"
        ]
    ]
} [
    'allow': 'Encrypt'
    'allow': 'Sign'
]

$ envelope format ${XID_DELEGATES[2]}

{
    XID(61b1f3c7) [
        'key': PublicKeys(eebd4add) [
            'allow': 'All'
            'name': "Carol"
        ]
    ]
} [
    'allow': 'All'
]

$ envelope format ${XID_DELEGATES[3]}

{
    XID(3636003e)
} [
    'allow': 'Elide'
]
```

#### `xid delegate find`: Find a Delegate by its XID Identifier

👉
```
$ envelope xid delegate find $DAVE_XID $ALICE_XID_DOC | envelope format

{
    XID(3636003e)
} [
    'allow': 'Elide'
]
```

#### `xid delegate update`: Update an Existing Delegate in an Existing XID Document

- Replaces the existing delegate with the one provided, which must already exist in the XID document.
- Replaces the permissions of the existing delegate with the ones provided.

In this example:
- Carol's XID document is replaced with her bare XID, and
- her permissions are reduced.

👉
```
$ CAROL_XID=`envelope xid id $CAROL_XID_DOC`
$ ALICE_XID_DOC_UPDATED=`envelope xid delegate update --allow 'auth' --allow 'encrypt' --allow 'sign' $CAROL_XID $ALICE_XID_DOC`
$ envelope format $ALICE_XID_DOC_UPDATED

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
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

#### `xid delegate remove`: Remove a Delegate from a XID Document

👉
```
$ BOB_XID=`envelope xid id $BOB_XID_DOC`
$ ALICE_XID_DOC_UPDATED=`envelope xid delegate remove $BOB_XID $ALICE_XID_DOC_UPDATED`
$ envelope format $ALICE_XID_DOC_UPDATED

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
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

### `xid service`: Work with Services

👉
```
$ envelope xid service --help
```

Services are URI endpoints along with the keys, delegates, and permissions that are allowed to use them.

The keys and delegates in a Service declaration are references to keys and delegates that must already exist in the XID document.

👉
```
$ ALICE_PRVKEY_BASE=ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct
$ ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEY_BASE`
$ BOB_PRVKEY_BASE=ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva
$ BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEY_BASE`
$ CAROL_PRVKEY_BASE="ur:crypto-prvkey-base/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos"
$ CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEY_BASE`
```

#### `xid service add`: Add a Service to a XID Document

Alice creates a basic XID document.

👉
```
$ ALICE_XID_DOC=`envelope xid new --name 'Alice' $ALICE_PUBKEYS`
$ envelope format $ALICE_XID_DOC

XID(93a4d4e7) [
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

Alice adds Bob as a delegate.

👉
```
$ BOB_XID_DOC=`envelope xid new --name 'Bob' $BOB_PUBKEYS`
$ ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
$ envelope format $ALICE_XID_DOC

XID(93a4d4e7) [
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
]
```

Alice adds a secure messaging service.

- Alice named the service "Messaging".
- The service is at `https://messaging.example.com`.
- The service provides the capability `com.example.messaging`.
- People can use the service to contact Alice (or Bob acting on behalf of Alice).
- The permissions on the service limit Alice and Bob's public keys to encrypting to Alice, and verifying signatures.
- Alice and Bob, as the holders of the private keys, can decrypt messages sent to them and sign messages they send.

👉
```
$ ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Messaging' \
    --capability 'com.example.messaging' \
    --allow 'sign' \
    --allow 'encrypt' \
    --key $ALICE_PUBKEYS \
    --delegate $BOB_XID_DOC \
    "https://messaging.example.com" \
    $ALICE_XID_DOC`

$ envelope format $ALICE_XID_DOC_WITH_SERVICE

XID(93a4d4e7) [
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
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
```

Alice adds a second service for retrieving her status.

- Alice named the service "Status".
- The service is at `https://status.example.com/alice`.
- The service provides the capability `com.example.status`.
- The public key is the only one used to verify Alice's signatures.
- Alice, as the holder of the private key, can sign her status updates.

👉
```
$ ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Status' \
    --capability 'com.example.status' \
    --allow 'sign' \
    --key $ALICE_PUBKEYS \
    "https://status.example.com/alice" \
    $ALICE_XID_DOC_WITH_SERVICE`

$ envelope format $ALICE_XID_DOC_WITH_SERVICE

XID(93a4d4e7) [
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
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
```

#### `xid service count`: Count the Number of Services in a XID Document

👉
```
$ envelope xid service count $ALICE_XID_DOC_WITH_SERVICE

2
```

#### `xid service at`: Return the Service at the Specified Index

The indexes are zero-based, and in the order the service assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

👉
```
$ envelope xid service at 0 $ALICE_XID_DOC_WITH_SERVICE | envelope format

URI(https://messaging.example.com) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'capability': "com.example.messaging"
    'delegate': Reference(f1199a75)
    'key': Reference(cab108a0)
    'name': "Messaging"
]
```

👉
```
$ envelope xid service at 1 $ALICE_XID_DOC_WITH_SERVICE | envelope format

URI(https://status.example.com/alice) [
    'allow': 'Sign'
    'capability': "com.example.status"
    'key': Reference(cab108a0)
    'name': "Status"
]
```

#### `xid service all`: List All Services in a XID Document

👉
```
$ envelope xid service all $ALICE_XID_DOC_WITH_SERVICE

ur:envelope/lttpsotpcxkscaisjyjyjojkftdldljnihjkjkhsioinjtiodmihkshsjnjojzihdmiajljnoycsfhtpsotanshkhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloybdtpsoingtihjkjkhsioinjtiooycsfxtpsokpiajljndmihkshsjnjojzihdmjnihjkjkhsioinjtiooyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdoycsfncsgagdvamume
ur:envelope/lptpsotpcxkscxisjyjyjojkftdldljkjyhsjykpjkdmihkshsjnjojzihdmiajljndlhsjziniaihoybdtpsoiygujyhsjykpjkoycsfxtpsojpiajljndmihkshsjnjojzihdmjkjyhsjykpjkoyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdglmhuenb
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

👉
```
$ XID_SERVICES=($(envelope xid service all $ALICE_XID_DOC_WITH_SERVICE))
$ envelope format ${XID_SERVICES[1]}

URI(https://messaging.example.com) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'capability': "com.example.messaging"
    'delegate': Reference(f1199a75)
    'key': Reference(cab108a0)
    'name': "Messaging"
]

$ envelope format ${XID_SERVICES[2]}

URI(https://status.example.com/alice) [
    'allow': 'Sign'
    'capability': "com.example.status"
    'key': Reference(cab108a0)
    'name': "Status"
]
```

#### `xid service find`: Find a Service by its URI

##### `xid service find uri`: Find a Service by its URI

Returns at most one service envelope.

👉
```
$ envelope xid service find uri 'https://status.example.com/alice' $ALICE_XID_DOC_WITH_SERVICE | envelope format

URI(https://status.example.com/alice) [
    'allow': 'Sign'
    'capability': "com.example.status"
    'key': Reference(cab108a0)
    'name': "Status"
]
```

##### `xid service find name`: Find a Service by its Name

May return multiple service envelopes.

👉
```
$ envelope xid service find name 'Messaging' $ALICE_XID_DOC_WITH_SERVICE | envelope format

URI(https://messaging.example.com) [
    'allow': 'Encrypt'
    'allow': 'Sign'
    'capability': "com.example.messaging"
    'delegate': Reference(f1199a75)
    'key': Reference(cab108a0)
    'name': "Messaging"
]
```

#### `xid service remove`: Remove a Service from a XID Document

Alice removes the messaging service.

👉
```
$ ALICE_XID_DOC_WITH_SERVICE_REMOVED=`envelope xid service remove 'https://messaging.example.com' $ALICE_XID_DOC_WITH_SERVICE`
$ envelope format $ALICE_XID_DOC_WITH_SERVICE_REMOVED

XID(93a4d4e7) [
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
    'service': URI(https://status.example.com/alice) [
        'allow': 'Sign'
        'capability': "com.example.status"
        'key': Reference(cab108a0)
        'name': "Status"
    ]
]
```

#### `xid service update`: Update an Existing Service in an Existing XID Document

- To remove the name, use `--name ''`.
- To remove the capability, use `--capability ''`.
- Passing one or more `--key` options replaces the existing keys with the ones provided.
- Passing one or more `--delegate` options replaces the existing delegates with the ones provided.
- Passing one or more `--allow` options replaces the existing permissions with the ones provided.

Alice adds Bob as a delegate to the status service. This leaves Alices key and all other attributes of the service unchanged.

👉
```
$ ALICE_XID_DOC_WITH_SERVICE_UPDATED=`envelope xid service update \
    --delegate $BOB_XID_DOC \
    'https://status.example.com/alice' \
    $ALICE_XID_DOC_WITH_SERVICE_REMOVED`

$ envelope format $ALICE_XID_DOC_WITH_SERVICE_UPDATED

XID(93a4d4e7) [
    'delegate': {
        XID(f1199a75) [
            'key': PublicKeys(e2c18423) [
                'allow': 'All'
                'name': "Bob"
            ]
        ]
    } [
        'allow': 'Encrypt'
        'allow': 'Sign'
    ]
    'key': PublicKeys(cab108a0) [
        'allow': 'All'
        'name': "Alice"
    ]
    'service': URI(https://status.example.com/alice) [
        'allow': 'All'
        'capability': "com.example.status"
        'delegate': Reference(f1199a75)
        'key': Reference(cab108a0)
        'name': "Status"
    ]
]
```

Removing a key or delegate from the XID that is referenced by a service is not allowed.

To remove a key or delegate that is referenced by a service, first remove the service.

👉
```
$ envelope xid delegate remove $BOB_XID_DOC $ALICE_XID_DOC_WITH_SERVICE_UPDATED

Error: Delegate is referenced by a service
```

