# XID Basics

## `xid` Subcommand

The `xid` subcommand parses and manipulates XID documents. Invalid XID documents will be rejected. All XID documents returned by its subcommands are in `ur:xid` form.

```
envelope xid --help

│ Work with Extensible Identifiers (XID)
│
│ Usage: envelope xid <COMMAND>
│
│ Commands:
│   new         Create a new XID document from an inception key
│   export      Export a XID document with specified output options
│   provenance  Work with provenance marks
│   id          Validate the XID document and return its XID identifier
│   key         Work with a XID document's keys
│   method      Work a XID document's resolution methods
│   delegate    Work with a XID document's delegates
│   service     Work with a XID document's services
│   attachment  Work with a XID document's attachments
│   help        Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

## `xid id`: Extract the Bare XID from a XID Document

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

## `xid new`: Create New XID Documents From Public or Private Keys

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
