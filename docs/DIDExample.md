# `envelope` - Distributed Identifier Example

This example offers an analogue of a DID document, which identifies an entity. The document itself can be referred to by its ARID, while the signed document can be referred to by its digest.

**See Associated Video:**

**NOTE:** This video shows the command line syntax of the Swift `envelope` command line tool. The Rust-based `envelope` tool has a slightly different syntax, but the meaning of the commands is the same.

[![Gordian Envelope CLI - 4 - DID Example](https://img.youtube.com/vi/Dvs2CT60_uI/mqdefault.jpg)](https://www.youtube.com/watch?v=Dvs2CT60_uI)


```
ALICE_UNSIGNED_DOCUMENT=`envelope subject type ur $ALICE_ARID | \
    envelope assertion add pred-obj known controller ur $ALICE_ARID | \
    envelope assertion add pred-obj known key ur $ALICE_PUBKEYS`
ALICE_SIGNED_DOCUMENT=`envelope subject type wrapped $ALICE_UNSIGNED_DOCUMENT | \
    envelope sign --signer $ALICE_PRVKEY_BASE --note "Made by Alice."`
envelope format $ALICE_SIGNED_DOCUMENT

│ {
│     ARID(d44c5e0a) [
│         'controller': ARID(d44c5e0a)
│         'key': PublicKeys(cab108a0)
│     ]
│ } [
│     'signed': {
│         Signature [
│             'note': "Made by Alice."
│         ]
│     } [
│         'signed': Signature
│     ]
│ ]
```

➡️ ☁️ ➡️

A registrar checks the signature on Alice's submitted identifier document, performs any other necessary validity checks, and then extracts her ARID from it.

```
ALICE_UNWRAPPED=`envelope verify $ALICE_SIGNED_DOCUMENT --verifier $ALICE_PUBKEYS | \
    envelope extract wrapped`
ALICE_ARID_UR=`envelope extract arid $ALICE_UNWRAPPED`
ALICE_ARID_HEX=`envelope extract arid-hex $ALICE_UNWRAPPED`
```

The registrar creates its own registration document using Alice's ARID as the subject, incorporating Alice's signed document, and adding its own signature.

```
ALICE_URI="https://exampleledger.com/arid/$ALICE_ARID_HEX"
ALICE_REGISTRATION=`envelope subject type ur $ALICE_ARID_UR | \
    envelope assertion add pred-obj known entity envelope $ALICE_SIGNED_DOCUMENT | \
    envelope assertion add pred-obj known dereferenceVia uri $ALICE_URI | \
    envelope subject type wrapped | \
    envelope sign --signer $LEDGER_PRVKEY_BASE --note "Made by ExampleLedger."`
envelope format $ALICE_REGISTRATION

│ {
│     ARID(d44c5e0a) [
│         'dereferenceVia': URI(https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f)
│         'entity': {
│             ARID(d44c5e0a) [
│                 'controller': ARID(d44c5e0a)
│                 'key': PublicKeys(cab108a0)
│             ]
│         } [
│             'signed': {
│                 Signature [
│                     'note': "Made by Alice."
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         ]
│     ]
│ } [
│     'signed': {
│         Signature [
│             'note': "Made by ExampleLedger."
│         ]
│     } [
│         'signed': Signature
│     ]
│ ]
```

Alice receives the registration document back, verifies its signature, and extracts the URI that now points to her record.

```
ALICE_URI=`envelope verify $ALICE_REGISTRATION --verifier $LEDGER_PUBKEYS | \
    envelope extract wrapped | \
    envelope assertion find predicate known dereferenceVia | \
    envelope extract object | \
    envelope extract uri`
echo $ALICE_URI

│ https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f
```

Alice wants to introduce herself to Bob, so Bob needs to know she controls her identifier. Bob sends a challenge:

```
ALICE_CHALLENGE=`envelope generate nonce | \
    envelope subject type ur | \
    envelope assertion add pred-obj known note string "Challenge to Alice from Bob."`
echo $ALICE_CHALLENGE

│ ur:envelope/lftpsotansglgshevewshdtobktdemaslfdyrsoyaatpsokscefxishsjzjzihjtioihcxjyjlcxfpjziniaihcxiyjpjljncxfwjliddmbtcavsrl
```

```
envelope format $ALICE_CHALLENGE

│ Nonce [
│     note: "Challenge to Alice from Bob."
│ ]
```

Alice responds by adding her registered URI to the nonce, and signing it.

```
ALICE_RESPONSE=`envelope subject type wrapped $ALICE_CHALLENGE | \
    envelope assertion add pred-obj known dereferenceVia uri $ALICE_URI | \
    envelope subject type wrapped | \
    envelope sign --signer $ALICE_PRVKEY_BASE --note "Made by Alice."`
envelope format $ALICE_RESPONSE

│ {
│     {
│         Nonce [
│             'note': "Challenge to Alice from Bob."
│         ]
│     } [
│         'dereferenceVia': URI(https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f)
│     ]
│ } [
│     'signed': Signature [
│         'note': "Made by Alice."
│     ]
│ ]
```

Bob receives Alice's response, and first checks that the nonce is the once he sent.
```
ALICE_CHALLENGE_2=`envelope extract wrapped $ALICE_RESPONSE | \
    envelope extract wrapped`
echo $ALICE_CHALLENGE_2

│ ur:envelope/lftpsotansglgshevewshdtobktdemaslfdyrsoyaatpsokscefxishsjzjzihjtioihcxjyjlcxfpjziniaihcxiyjpjljncxfwjliddmbtcavsrl
```

`ALICE_CHALLENGE_2` is indeed the same as `ALICE_CHALLENGE`, above. Bob then extracts Alice's registered URI.

```
ALICE_URI=`envelope extract wrapped $ALICE_RESPONSE | \
    envelope assertion find predicate known dereferenceVia | \
    envelope extract object | \
    envelope extract uri`
echo $ALICE_URI

│ https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f
```

Bob uses the URI to ask ExampleLedger for Alice's identifier document, then checks ExampleLedgers's signature. Bob trusts ExampleLedger's validation of Alice's original document, so doesn't bother to check it for internal consistency, and instead goes ahead and extracts Alice's public keys from it.

```
ALICE_PUBKEYS=`envelope verify $ALICE_REGISTRATION --verifier $LEDGER_PUBKEYS | \
    envelope extract wrapped | \
    envelope assertion find predicate known entity | \
    envelope extract object | \
    envelope extract wrapped | \
    envelope assertion find predicate known key | \
    envelope extract object | \
    envelope extract ur`
```

Finally, Bob uses Alice's public keys to validate the challenge he sent her.

```
envelope verify --silent $ALICE_RESPONSE --verifier $ALICE_PUBKEYS
```
