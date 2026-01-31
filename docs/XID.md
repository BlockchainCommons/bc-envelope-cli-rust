# `envelope` XID Support

The `envelope` tool includes support for working with [XID Documents](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-010-xid.md). This includes creating, updating, and removing keys, resolution methods, delegates, and services. XID documents are a type of envelope that contain public keys, permissions, and other metadata. They are used to represent the identity of a person, device, or service.

**Key Features:**

- Create XID documents from public or private keys
- Encrypt private keys with password protection
- Modify documents (add methods, delegates, services) without decrypting keys
- Commands that don't touch keys don't require passwords
- Transparent preservation of encrypted keys across document modifications

## Chapters

1. [XID Basics](XID-Basics.md) — Creating and identifying XID documents (`xid id`, `xid new`)
2. [XID Export](XID-Export.md) — Exporting with controlled private key and generator handling
3. [XID Keys](XID-Keys.md) — Working with XID document keys
4. [XID Methods](XID-Methods.md) — Resolution methods for XID documents
5. [XID Delegates](XID-Delegates.md) — Working with delegates
6. [XID Services](XID-Services.md) — Working with services
7. [XID Provenance](XID-Provenance.md) — Provenance mark chains for document history
8. [XID Signing](XID-Signing.md) — Signing and verifying XID documents
9. [XID Attachments](XID-Attachments.md) — Standardized third-party metadata
10. [XID Edges](XID-Edges.md) — Verifiable claims connecting XID entities

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

Note that this does not validate the XID document (or any other envelope-containing UR), it just reads the UR's envelope, meaning you can manipulate it like any other envelope.

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
