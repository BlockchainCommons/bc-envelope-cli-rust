# XID Export

The `xid export` command creates a version of a XID document with controlled handling of sensitive data. This is essential for creating publicly distributable versions of XID documents.

## Output Options

All XID-modifying commands support `--private` and `--generator` options that control how sensitive data is handled in the output:

| Option Value        | Private Keys       | Provenance Generator | Structure      |
| ------------------- | ------------------ | -------------------- | -------------- |
| `include` (default) | Plaintext          | Plaintext            | Full           |
| `elide`             | ELIDED placeholder | ELIDED placeholder   | ELIDED markers |
| `omit`              | Removed            | Removed              | Minimal        |
| `encrypt`           | Encrypted          | Encrypted            | Full           |

**When to use each option:**

- **`include`**: Default. For internal use when you need full access to private keys.
- **`elide`**: For public distribution. Preserves the merkle tree and signature without re-signing.
- **`omit`**: Creates a minimal document without secrets or ELIDED markers. Requires re-signing.
- **`encrypt`**: For secure storage. Protects secrets with a password.

## Basic Export Examples

Create a XID document with private keys:

```
XID_DOC=`envelope xid new $ALICE_PRVKEYS --nickname "Alice"`
envelope format $XID_DOC

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Export with private keys elided (creates ELIDED placeholders):

```
envelope xid export --private elide $XID_DOC | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│         ELIDED
│     ]
│ ]
```

Export with private keys omitted (removes them entirely):

```
envelope xid export --private omit $XID_DOC | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

## Exporting Documents with Provenance Marks

When a XID document has a provenance mark, the `--generator` option controls how the generator is handled:

```
XID_WITH_PM=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --generator include`
envelope xid export --private elide --generator elide $XID_WITH_PM | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│         ELIDED
│     ]
│     'provenance': ProvenanceMark(c2e7f91e) [
│         ELIDED
│     ]
│ ]
```

## Complete Export Workflow with Encrypted Secrets

This example demonstrates exporting a signed XID document that has encrypted private keys and an encrypted provenance mark generator.

**Step 1:** Create a signed XID with encrypted secrets:

```
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" \
    --private encrypt --encrypt-password "secret" \
    --generator encrypt \
    --sign inception`
envelope format $SIGNED_XID

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
│         'provenance': ProvenanceMark(983ced7c) [
│             {
│                 'provenanceGenerator': ENCRYPTED [
│                     'hasSecret': EncryptedKey(Argon2id)
│                 ]
│             } [
│                 'salt': Salt
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

**Step 2:** Export with elided secrets (preserves signature—no re-signing needed):

```
envelope xid export --private elide --generator elide $SIGNED_XID | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             'allow': 'All'
│             'nickname': "Alice"
│             ELIDED
│         ]
│         'provenance': ProvenanceMark(...) [
│             ELIDED
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

The ELIDED placeholders show where secrets were hidden. The signature remains valid because elision preserves the merkle tree—the elided nodes' digests are retained.

**Step 3:** Export with omitted secrets (requires re-signing because structure changes):

```
envelope xid export --private omit --generator omit \
    --sign inception --password "secret" $SIGNED_XID | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             'allow': 'All'
│             'nickname': "Alice"
│         ]
│         'provenance': ProvenanceMark(...)
│     ]
│ } [
│     'signed': Signature
│ ]
```

The omitted version has no ELIDED markers and the provenance generator is completely removed—only the provenance mark itself (which is public) is retained. Omitting *always* requires re-signing because the merkle tree is modified.

## Working with Signed Documents

Elision preserves signatures automatically:

```
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception`
envelope xid export --private elide $SIGNED_XID | envelope format

│ {
│     XID(93a4d4e7) [
│         'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│             'allow': 'All'
│             'nickname': "Alice"
│             ELIDED
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

The signature can be verified on the elided document:

```
envelope xid id --verify inception $ELIDED_XID

│ ur:xid/hdcxmuoxtyvddifztyryhymkgolbmefhssmejsgaykcljtjnfmaelrrkvwayehbzfesspmwerowy
```

## Output Options on Other Commands

The `--private` and `--generator` options are available on all XID-modifying commands, not just `export`. This allows you to control output format when performing any operation:

```
# Add a method and elide private keys in one step
envelope xid method add https://example.com --private elide $XID_DOC | envelope format

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(https://example.com)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│         ELIDED
│     ]
│ ]
```
