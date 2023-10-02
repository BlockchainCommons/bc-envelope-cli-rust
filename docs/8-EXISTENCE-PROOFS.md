# envelope - Existence Proofs

**NOTE:** Most of this documentation has *not* been updated to reflect the new command line syntax. In particular, this document describes functionality that is not yet supported.

**See Associated Video:**

**NOTE:** This video shows the command line syntax of the Swift `envelope` command line tool. The Rust-based `nvelope` tool has a slightly different syntax, but the meaning of the commands is the same.

[![Gordian Envelope CLI - 5 - Existence Proofs](https://img.youtube.com/vi/LUQ-n9EZa0U/mqdefault.jpg)](https://www.youtube.com/watch?v=LUQ-n9EZa0U)

## Introduction

An *existence proof* is a method of showing that particular information exists in a document without revealing more than is necessary about the document in which it exists.

In a previous chapter we discussed elision, which is a method whereby information can be removed from an envelope without changing its digest tree structure.

Because each element of an envelope provides a unique digest, and because changing an element in an envelope changes the digest of all elements upwards towards its root, the structure of an envelope is comparable to a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree).

In a Merkle Tree, all semantically significant information is carried by the tree's leaves (for example, the transactions in a block of Bitcoin transactions) while the internal nodes of the tree are nothing but digests computed from combinations of pairs of lower nodes, all the way up to the root of the tree (the "Merkle root".)

In an envelope, every digest points at some potentially useful semantic information: at the subject of the envelope, at one of the assertions in the envelope, or at the predicate or object of a given assertion. Of course, those object are all envelopes themselves, so by using salt, the digest of the element can be decorrelated from its content.

In a merkle tree, the minumum subset of hashes necessary to confirm that a specific leaf node (the "target") must be present is called a "Merkle proof." For envelopes, an analogous proof would be a transformation of the envelope that is entirely elided but preserves the structure necesssary to reveal the target.

## Example 1: Alice's Friends

This document contains a list of people Alice knows. Each "knows" assertion has been salted so if the assertions have been elided one can't merely guess at who she knows by pairing the "knows" predicate with the names of possibly-known associates and comparing the resulting digests to the elided digests in the document.

```bash
👉
ALICE_FRIENDS=`envelope subject Alice |
    envelope assertion knows Bob --salted |
    envelope assertion knows Carol --salted |
    envelope assertion knows Dan --salted`
envelope $ALICE_FRIENDS
```

```
👈
"Alice" [
    {
        "knows": "Bob"
    } [
        salt: Salt
    ]
    {
        "knows": "Carol"
    } [
        salt: Salt
    ]
    {
        "knows": "Dan"
    } [
        salt: Salt
    ]
]
```

Alice provides just the root digest of her document to a third party. This is simply an envelope in which everything has been elided and nothing revealed.

```bash
👉
ALICE_FRIENDS_ROOT=`envelope elide revealing $ALICE_FRIENDS`
envelope $ALICE_FRIENDS_ROOT
```

```
👈
ELIDED
```

Now Alice wants to prove to the third party that her document contains a "knows Bob" assertion. To do this, she produces a proof that is an envelope with the minimal structure of digests included so that the proof envelope has the same digest as the completely elided envelope, but also exposes the digest of the target of the proof.

Note that in the proof the digests of the two other elided "knows" assertions are present, but because they have been salted, the third party cannot easily guess who else she knows.

```bash
👉
KNOWS_BOB_DIGEST=`envelope subject assertion knows Bob | envelope digest`
KNOWS_BOB_PROOF=`envelope proof create $ALICE_FRIENDS $KNOWS_BOB_DIGEST`
envelope $KNOWS_BOB_PROOF
```

```
👈
ELIDED [
    ELIDED [
        ELIDED
    ]
    ELIDED (2)
]
```

The third party then uses the previously known and trusted root to confirm that the envelope does indeed contain a "knows bob" assertion.

```bash
👉
envelope proof confirm --silent $ALICE_FRIENDS_ROOT $KNOWS_BOB_PROOF $KNOWS_BOB_DIGEST
```

There is no output because the proof succeeded.

## Example 2: Verifiable Credential

A verifiable credential is constructed such that elements that might be elided are also salted, making correlation between digest and message much more difficult. Other assertions like `.issuer` and `.controller` are left unsalted.


```bash
👉
BOARD_PRVKEYS="ur:crypto-prvkeys/hdcxynlntpsbfrbgjkcetpzorohgsafsihcnhyrtoebzwegtvyzolbgtdaskcsldfgadtldmrkld"
CREDENTIAL=`envelope subject --arid 4676635a6e6068c2ef3ffd8ff726dd401fd341036e920f136a1d8af5e829496d |
    envelope assertion --known isA "Certificate of Completion" |
    envelope assertion --known issuer "Example Electrical Engineering Board" |
    envelope assertion --known controller "Example Electrical Engineering Board" |
    envelope assertion --salted firstName James |
    envelope assertion --salted lastName Maxwell |
    envelope assertion --salted --string issueDate --date 2020-01-01 |
    envelope assertion --salted --string expirationDate --date 2028-01-01 |
    envelope assertion --salted photo "This is James Maxwell's photo." |
    envelope assertion --salted certificateNumber 123-456-789 |
    envelope assertion --salted subject "RF and Microwave Engineering" |
    envelope assertion --string continuingEducationUnits --number 1.5 |
    envelope assertion --string professionalDevelopmentHours --number 15 |
    envelope assertion --string topics --cbor 0x82695375626a6563742031695375626a6563742032 |
    envelope subject --wrapped |
    envelope sign --prvkeys $BOARD_PRVKEYS |
    envelope assertion --known note "Signed by Example Electrical Engineering Board"`
envelope $CREDENTIAL
```

```
👈
{
    ARID(4676635a) [
        isA: "Certificate of Completion"
        {
            "certificateNumber": "123-456-789"
        } [
            salt: Salt
        ]
        {
            "expirationDate": 2028-01-01
        } [
            salt: Salt
        ]
        {
            "firstName": "James"
        } [
            salt: Salt
        ]
        {
            "issueDate": 2020-01-01
        } [
            salt: Salt
        ]
        {
            "lastName": "Maxwell"
        } [
            salt: Salt
        ]
        {
            "photo": "This is James Maxwell's photo."
        } [
            salt: Salt
        ]
        {
            "subject": "RF and Microwave Engineering"
        } [
            salt: Salt
        ]
        "continuingEducationUnits": 1.5
        "professionalDevelopmentHours": 15
        "topics": ["Subject 1", "Subject 2"]
        controller: "Example Electrical Engineering Board"
        issuer: "Example Electrical Engineering Board"
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

```bash
👉
CREDENTIAL_ROOT=`envelope elide revealing $CREDENTIAL`
envelope $CREDENTIAL_ROOT
```

```
👈
ELIDED
```

In this case the holder of a credential wants to prove a single assertion from it: the subject.

```bash
👉
SUBJECT_DIGEST=`envelope subject assertion "subject" "RF and Microwave Engineering" | envelope digest`
SUBJECT_PROOF=`envelope proof create $CREDENTIAL $SUBJECT_DIGEST`
envelope $SUBJECT_PROOF
```

The proof includes digests from all the elided assertions.

```
👈
{
    ELIDED [
        ELIDED [
            ELIDED
        ]
        ELIDED (12)
    ]
} [
    ELIDED (2)
]
```

The proof confirms the subject, as intended.

```bash
👉
envelope proof confirm --silent $CREDENTIAL_ROOT $SUBJECT_PROOF $SUBJECT_DIGEST
```

Assertions without salt can be guessed at, and confirmed if the the guess is correct.

```bash
👉
ISSUER_DIGEST=`envelope subject assertion --known issuer --string "Example Electrical Engineering Board" | envelope digest`
envelope proof confirm --silent $CREDENTIAL_ROOT $SUBJECT_PROOF $ISSUER_DIGEST
```

The proof cannot be used to confirm salted assertions.

```bash
👉
FIRST_NAME_DIGEST=`envelope subject assertion firstName James | envelope digest`
envelope proof confirm --silent $CREDENTIAL_ROOT $SUBJECT_PROOF $FIRST_NAME_DIGEST
```

```
👈
Error: invalidProof
```

## Example 3: Multiproofs

A single proof can be generated to reveal multiple target digests. In this example we prove the holder's `firstName` and `lastName` using a single proof, even though they are in different fields.

```bash
👉
FIRST_NAME_DIGEST=`envelope subject assertion firstName James | envelope digest`
LAST_NAME_DIGEST=`envelope subject assertion lastName Maxwell | envelope digest`
NAME_PROOF=`envelope proof create $CREDENTIAL $FIRST_NAME_DIGEST $LAST_NAME_DIGEST`
envelope $NAME_PROOF
```

```
👈
{
    ELIDED [
        ELIDED [
            ELIDED
        ]
        ELIDED [
            ELIDED
        ]
        ELIDED (11)
    ]
} [
    ELIDED (2)
]
```

Now we confirm the contents of both fields with a single command.

```bash
👉
envelope proof confirm --silent $CREDENTIAL_ROOT $NAME_PROOF $FIRST_NAME_DIGEST $LAST_NAME_DIGEST
```

Existence proofs are a way to confirm the existence of a digest or set of digests within an envelope using minimal disclosure, but they are only one tool in the toolbox of techniques that Envelope provides. Real-life applications are likely to employ several of these tools. In the example above, we're assuming certain things such as the credential root being trusted and the signature on the envelope having been validated; these aren't provided for by the existence proof mechanism on its own. In addition, it's possible for a specific digest to appear in more than one place in the structure of an envelope, so proving that it exists in a single place where it's expected to exist also needs to be part of the process. Using tools that incorporate randomness, like salting, signing, and encryption, as well as the tree structure of the envelope provide a variety of ways to ensure that a specific digest occurs in exactly one place.
