# `envelope` - Distributed Identifier Example

This example offers an analogue of a DID document, which identifies an entity. The document itself can be referred to by its ARID, while the signed document can be referred to by its digest.

**See Associated Video:**

**NOTE:** This video shows the command line syntax of the Swift `envelope` command line tool. The Rust-based `nvelope` tool has a slightly different syntax, but the meaning of the commands is the same.

[![Gordian Envelope CLI - 4 - DID Example](https://img.youtube.com/vi/Dvs2CT60_uI/mqdefault.jpg)](https://www.youtube.com/watch?v=Dvs2CT60_uI)


```bash
👉
ALICE_UNSIGNED_DOCUMENT=`nvelope subject type ur $ALICE_ARID | \
    nvelope assertion add pred-obj known controller ur $ALICE_ARID | \
    nvelope assertion add pred-obj known publicKeys ur $ALICE_PUBKEYS`
ALICE_SIGNED_DOCUMENT=`nvelope subject type wrapped $ALICE_UNSIGNED_DOCUMENT | \
    nvelope sign --prvkeys $ALICE_PRVKEYS --note "Made by Alice."`
nvelope format $ALICE_SIGNED_DOCUMENT
```

```
👈
{
    ARID(d44c5e0a) [
        'controller': ARID(d44c5e0a)
        'publicKeys': PublicKeyBase
    ]
} [
    'verifiedBy': Signature [
        'note': "Made by Alice."
    ]
]
```

➡️ ☁️ ➡️

A registrar checks the signature on Alice's submitted identifier document, performs any other necessary validity checks, and then extracts her ARID from it.

```bash
👉
ALICE_UNWRAPPED=`nvelope verify $ALICE_SIGNED_DOCUMENT --pubkeys $ALICE_PUBKEYS | \
    nvelope extract wrapped`
ALICE_ARID_UR=`nvelope extract arid $ALICE_UNWRAPPED`
ALICE_ARID_HEX=`nvelope extract arid-hex $ALICE_UNWRAPPED`
```

The registrar creates its own registration document using Alice's ARID as the subject, incorporating Alice's signed document, and adding its own signature.

```bash
👉
ALICE_URI="https://exampleledger.com/arid/$ALICE_ARID_HEX"
ALICE_REGISTRATION=`nvelope subject type ur $ALICE_ARID_UR | \
    nvelope assertion add pred-obj known entity envelope $ALICE_SIGNED_DOCUMENT | \
    nvelope assertion add pred-obj known dereferenceVia uri $ALICE_URI | \
    nvelope subject type wrapped | \
    nvelope sign --prvkeys $LEDGER_PRVKEYS --note "Made by ExampleLedger."`
nvelope format $ALICE_REGISTRATION
```

```
👈
{
    ARID(d44c5e0a) [
        'dereferenceVia': URI(https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f)
        'entity': {
            ARID(d44c5e0a) [
                'controller': ARID(d44c5e0a)
                'publicKeys': PublicKeyBase
            ]
        } [
            'verifiedBy': Signature [
                'note': "Made by Alice."
            ]
        ]
    ]
} [
    'verifiedBy': Signature [
        'note': "Made by ExampleLedger."
    ]
]
```

Alice receives the registration document back, verifies its signature, and extracts the URI that now points to her record.

```bash
👉
ALICE_URI=`nvelope verify $ALICE_REGISTRATION --pubkeys $LEDGER_PUBKEYS | \
    nvelope extract wrapped | \
    nvelope assertion find predicate known dereferenceVia | \
    nvelope extract object | \
    nvelope extract uri`
echo $ALICE_URI
```

```
👈
https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f
```

Alice wants to introduce herself to Bob, so Bob needs to know she controls her identifier. Bob sends a challenge:

```bash
👉
ALICE_CHALLENGE=`nvelope generate nonce | \
    nvelope subject type ur | \
    nvelope assertion add pred-obj known note string "Challenge to Alice from Bob."`
echo $ALICE_CHALLENGE
```

```
👈
ur:envelope/lftpcstansglgspygwfrjzjpiewlwtinwyhpmkoyaatpcskscefxishsjzjzihjtioihcxjyjlcxfpjziniaihcxiyjpjljncxfwjliddmqdetdsta
```

```bash
👉
nvelope format $ALICE_CHALLENGE
```

```
👈
Nonce [
    note: "Challenge to Alice from Bob."
]
```

Alice responds by adding her registered URI to the nonce, and signing it.

```bash
👉
ALICE_RESPONSE=`nvelope subject type wrapped $ALICE_CHALLENGE | \
    nvelope assertion add pred-obj known dereferenceVia uri $ALICE_URI | \
    nvelope subject type wrapped | \
    nvelope sign --prvkeys $ALICE_PRVKEYS --note "Made by Alice."`
nvelope format $ALICE_RESPONSE
```

```
👈
{
    {
        Nonce [
            'note': "Challenge to Alice from Bob."
        ]
    } [
        'dereferenceVia': URI(https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f)
    ]
} [
    'verifiedBy': Signature [
        'note': "Made by Alice."
    ]
]
```

Bob receives Alice's response, and first checks that the nonce is the once he sent.
```bash
👉
ALICE_CHALLENGE_2=`nvelope extract wrapped $ALICE_RESPONSE | \
    nvelope extract wrapped`
echo $ALICE_CHALLENGE_2
```

```
👈
ur:envelope/lftpcstansglgspygwfrjzjpiewlwtinwyhpmkoyaatpcskscefxishsjzjzihjtioihcxjyjlcxfpjziniaihcxiyjpjljncxfwjliddmqdetdsta
```

`ALICE_CHALLENGE_2` is indeed the same as `ALICE_CHALLENGE`, above. Bob then extracts Alice's registered URI.

```bash
👉
ALICE_URI=`nvelope extract wrapped $ALICE_RESPONSE | \
    nvelope assertion find predicate known dereferenceVia | \
    nvelope extract object | \
    nvelope extract uri`
echo $ALICE_URI
```

```
👈
https://exampleledger.com/arid/d44c5e0afd353f47b02f58a5a3a29d9a2efa6298692f896cd2923268599a0d0f
```

Bob uses the URI to ask ExampleLedger for Alice's identifier document, then checks ExampleLedgers's signature. Bob trusts ExampleLedger's validation of Alice's original document, so doesn't bother to check it for internal consistency, and instead goes ahead and extracts Alice's public keys from it.

```bash
👉
ALICE_PUBKEYS=`nvelope verify $ALICE_REGISTRATION --pubkeys $LEDGER_PUBKEYS | \
    nvelope extract wrapped | \
    nvelope assertion find predicate known entity | \
    nvelope extract object | \
    nvelope extract wrapped | \
    nvelope assertion find predicate known publicKeys | \
    nvelope extract object | \
    nvelope extract ur`
```

Finally, Bob uses Alice's public keys to validate the challenge he sent her.

```bash
👉
nvelope verify --silent $ALICE_RESPONSE --pubkeys $ALICE_PUBKEYS
```
