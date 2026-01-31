# Signed XID Documents

XID documents can be cryptographically signed to ensure their authenticity and integrity. When a XID document is signed, it is wrapped and a signature assertion is added. The signature can be verified to confirm that the document was signed by the holder of the inception key's private key.

## Signing XID Documents

Most XID commands support a `--sign` option that allows you to sign the resulting XID document. The `inception` value signs the document with the XID's inception key.

### Creating a Signed XID Document

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

### Signing with Encrypted Private Keys

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

## Verifying Signed XID Documents

Most XID commands that accept a XID document also support a `--verify` option to verify the signature before processing. The `inception` value verifies that the signature was made with the inception key, which must be contained within the XID document.

### Verifying with `xid id`

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

### Modifying Signed Documents

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

## Signature Options

The `--sign` and `--verify` options accept the following values:

- `none`: (default) Do not sign or verify
- `inception`: Sign with or verify using the XID's inception key

When using `--sign inception`, the inception key must be available in the XID document as a private key. If it is encrypted, the password used to encrypt it is also automatically used for signing.
