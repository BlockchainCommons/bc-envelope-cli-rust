# XID Keys

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

## `xid key add`: Add a New Key to an Existing XID Document

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

## `xid key update`: Update an Existing Key in an Existing XID Document

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

## `xid key count`: Count the Number of Keys in a XID Document

```
envelope xid key count $XID_DOC_UPDATED

│ 2
```

## `xid key at`: Returns the Key at the Specified Index

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

## `xid key all`: Returns All Keys in a XID Document

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

### Retrieving Private Keys with `--private`

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

## `xid key find`: Find a Key by the Given Criteria

### `xid key find public`: Find a Key by the Given Public Key

Returns at most one key envelope.

```
envelope xid key find public $BOB_PUBKEYS $XID_DOC_UPDATED | envelope format

│ PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'nickname': "Bob"
│ ]
```

### `xid key find name`: Find a Key by the Given Name

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

### `xid key find inception`: Find the Document's Inception Key

Returns at most one key envelope.

```
envelope xid key find inception $XID_DOC_UPDATED | envelope format

│ PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│     'allow': 'All'
│     'nickname': "Alice"
│ ]
```

## `xid key remove`: Remove a Given Key

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
