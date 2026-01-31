# XID Attachments

Attachments provide a standardized way to add discoverable third-party metadata to XID documents. While XID documents are envelopes and can use the general `envelope attachment` commands, the `envelope xid attachment` commands are strongly recommended for working with signed XID documents.

## Why Use XID Attachment Commands?

The XID-specific attachment commands (`envelope xid attachment`) differ from the general attachment commands in one key way: *signature handling*.

| Feature                                       | `envelope attachment` | `envelope xid attachment` |
| --------------------------------------------- | --------------------- | ------------------------- |
| Works with any envelope                       | ✓                     | Only XID documents        |
| Preserves all document content                | ✓                     | ✓                         |
| Signature verification (`--verify inception`) | ✗                     | ✓                         |
| Re-signing (`--sign inception`)               | ✗                     | ✓                         |

Both command sets preserve the complete XID document structure, including private keys and existing assertions. The critical difference is that `xid attachment` commands support verifying and updating signatures when modifying signed documents.

**Use `xid attachment` for:** Signed XID documents (most production use cases)

**Use `envelope attachment` for:** Unsigned XID documents where signature handling isn't needed

The remainder of this section demonstrates the recommended `xid attachment` commands.

## Adding Attachments

Create a XID document and add an attachment with full private key preservation:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxtapslfasrhbtamlslubtmwrdfxettbclaotnfrtowzmydpcyjlgdqzurgrcamsgatansgehdcxhlembaflbsgobydmwfwywfaxyacpatpefmcpbsuoghatsgpfrslshyfmhluymsdakobgenfs
XID_DOC=`envelope xid new $ALICE_PRVKEYS --nickname "Alice"`
PAYLOAD=`envelope subject type string "Alice's contact info"`
XID_WITH_ATTACHMENT=`envelope xid attachment add --vendor "com.example.contacts" --conforms-to "https://example.com/contact-schema/v1" --payload "$PAYLOAD" $XID_DOC`
envelope format $XID_WITH_ATTACHMENT

│ XID(37cf16a3) [
│     'attachment': {
│         "Alice's contact info"
│     } [
│         'conformsTo': "https://example.com/contact-schema/v1"
│         'vendor': "com.example.contacts"
│     ]
│     'key': PublicKeys(d7e77657, SigningPublicKey(37cf16a3, SchnorrPublicKey(145232a9)), EncapsulationPublicKey(4280739f, X25519PublicKey(4280739f))) [
│         {
│             'privateKey': PrivateKeys(677c674f, SigningPrivateKey(dd2814a6, SchnorrPrivateKey(29acb360)), EncapsulationPrivateKey(39e66aca, X25519PrivateKey(39e66aca)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Note that private keys are automatically preserved in the output.

## Working with Signed XID Documents

When working with signed XID documents, use `--verify` and `--sign` to maintain cryptographic integrity:

```
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception`
XID_WITH_ATTACHMENT=`envelope xid attachment add --vendor "com.example.contacts" --conforms-to "https://example.com/contact-schema/v1" --payload "$PAYLOAD" --verify inception --sign inception $SIGNED_XID`
envelope format $XID_WITH_ATTACHMENT

│ {
│     XID(37cf16a3) [
│         'attachment': {
│             "Alice's contact info"
│         } [
│             'conformsTo': "https://example.com/contact-schema/v1"
│             'vendor': "com.example.contacts"
│         ]
│         'key': PublicKeys(d7e77657, SigningPublicKey(37cf16a3, SchnorrPublicKey(145232a9)), EncapsulationPublicKey(4280739f, X25519PublicKey(4280739f))) [
│             {
│                 'privateKey': PrivateKeys(677c674f, SigningPrivateKey(dd2814a6, SchnorrPrivateKey(29acb360)), EncapsulationPrivateKey(39e66aca, X25519PrivateKey(39e66aca)))
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

The workflow:
1. `--verify inception` verifies the existing signature
2. The attachment is added to the document
3. `--sign inception` creates a new signature covering the modified document (including the attachment)
4. Both private keys and the attachment are preserved

## Attachments Persist Across XID Operations

Attachments are automatically preserved when you perform other XID operations:

```
XID_WITH_METHOD=`envelope xid method add https://resolver.example.com --verify inception --sign inception $XID_WITH_ATTACHMENT`
envelope format $XID_WITH_METHOD

│ {
│     XID(37cf16a3) [
│         'attachment': {
│             "Alice's contact info"
│         } [
│             'conformsTo': "https://example.com/contact-schema/v1"
│             'vendor': "com.example.contacts"
│         ]
│         'dereferenceVia': URI(https://resolver.example.com)
│         'key': PublicKeys(d7e77657, SigningPublicKey(37cf16a3, SchnorrPublicKey(145232a9)), EncapsulationPublicKey(4280739f, X25519PublicKey(4280739f))) [
│             {
│                 'privateKey': PrivateKeys(677c674f, SigningPrivateKey(dd2814a6, SchnorrPrivateKey(29acb360)), EncapsulationPrivateKey(39e66aca, X25519PrivateKey(39e66aca)))
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

The attachment, private keys, and signature all remain intact.

## Querying Attachments

Count attachments in a XID document:

```
envelope xid attachment count $XID_WITH_METHOD

│ 1
```

Find attachments by vendor:

```
envelope xid attachment find --vendor "com.example.contacts" $XID_WITH_METHOD | envelope format

│ 'attachment': {
│     "Alice's contact info"
│ } [
│     'conformsTo': "https://example.com/contact-schema/v1"
│     'vendor': "com.example.contacts"
│ ]
```

List all attachments:

```
envelope xid attachment all $XID_WITH_METHOD
```

Get a specific attachment by index:

```
envelope xid attachment at 0 $XID_WITH_METHOD | envelope format
```

## Removing Attachments

Remove an attachment while maintaining signature integrity:

```
ATTACHMENT=`envelope xid attachment find --vendor "com.example.contacts" $XID_WITH_METHOD`
XID_NO_ATTACHMENT=`envelope xid attachment remove $ATTACHMENT --verify inception --sign inception $XID_WITH_METHOD`
envelope xid attachment count $XID_NO_ATTACHMENT

│ 0
```

The `--verify` and `--sign` options ensure secure removal with signature verification and renewal.

## Complete Workflow Example

This example demonstrates a complete workflow maintaining attachments, private keys, and signatures throughout:

```
# Create a signed XID document
SIGNED_XID=`envelope xid new $ALICE_PRVKEYS --nickname "Alice" --sign inception`

# Add an attachment with verification and re-signing
XID_WITH_ATTACHMENT=`envelope xid attachment add --vendor "com.example.contacts" --conforms-to "https://example.com/contact-schema/v1" --payload "$PAYLOAD" --verify inception --sign inception $SIGNED_XID`

# Add a resolution method, preserving attachment and signature
XID_COMPLETE=`envelope xid method add https://resolver.example.com --verify inception --sign inception $XID_WITH_ATTACHMENT`

# Add a service, still preserving everything
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
XID_FINAL=`envelope xid service add https://example.com/api --key "$ALICE_PUBKEYS" --verify inception --sign inception $XID_COMPLETE`

# Verify everything is preserved
envelope xid attachment count $XID_FINAL

│ 1
```

Throughout this workflow:
- ✓ Private keys remain in the document
- ✓ The attachment persists through all operations
- ✓ The signature is verified before each modification
- ✓ A new signature is created after each modification
- ✓ All XID document operations maintain consistency
