# `envelope` XID Support

The `envelope` tool includes support for working with [XID Documents](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-010-xid.md). This includes creating, updating, and removing keys, resolution methods, delegates, and services. XID documents are a type of envelope that contain public keys, permissions, and other metadata. They are used to represent the identity of a person, device, or service.

**Key Features:**

- Create XID documents from public or private keys
- Encrypt private keys with password protection
- Modify documents (add methods, delegates, services) without decrypting keys
- Commands that don't touch keys don't require passwords
- Transparent preservation of encrypted keys across document modifications

- [`envelope` XID Support](#envelope-xid-support)
  - [Future Work](#future-work)
  - [Import All Envelope URs](#import-all-envelope-urs)
  - [`xid` Subcommand](#xid-subcommand)
    - [`xid id`: Extract the Bare XID from a XID Document](#xid-id-extract-the-bare-xid-from-a-xid-document)
    - [`xid new`: Create New XID Documents From Public or Private Keys](#xid-new-create-new-xid-documents-from-public-or-private-keys)
    - [`xid key`: Work With XID Document Keys](#xid-key-work-with-xid-document-keys)
      - [`xid key add`: Add a New Key to an Existing XID Document](#xid-key-add-add-a-new-key-to-an-existing-xid-document)
      - [`xid key update`: Update an Existing Key in an Existing XID Document](#xid-key-update-update-an-existing-key-in-an-existing-xid-document)
      - [`xid key count`: Count the Number of Keys in a XID Document](#xid-key-count-count-the-number-of-keys-in-a-xid-document)
      - [`xid key at`: Returns the Key at the Specified Index](#xid-key-at-returns-the-key-at-the-specified-index)
      - [`xid key all`: Returns All Keys in a XID Document](#xid-key-all-returns-all-keys-in-a-xid-document)
        - [Retrieving Private Keys with `--private`](#retrieving-private-keys-with---private)
      - [`xid key find`: Find a Key by the Given Criteria](#xid-key-find-find-a-key-by-the-given-criteria)
        - [`xid key find public`: Find a Key by the Given Public Key](#xid-key-find-public-find-a-key-by-the-given-public-key)
        - [`xid key find name`: Find a Key by the Given Name](#xid-key-find-name-find-a-key-by-the-given-name)
        - [`xid key find inception`: Find the Document's Inception Key](#xid-key-find-inception-find-the-documents-inception-key)
      - [`xid key remove`: Remove a Given Key](#xid-key-remove-remove-a-given-key)
    - [`xid method`: Work with Resolution Methods](#xid-method-work-with-resolution-methods)
      - [`xid method add`: Add a Resolution Method to a XID Document](#xid-method-add-add-a-resolution-method-to-a-xid-document)
      - [`xid method count`: Count the Number of Resolution Methods in a XID Document](#xid-method-count-count-the-number-of-resolution-methods-in-a-xid-document)
      - [`xid method at`: Return the Resolution Method at the Specified Index](#xid-method-at-return-the-resolution-method-at-the-specified-index)
      - [`xid method all`: List All Resolution Methods in a XID Document](#xid-method-all-list-all-resolution-methods-in-a-xid-document)
      - [`xid method remove`: Remove a Resolution Method from a XID Document](#xid-method-remove-remove-a-resolution-method-from-a-xid-document)
    - [`xid delegate`: Work with Delegates](#xid-delegate-work-with-delegates)
      - [`xid delegate add`: Add a Delegate to a XID Document](#xid-delegate-add-add-a-delegate-to-a-xid-document)
      - [`xid delegate count`: Count the Number of Delegates in a XID Document](#xid-delegate-count-count-the-number-of-delegates-in-a-xid-document)
      - [`xid delegate at`: Return the Delegate at the Specified Index](#xid-delegate-at-return-the-delegate-at-the-specified-index)
      - [`xid delegate all`: List All Delegates in a XID Document](#xid-delegate-all-list-all-delegates-in-a-xid-document)
      - [`xid delegate find`: Find a Delegate by its XID Identifier](#xid-delegate-find-find-a-delegate-by-its-xid-identifier)
      - [`xid delegate update`: Update an Existing Delegate in an Existing XID Document](#xid-delegate-update-update-an-existing-delegate-in-an-existing-xid-document)
      - [`xid delegate remove`: Remove a Delegate from a XID Document](#xid-delegate-remove-remove-a-delegate-from-a-xid-document)
    - [`xid service`: Work with Services](#xid-service-work-with-services)
      - [`xid service add`: Add a Service to a XID Document](#xid-service-add-add-a-service-to-a-xid-document)
      - [`xid service count`: Count the Number of Services in a XID Document](#xid-service-count-count-the-number-of-services-in-a-xid-document)
      - [`xid service at`: Return the Service at the Specified Index](#xid-service-at-return-the-service-at-the-specified-index)
      - [`xid service all`: List All Services in a XID Document](#xid-service-all-list-all-services-in-a-xid-document)
      - [`xid service find`: Find a Service by its URI](#xid-service-find-find-a-service-by-its-uri)
        - [`xid service find uri`: Find a Service by its URI](#xid-service-find-uri-find-a-service-by-its-uri)
        - [`xid service find name`: Find a Service by its Name](#xid-service-find-name-find-a-service-by-its-name)
      - [`xid service remove`: Remove a Service from a XID Document](#xid-service-remove-remove-a-service-from-a-xid-document)
      - [`xid service update`: Update an Existing Service in an Existing XID Document](#xid-service-update-update-an-existing-service-in-an-existing-xid-document)
  - [Working with Provenance Marks](#working-with-provenance-marks)
    - [`xid provenance get`: Extract the Provenance Mark](#xid-provenance-get-extract-the-provenance-mark)
    - [`xid provenance next`: Advance the Provenance Mark](#xid-provenance-next-advance-the-provenance-mark)
  - [Working with Signed XID Documents](#working-with-signed-xid-documents)

## Import All Envelope URs

Anywhere in `envelope` that accepts a `ur:envelope` can also accept any other UR type, including XID documents.

```
XID_DOC=ur:xid/tpsplftpsotanshdhdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnoyaylftpsotansgylftanshfhdcxhslkfzemaylrwttynsdlghrydpmdfzvdglndloimaahykorefddtsguogmvlahqztansgrhdcxetlewzvlwyfdtobeytidosbamkswaomwwfyabakssakggegychesmerkcatekpcxoycsfncsfggmplgshd
envelope format $XID_DOC

│ XID(71274df1) [
│     'key': PublicKeys(eb9b1cae, SigningPublicKey(71274df1, SchnorrPublicKey(9022010e)), EncapsulationPublicKey(b4f7059a, X25519PublicKey(b4f7059a))) [
│         'allow': 'All'
│     ]
│ ]
```

Note that this does not validate the XID document (or any other envelope-containing UR), it just reads the UR’s envelope, meaning you can manipulate it like any other envelope.

```
envelope assertion at 0 $XID_DOC | \
    envelope format

│ 'key': PublicKeys(eb9b1cae, SigningPublicKey(71274df1, SchnorrPublicKey(9022010e)), EncapsulationPublicKey(b4f7059a, X25519PublicKey(b4f7059a))) [
│     'allow': 'All'
│ ]
```

```
envelope assertion at 0 $XID_DOC | \
    envelope extract object | \
    envelope assertion at 0 | \
    envelope format

│ 'allow': 'All'
```

XID Documents always have the XID CBOR object as their subject. So you can extract the bare XID of a XID document using the `extract xid` subcommand.

```
BARE_XID=`envelope extract xid $XID_DOC`
echo $BARE_XID

│ ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Bare XID URs, although they do not contain an envelope (they are just CBOR) are also internally imported into an empty XID document and then turned into an envelope, with just the XID as its subject.

```
envelope format $BARE_XID

│ XID(71274df1)
```

This means that bare XIDs can be brought in like any other envelope subject. Again, no XID Document-specific validation is done.

```
envelope assertion add pred-obj string "knows" string "Bob" $BARE_XID | envelope format

│ XID(71274df1) [
│     "knows": "Bob"
│ ]
```

## `xid` Subcommand

The `xid` subcommand parses and manipulates XID documents. Invalid XID documents will be rejected. All XID documents returned by its subcommands are in `ur:xid` form.

```
envelope xid --help

│ Work with Extensible Identifiers (XID)
│
│ Usage: envelope xid <COMMAND>
│
│ Commands:
│   new       Create a new XID document from an inception key
│   id        Validate the XID document and return its XID identifier
│   key       Work with a XID document's keys
│   method    Work a XID document's resolution methods
│   delegate  Work with a XID document's delegates
│   service   Work with a XID document's services
│   help      Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

### `xid id`: Extract the Bare XID from a XID Document

Unlike the technique of simply extracting the subject above, this subcommand validates the entire XID document.

```
XID_ID=`envelope xid id $XID_DOC`
echo $XID_ID

│ ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Extracting the bare XID from a bare XID UR is idempotent.

```
envelope xid id $XID_ID

│ ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
```

Several output formats are supported. `ur` is the default and is machine-readable, while the others are human-readable.

```
envelope xid id \
    --format ur \
    --format hex \
    --format bytewords \
    --format bytemoji \
    $XID_DOC

│ ur:xid/hdcxjsdigtwneocmnybadpdlzobysbstmekteypspeotcfldynlpsfolsbintyjkrhfnvsbyrdfw
│ XID(71274df1)
│ 🅧 JUGS DELI GIFT WHEN
│ 🅧 🌊 😹 🌽 🐞
```

### `xid new`: Create New XID Documents From Public or Private Keys

The `xid new` subcommand converts a `PrivateKeyBase` or `PublicKeys` into a XID Document with the provided key as the inception key.

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`

envelope xid new $ALICE_PUBKEYS | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│     ]
│ ]
```

A XID document returned by the `xid new` subcommand is returned as a `ur:xid`.

```
envelope xid new $ALICE_PUBKEYS

│ ur:xid/tpsplftpsotanshdhdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfessoyaylftpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgzckbfwes
```

If a `PrivateKeyBase` is provided, by default the salted private key itself will be included.

```
envelope xid new $ALICE_PRVKEYS | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│ ]
```

The private key can be omitted using the `--private omit` option, or elided using `--private elide`.

```
envelope xid new $ALICE_PRVKEYS --private omit | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│     ]
│ ]
```

```
envelope xid new $ALICE_PRVKEYS --private elide | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         ELIDED
│     ]
│ ]
```

Private keys can be encrypted with a password using `--private encrypt --encrypt-password <PASSWORD>`. This allows you to store and share XID documents with encrypted private keys. The encrypted keys are preserved even when modifying the document (adding resolution methods, delegates, or services) without providing the password.

```
envelope xid new $ALICE_PRVKEYS --private encrypt --encrypt-password "secret" | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│ ]
```

Encrypted private keys are automatically preserved when modifying the document, even without providing the password:

```
XID_ENCRYPTED=`envelope xid new $ALICE_PRVKEYS --private encrypt --encrypt-password "secret"`
envelope xid method add https://resolver.example.com $XID_ENCRYPTED | envelope format

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(https://resolver.example.com)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│ ]
```

Note that the encrypted private key is still present after adding the resolution method. To add a new key to a document with encrypted keys, you must provide the password:

```
XID_WITH_METHOD=`envelope xid new $ALICE_PRVKEYS --private encrypt --encrypt-password "secret" | envelope xid method add https://resolver.example.com`
NEW_KEY=`envelope generate keypairs --signing ed25519 --encryption mlkem512`
envelope xid key add --password "secret" --private encrypt --encrypt-password "secret" --nickname "Backup Key" $NEW_KEY $XID_WITH_METHOD | envelope format

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(https://resolver.example.com)
│     'key': PublicKeys(92338deb, SigningPublicKey(6b4423f4, Ed25519PublicKey(191d34b1)), EncapsulationPublicKey(934ee90f, MLKEM512PublicKey(934ee90f))) [
│         {
│             'privateKey': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│         'nickname': "Backup Key"
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│ ]
```

Commands that only read or modify non-key attributes (like `xid method`, `xid delegate`, and `xid service`) do not require passwords, making it easy to work with documents that have encrypted keys.

One or more endpoint URIs may be added to the inception key.

```
envelope xid new $ALICE_PUBKEYS \
    --endpoint 'https://endpoint.example.com/' \
    --endpoint 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6' \
    | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'endpoint': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
│         'endpoint': URI(https://endpoint.example.com/)
│     ]
│ ]
```

One or more permissions may be specified for the inception key. These replace the default `'All'` permission.

```
envelope xid new $ALICE_PUBKEYS \
    --allow 'encrypt' \
    --allow 'sign' \
    | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│ ]
```

The key may be given a user-assigned name ("nickname") using the `--nickname` option.

```
envelope xid new $ALICE_PUBKEYS \
    --nickname 'Alice'\''s Key' \
    | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice's Key"
│     ]
│ ]
```

### `xid key`: Work With XID Document Keys

```
envelope xid key --help

│ Work with a XID document's keys
│
│ Usage: envelope xid key <COMMAND>
│
│ Commands:
│   add     Add a key to the XID document
│   all     Retrieve all the XID document's keys
│   at      Retrieve the XID Document's key at the given index
│   count   Print the count of the XID document's keys
│   find    Find all XID keys matching the given criteria
│   remove  Remove the given key from the XID document
│   update  Updates the permissions, endpoints, or name of a key in a XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

#### `xid key add`: Add a New Key to an Existing XID Document

All the same options as `xid new` are available. The same key may not be added twice.

```
XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`

BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`

envelope xid key add --nickname 'Bob' $BOB_PUBKEYS $XID_DOC | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│         'allow': 'All'
│         'nickname': "Bob"
│     ]
│ ]
```

#### `xid key update`: Update an Existing Key in an Existing XID Document

All the same options as `xid new` are available. The key must already exist in the XID document.

```
XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS | envelope xid key add --nickname 'Bob' $BOB_PUBKEYS`
envelope format $XID_DOC

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│         'allow': 'All'
│         'nickname': "Bob"
│     ]
│ ]
```

```
XID_DOC_UPDATED=`envelope xid key update $BOB_PUBKEYS \
    --allow 'encrypt' \
    --allow 'sign' \
    $XID_DOC`
envelope format $XID_DOC_UPDATED

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'nickname': "Bob"
│     ]
│ ]
```

#### `xid key count`: Count the Number of Keys in a XID Document

```
envelope xid key count $XID_DOC_UPDATED

│ 2
```

#### `xid key at`: Returns the Key at the Specified Index

The indexes are zero-based, and in the order the key assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid key at 0 $XID_DOC_UPDATED | envelope format

│ PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

```
envelope xid key at 1 $XID_DOC_UPDATED | envelope format

│ PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'nickname': "Bob"
│ ]
```

#### `xid key all`: Returns All Keys in a XID Document

The keys envelopes separated by newlines.

```
envelope xid key all $XID_DOC_UPDATED

│ ur:envelope/lstpsotansgylftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnoycsfncsfgoycscstpsoihfpjziniaihqdkobsbw
│ ur:envelope/lrtpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfdoycsfncsgaoycscstpsoiafwjlidkpjkotey
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

```
XID_KEYS=($(envelope xid key all $XID_DOC_UPDATED))
envelope format ${XID_KEYS[1]}

│ PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

```
envelope format ${XID_KEYS[2]}

│ PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'nickname': "Bob"
│ ]
```

##### Retrieving Private Keys with `--private`

The key retrieval commands (`xid key all`, `xid key at`, `xid key find`) support a `--private` flag that returns the private key portion instead of the public key envelope. The behavior depends on whether the private key is encrypted and whether a password is provided.

**For unencrypted private keys:**

```
PRVKEY=`envelope generate prvkeys`
XID_UNENCRYPTED=`envelope xid new $PRVKEY --nickname "Bob"`
envelope xid key all --private $XID_UNENCRYPTED

│ ur:crypto-prvkeys/lftansgohdcxdlaojztijecphkdicstymeursboxiawllnmhbyynasjtcybdamisesasdmeniysptansgehdcxwdueglfycnynihpmdyimkksrcxenhtkgbaoylazcgalofwbzlfbgghjnvefmetehytaoynyt
```

The `--private` flag returns the raw `ur:crypto-prvkeys` UR, which can be used directly with other `envelope` commands that accept private keys.

**For encrypted private keys without a password:**

```
XID_ENCRYPTED=`envelope xid new $PRVKEY --private encrypt --encrypt-password "secret" --nickname "Alice"`
envelope xid key all --private $XID_ENCRYPTED | envelope format

│ ENCRYPTED [
│     'hasSecret': EncryptedKey(Argon2id)
│ ]
```

Without providing a password, the encrypted envelope is returned as-is. This allows you to verify that a key is encrypted without needing to decrypt it.

**For encrypted private keys with the correct password:**

```
envelope xid key all --private --password "secret" $XID_ENCRYPTED

│ ur:crypto-prvkeys/lftansgohdcxdlaojztijecphkdicstymeursboxiawllnmhbyynasjtcybdamisesasdmeniysptansgehdcxwdueglfycnynihpmdyimkksrcxenhtkgbaoylazcgalofwbzlfbgghjnvefmetehytaoynyt
```

With the correct password, the private key is decrypted and returned as the raw `ur:crypto-prvkeys` UR.

**For encrypted private keys with an incorrect password:**

```
envelope xid key all --private --password "wrong" $XID_ENCRYPTED

│ Error: invalid password
```

Providing an incorrect password results in an error.

**For keys with no private key:**

```
PUBKEYS=`envelope generate prvkeys | envelope generate pubkeys`
XID_NO_PRIVATE=`envelope xid new $PUBKEYS --nickname "Public Only"`
envelope xid key all --private $XID_NO_PRIVATE

│ Error: No private key present in this key
```

When a key was created from `PublicKeys` only (without a private key), attempting to retrieve the private key results in an error.

**The `--private` flag works with all key retrieval commands:**

```
# Get private key at index
envelope xid key at 0 --private --password "secret" $XID_ENCRYPTED

# Find inception key's private key
envelope xid key find inception --private --password "secret" $XID_ENCRYPTED

# Find key by name and get private key
envelope xid key find name Alice --private --password "secret" $XID_ENCRYPTED

# Find key by public key and get private key
envelope xid key find public $PUBKEYS --private --password "secret" $XID_ENCRYPTED
```

**Note:** Without the `--private` flag, key retrieval commands return the complete public key envelope, which includes public keys, metadata (nickname, endpoints, permissions), and the encrypted private key assertion (if present):

```
envelope xid key all $XID_ENCRYPTED | envelope format

│ PublicKeys(074761e6, SigningPublicKey(749b09a9, SchnorrPublicKey(cb62db34)), EncapsulationPublicKey(d9963678, X25519PublicKey(d9963678))) [
│     {
│         'privateKey': ENCRYPTED [
│             'hasSecret': EncryptedKey(Argon2id)
│         ]
│     } [
│         'salt': Salt
│     ]
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

#### `xid key find`: Find a Key by the Given Criteria

##### `xid key find public`: Find a Key by the Given Public Key

Returns at most one key envelope.

```
envelope xid key find public $BOB_PUBKEYS $XID_DOC_UPDATED | envelope format

│ PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'nickname': "Bob"
│ ]
```

##### `xid key find name`: Find a Key by the Given Name

May return multiple key envelopes.

```
envelope xid key find name 'Alice' $XID_DOC_UPDATED | envelope format

│ PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

```
envelope xid key find name 'Wolf' $XID_DOC_UPDATED
```

```
(nothing returned)
```

##### `xid key find inception`: Find the Document's Inception Key

Returns at most one key envelope.

```
envelope xid key find inception $XID_DOC_UPDATED | envelope format

│ PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

#### `xid key remove`: Remove a Given Key

```
XID_DOC_REMOVED=`envelope xid key remove $ALICE_PUBKEYS $XID_DOC_UPDATED`
envelope format $XID_DOC_REMOVED

│ XID(93a4d4e7) [
│     'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'nickname': "Bob"
│     ]
│ ]
```

```
envelope xid key find inception $XID_DOC_REMOVED
```

```
(nothing returned)
```

### `xid method`: Work with Resolution Methods

Resolution methods are URIs that describe how to resolve a XID. They are used to find the complete, most up-to-date version of a XID document.

```
envelope xid method --help

│ Work a XID document's resolution methods
│
│ Usage: envelope xid method <COMMAND>
│
│ Commands:
│   add     Add a resolution method to a XID document
│   all     Retrieve all the XID document's resolution methods
│   at      Retrieve the resolution method at the given index
│   count   Print the count of the XID document's resolution methods
│   remove  Remove the given resolution method from the XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

#### `xid method add`: Add a Resolution Method to a XID Document

```
XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
XID_DOC_WITH_RESOLVERS=`envelope xid method add 'https://resolver.example.com/' $XID_DOC | \
    envelope xid method add 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6'`
envelope format $XID_DOC_WITH_RESOLVERS

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
│     'dereferenceVia': URI(https://resolver.example.com/)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

#### `xid method count`: Count the Number of Resolution Methods in a XID Document

```
envelope xid method count $XID_DOC_WITH_RESOLVERS

│ 2
```

#### `xid method at`: Return the Resolution Method at the Specified Index

The indexes are zero-based, and in the order the resolution methods appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid method at 0 $XID_DOC_WITH_RESOLVERS

│ https://resolver.example.com/
```

```
envelope xid method at 1 $XID_DOC_WITH_RESOLVERS

│ btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

#### `xid method all`: List All Resolution Methods in a XID Document

```
envelope xid method all $XID_DOC_WITH_RESOLVERS
```

```
https://resolver.example.com/
btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

#### `xid method remove`: Remove a Resolution Method from a XID Document

```
envelope xid method remove 'https://resolver.example.com/' $XID_DOC_WITH_RESOLVERS | envelope format

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

### `xid delegate`: Work with Delegates

A *delegate* is XID document that is authorized to act on behalf of the *principal* XID document. A delegate can be granted any permissions, but its *effective* permissions will be a subset of the permissions of the principal XID document.

```
envelope xid delegate --help

│ Work with a XID document's delegates
│
│ Usage: envelope xid delegate <COMMAND>
│
│ Commands:
│   add     Add a delegate to the XID document
│   all     Retrieve all delegates from the XID document
│   at      Retrieve the XID document's delegate at the specified index
│   count   Print the count of the XID document's delegates
│   find    Find a delegate in the XID document
│   remove  Remove a delegate from the XID document
│   update  Update a delegate in the XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

#### `xid delegate add`: Add a Delegate to a XID Document

This example:

- creates a XID documents for Alice, Bob, Carol, and Dave,
- grants Carol all permissions on behalf of Alice,
- grants Bob the ability to sign and encrypt on behalf of Alice,
- grants Dave the ability to elide data on behalf of Alice,
    - but only add's Dave's XID identifier to the XID document, which means it will have to be resolved to be used.

```
ALICE_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls"
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
BOB_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk"
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`
CAROL_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxmorsytadihzswmckyltauyolecmevychhlwmtylbhsmdptfdrtuewnjtdkmnmkretansgehdcxhentsejphsfwclylihbwroaoisptaskegrimyldebecsdrrtbdlrrslazeursspmldtkmdds"
CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEYS`
DAVE_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxsbqzasvdrpmuhegoaelekbwznnlfskkpyadrfhsncxlrmkihrecskpvapactresotansgehdcxflaxjtaskssogemtdpioaehpdytbtedyrtclkoceckbbadtlhlhljtensnylatvokkztwdny"
DAVE_PUBKEYS=`envelope generate pubkeys $DAVE_PRVKEYS`

ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`
CAROL_XID_DOC=`envelope xid new --nickname 'Carol' $CAROL_PUBKEYS`
DAVE_XID_DOC=`envelope xid new --nickname 'Dave' $DAVE_PUBKEYS`
DAVE_XID=`envelope xid id $DAVE_XID_DOC`

ALICE_XID_DOC=`envelope xid delegate add --allow 'all' $CAROL_XID_DOC $ALICE_XID_DOC`
ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
ALICE_XID_DOC=`envelope xid delegate add --allow 'elide' $DAVE_XID $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7) [
│             'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│                 'allow': 'All'
│                 'nickname': "Carol"
│             ]
│         ]
│     } [
│         'allow': 'All'
│     ]
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

#### `xid delegate count`: Count the Number of Delegates in a XID Document

```
envelope xid delegate count $ALICE_XID_DOC

│ 3
```

#### `xid delegate at`: Return the Delegate at the Specified Index

The indexes are zero-based, and in the order the delegate assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid delegate at 0 $ALICE_XID_DOC | envelope format

│ {
│     XID(61b1f3c7) [
│         'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│             'allow': 'All'
│             'nickname': "Carol"
│         ]
│     ]
│ } [
│     'allow': 'All'
│ ]
```

```
envelope xid delegate at 1 $ALICE_XID_DOC | envelope format

│ {
│     XID(f1199a75) [
│         'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│             'allow': 'All'
│             'nickname': "Bob"
│         ]
│     ]
│ } [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│ ]
```

```
envelope xid delegate at 2 $ALICE_XID_DOC | envelope format

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]
```

#### `xid delegate all`: List All Delegates in a XID Document

```
envelope xid delegate all $ALICE_XID_DOC

│ ur:envelope/lftpsplftpsotanshdhdcxhspawfstecswotwpbsweiowlsrmyfpwpskmeonrtjsrhetsrhnaxfwylvtvsuorkoyaylstpsotansgylftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwoycsfncsfgoycscstpsoihfxhsjpjljzoycsfncsfgknhpttwe
│ ur:envelope/lstpsplftpsotanshdhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloyaylstpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfgoycscstpsoiafwjlidoycsfncsfdoycsfncsgawnftoeoy
│ ur:envelope/lftpsptpsotanshdhdcxenenaefmosgecksalokgmnrhgrsemhhfnlfssroxbytkvllrvsrhgtgscpvswfveoycsfncsgegtgtyljt
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

```
XID_DELEGATES=($(envelope xid delegate all $ALICE_XID_DOC))
envelope format ${XID_DELEGATES[1]}

│ {
│     XID(61b1f3c7) [
│         'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│             'allow': 'All'
│             'nickname': "Carol"
│         ]
│     ]
│ } [
│     'allow': 'All'
│ ]
```

```
envelope format ${XID_DELEGATES[2]}

│ {
│     XID(f1199a75) [
│         'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│             'allow': 'All'
│             'nickname': "Bob"
│         ]
│     ]
│ } [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│ ]
```

```
envelope format ${XID_DELEGATES[3]}
```

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]

#### `xid delegate find`: Find a Delegate by its XID Identifier

```
envelope xid delegate find $DAVE_XID $ALICE_XID_DOC | envelope format

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]
```

#### `xid delegate update`: Update an Existing Delegate in an Existing XID Document

- Replaces the existing delegate with the one provided, which must already exist in the XID document.
- Replaces the permissions of the existing delegate with the ones provided.

In this example:
- Carol's XID document is replaced with her bare XID, and
- her permissions are reduced.

```
CAROL_XID=`envelope xid id $CAROL_XID_DOC`
ALICE_XID_DOC_UPDATED=`envelope xid delegate update --allow 'auth' --allow 'encrypt' --allow 'sign' $CAROL_XID $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7)
│     } [
│         'allow': 'Auth'
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

#### `xid delegate remove`: Remove a Delegate from a XID Document

```
BOB_XID=`envelope xid id $BOB_XID_DOC`
ALICE_XID_DOC_UPDATED=`envelope xid delegate remove $BOB_XID $ALICE_XID_DOC_UPDATED`
envelope format $ALICE_XID_DOC_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7)
│     } [
│         'allow': 'Auth'
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

### `xid service`: Work with Services

```
envelope xid service --help

│ Work with a XID document's services
│
│ Usage: envelope xid service <COMMAND>
│
│ Commands:
│   add     Add a service to the XID document
│   all     Retrieve all the XID services
│   at      Retrieve the XID Document's service at the given index
│   count   Print the count of the XID document's services
│   find    Find all XID services matching the given criteria
│   remove  Remove the given service from the XID document
│   update  Updates the permissions, delegates, keys, capability identifer, or name of a service in a XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

Services are URI endpoints along with the keys, delegates, and permissions that are allowed to use them.

The keys and delegates in a Service declaration are references to keys and delegates that must already exist in the XID document.

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`
CAROL_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxmorsytadihzswmckyltauyolecmevychhlwmtylbhsmdptfdrtuewnjtdkmnmkretansgehdcxhentsejphsfwclylihbwroaoisptaskegrimyldebecsdrrtbdlrrslazeursspmldtkmdds"
CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEYS`
```

#### `xid service add`: Add a Service to a XID Document

Alice creates a basic XID document.

```
ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Alice adds Bob as a delegate.

```
BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`
ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Alice adds a secure messaging service.

- Alice named the service "Messaging".
- The service is at `https://messaging.example.com`.
- The service provides the capability `com.example.messaging`.
- People can use the service to contact Alice (or Bob acting on behalf of Alice).
- The permissions on the service limit Alice and Bob's public keys to encrypting to Alice, and verifying signatures.
- Alice and Bob, as the holders of the private keys, can decrypt messages sent to them and sign messages they send.

```
ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Messaging' \
    --capability 'com.example.messaging' \
    --allow 'sign' \
    --allow 'encrypt' \
    --key $ALICE_PUBKEYS \
    --delegate $BOB_XID_DOC \
    "https://messaging.example.com" \
    $ALICE_XID_DOC`

envelope format $ALICE_XID_DOC_WITH_SERVICE

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://messaging.example.com) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'capability': "com.example.messaging"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Messaging"
│     ]
│ ]
```

Alice adds a second service for retrieving her status.

- Alice named the service "Status".
- The service is at `https://status.example.com/alice`.
- The service provides the capability `com.example.status`.
- The public key is the only one used to verify Alice's signatures.
- Alice, as the holder of the private key, can sign her status updates.

```
ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Status' \
    --capability 'com.example.status' \
    --allow 'sign' \
    --key $ALICE_PUBKEYS \
    "https://status.example.com/alice" \
    $ALICE_XID_DOC_WITH_SERVICE`

envelope format $ALICE_XID_DOC_WITH_SERVICE

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://messaging.example.com) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'capability': "com.example.messaging"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Messaging"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'Sign'
│         'capability': "com.example.status"
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

#### `xid service count`: Count the Number of Services in a XID Document

```
envelope xid service count $ALICE_XID_DOC_WITH_SERVICE
```

```
2
```

#### `xid service at`: Return the Service at the Specified Index

The indexes are zero-based, and in the order the service assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid service at 0 $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

```
envelope xid service at 1 $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

#### `xid service all`: List All Services in a XID Document

```
envelope xid service all $ALICE_XID_DOC_WITH_SERVICE

│ ur:envelope/lttpsotpcxkscaisjyjyjojkftdldljnihjkjkhsioinjtiodmihkshsjnjojzihdmiajljnoycsfhtpsotanshkhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloybdtpsoingtihjkjkhsioinjtiooycsfxtpsokpiajljndmihkshsjnjojzihdmjnihjkjkhsioinjtiooyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdoycsfncsgagdvamume
│ ur:envelope/lptpsotpcxkscxisjyjyjojkftdldljkjyhsjykpjkdmihkshsjnjojzihdmiajljndlhsjziniaihoybdtpsoiygujyhsjykpjkoycsfxtpsojpiajljndmihkshsjnjojzihdmjkjyhsjykpjkoyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdglmhuenb
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

```
XID_SERVICES=($(envelope xid service all $ALICE_XID_DOC_WITH_SERVICE))
envelope format ${XID_SERVICES[1]}

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

```
envelope format ${XID_SERVICES[2]}

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

#### `xid service find`: Find a Service by its URI

##### `xid service find uri`: Find a Service by its URI

Returns at most one service envelope.

```
envelope xid service find uri 'https://status.example.com/alice' $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

##### `xid service find name`: Find a Service by its Name

May return multiple service envelopes.

```
envelope xid service find name 'Messaging' $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

#### `xid service remove`: Remove a Service from a XID Document

Alice removes the messaging service.

```
ALICE_XID_DOC_WITH_SERVICE_REMOVED=`envelope xid service remove 'https://messaging.example.com' $ALICE_XID_DOC_WITH_SERVICE`
envelope format $ALICE_XID_DOC_WITH_SERVICE_REMOVED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'Sign'
│         'capability': "com.example.status"
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

#### `xid service update`: Update an Existing Service in an Existing XID Document

- To remove the name, use `--name ''`.
- To remove the capability, use `--capability ''`.
- Passing one or more `--key` options replaces the existing keys with the ones provided.
- Passing one or more `--delegate` options replaces the existing delegates with the ones provided.
- Passing one or more `--allow` options replaces the existing permissions with the ones provided.

Alice adds Bob as a delegate to the status service. This leaves Alices key and all other attributes of the service unchanged.

```
ALICE_XID_DOC_WITH_SERVICE_UPDATED=`envelope xid service update \
    --delegate $BOB_XID_DOC \
    'https://status.example.com/alice' \
    $ALICE_XID_DOC_WITH_SERVICE_REMOVED`

envelope format $ALICE_XID_DOC_WITH_SERVICE_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'All'
│         'capability': "com.example.status"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

Removing a key or delegate from the XID that is referenced by a service is not allowed.

To remove a key or delegate that is referenced by a service, first remove the service.

```
envelope xid delegate remove $BOB_XID_DOC $ALICE_XID_DOC_WITH_SERVICE_UPDATED

│ Error: item is still referenced: delegate
```

## Working with Provenance Marks

XID documents can include provenance marks that provide a verifiable chain of custody and state transitions. A provenance mark is a cryptographic proof that shows the document's history, using a hash-based chain structure. Each mark contains:

- A sequence number (starting from 0 for the genesis mark)
- A date timestamp
- Optional structured information (as any UR type)
- A chain code linking to previous marks

Provenance marks are particularly useful for tracking XID document updates over time, ensuring that modifications follow a verifiable sequence.

### Creating XID Documents with Provenance Marks

When creating a new XID document, you can include a genesis provenance mark using the `--generator` option:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --generator include | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(53929e33) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 1
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--generator include` option includes the provenance generator in the document. This generator contains the cryptographic state needed to create subsequent marks in the chain. The generator is included as a salted assertion on the provenance mark itself.

#### Generator Options

The `--generator` option accepts several values:

- `omit` (default): Do not include a provenance mark
- `include`: Include a provenance mark with the generator in plaintext
- `encrypt`: Include a provenance mark with an encrypted generator (requires `--encrypt-password`)
- `elide`: Not supported for new documents (will produce an error)

#### Custom Dates and Information

When creating a genesis mark, you can specify a custom date and attach structured information:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
DIGEST_UR=`envelope generate digest "Hello"`
envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15 --info $DIGEST_UR | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(2bbbd1e1) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 1
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--date` option accepts ISO 8601 format dates (e.g., "2025-01-15"). If omitted, the current date is used.

The `--info` option accepts any UR type (envelope, digest, ARID, etc.). This information is embedded in the provenance mark and can be used to attach context or metadata to the mark.

#### Encrypted Generators

For additional security, the provenance generator can be encrypted:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --generator encrypt --encrypt-password "secret" | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(10bd3a28) [
│         {
│             'provenanceGenerator': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Encrypted generators protect the cryptographic state while still allowing the provenance mark itself to be read. To advance the mark later, you'll need to provide the decryption password.

### `xid provenance get`: Extract the Provenance Mark

The `xid provenance get` command extracts the provenance mark from a XID document and returns it as a standalone UR:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include`
envelope xid provenance get $XID_WITH_PROV

│ ur:provenance/lfaxhdimpkwlsektpsataagwutlpdspmbtjkfrprmoptdtlrftbwdkdlvalrytchlrtdsavtplvsltsgbdjlwyhfvawejlvtutfmondavarnylstbncplymtatmkjkpylrclpkrlaoisgoinmkpyssmydlotfrmhdszeftmwgsluclrerlnlwtemcholrlrojpiyhksplsldztspihlkbentgwfsdnmomtax
```

If the document does not have a provenance mark, the command returns an empty string:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_NO_PROV=`envelope xid new $ALICE_PRVKEYS`
envelope xid provenance get $XID_NO_PROV

│
```

This command works with both plaintext and encrypted generators. It also supports signature verification with the `--verify` option (see [Working with Signed XID Documents](#working-with-signed-xid-documents)).

### `xid provenance next`: Advance the Provenance Mark

The `xid provenance next` command advances the provenance mark to the next state in the chain. This creates a new mark with an incremented sequence number and a hash linking to the previous mark:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 $XID_WITH_PROV | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(bf27fca8) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Notice that the provenance mark's identifier has changed (from `53929e33` to `bf27fca8` in this example), and the `next-seq` value in the generator has incremented from 1 to 2.

#### Using an Embedded Generator

When the XID document contains an embedded generator (either plaintext or encrypted), the `next` command uses it automatically:

- **Plaintext generators**: No password needed
- **Encrypted generators**: Require `--password` to decrypt

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_ENC_GEN=`envelope xid new $ALICE_PRVKEYS --generator encrypt --encrypt-password "secret" --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 --password "secret" --encrypt-password "secret" $XID_ENC_GEN | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(8205b945) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Note: When advancing with an encrypted generator, the `--password` option decrypts it for use, and `--encrypt-password` re-encrypts it in the output. If you want to change from encrypted to plaintext, you can omit `--encrypt-password`.

#### Attaching Information to New Marks

Like the genesis mark, you can attach custom dates and structured information when advancing:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ARID_UR=`envelope generate arid`
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 --info $ARID_UR $XID_WITH_PROV | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(79a0d8f3) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--info` parameter can be any UR type, making it flexible for various use cases such as attaching transaction references, update notes, or other contextual data.

## Working with Signed XID Documents

XID documents can be cryptographically signed to ensure their authenticity and integrity. When a XID document is signed, it is wrapped and a signature assertion is added. The signature can be verified to confirm that the document was signed by the holder of the inception key's private key.

### Signing XID Documents

Most XID commands support a `--sign` option that allows you to sign the resulting XID document. The `inception` value signs the document with the XID's inception key.

#### Creating a Signed XID Document

When creating a new XID document from a private key, you can sign it immediately:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             {
│                 'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│             } [
│                 'salt': Salt
│             ]
│             'allow': 'All'
│             'nickname': "Alice"
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

Note that the signed XID document has been wrapped (indicated by the outer `{ }` braces), with the signature appearing as a `'signed': Signature` assertion on the wrapped envelope.

#### Signing with Encrypted Private Keys

When using encrypted private keys, the encryption password is automatically used for signing:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --nickname "Alice" --private encrypt --encrypt-password "secret" --sign inception | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             {
│                 'privateKey': ENCRYPTED [
│                     'hasSecret': EncryptedKey(Argon2id)
│                 ]
│             } [
│                 'salt': Salt
│             ]
│             'allow': 'All'
│             'nickname': "Alice"
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

### Verifying Signed XID Documents

Most XID commands that accept a XID document also support a `--verify` option to verify the signature before processing. The `inception` value verifies that the signature was made with the inception key, which must be contained within the XID document.

#### Verifying with `xid id`

The `xid id` command can verify a signature when extracting the XID identifier:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception`
envelope xid id --verify inception $SIGNED_XID

│ ur:xid/hdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfesspmwerowy
```

If the XID document is not signed, verification fails:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
UNSIGNED_XID=`envelope xid new $ALICE_PUBKEYS --nickname "Alice"`
envelope xid id --verify inception $UNSIGNED_XID

│ Error: envelope is not signed
```

#### Modifying Signed Documents

When modifying a signed XID document, you should verify the existing signature and re-sign after modification:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception`
envelope xid key add --nickname "Bob" $BOB_PUBKEYS $SIGNED_XID --verify inception --sign inception | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             {
│                 'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│             } [
│                 'salt': Salt
│             ]
│             'allow': 'All'
│             'nickname': "Alice"
│         ]
│         'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│             'allow': 'All'
│             'nickname': "Bob"
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

In this example:
- `--verify inception` checks that the incoming document is properly signed
- The operation is performed (adding Bob's key)
- `--sign inception` creates a new signature for the modified document

### Signature Options

The `--sign` and `--verify` options accept the following values:

- `none`: (default) Do not sign or verify
- `inception`: Sign with or verify using the XID's inception key

When using `--sign inception`, the inception key must be available in the XID document as a private key. If it is encrypted, the password used to encrypt it is also automatically used for signing.
