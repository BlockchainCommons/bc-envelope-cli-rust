# `envelope` - Inclusion Proofs

**See Associated Video:**

[![Gordian Envelope CLI - 5 - Inclusion Proofs](https://img.youtube.com/vi/LUQ-n9EZa0U/mqdefault.jpg)](https://www.youtube.com/watch?v=LUQ-n9EZa0U)

## Table of Contents

- [`envelope` - Inclusion Proofs](#envelope---inclusion-proofs)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Example 1: Alice's Friends](#example-1-alices-friends)
  - [Example 2: Verifiable Credential](#example-2-verifiable-credential)
  - [Example 3: Multiproofs](#example-3-multiproofs)

## Introduction

An *inclusion proof* is a method of showing that particular information exists in a document without revealing more than is necessary about the document in which it exists.

In a previous chapter we discussed elision, which is a method whereby information can be removed from an envelope without changing its digest tree structure.

Because each element of an envelope provides a unique digest, and because changing an element in an envelope changes the digest of all elements upwards towards its root, the structure of an envelope is comparable to a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree).

In a Merkle Tree, all semantically significant information is carried by the tree's leaves (for example, the transactions in a block of Bitcoin transactions) while the internal nodes of the tree are nothing but digests computed from combinations of pairs of lower nodes, all the way up to the root of the tree (the "Merkle root".)

In an envelope, every digest points at some potentially useful semantic information: at the subject of the envelope, at one of the assertions in the envelope, or at the predicate or object of a given assertion. Of course, those object are all envelopes themselves, so by using salt, the digest of the element can be decorrelated from its content.

In a merkle tree, the minumum subset of hashes necessary to confirm that a specific leaf node (the "target") must be present is called a "Merkle proof." For envelopes, an analogous proof would be a transformation of the envelope that is entirely elided but preserves the structure necesssary to reveal the target.

## Example 1: Alice's Friends

This document contains a list of people Alice knows. Each "knows" assertion has been salted so if the assertions have been elided one can't merely guess at who she knows by pairing the "knows" predicate with the names of possibly-known associates and comparing the resulting digests to the elided digests in the document.

```
ALICE_FRIENDS=`envelope subject type string "Alice" |
    envelope assertion add pred-obj string "knows" string "Bob" --salted |
    envelope assertion add pred-obj string "knows" string "Carol" --salted |
    envelope assertion add pred-obj string "knows" string "Dan" --salted`
envelope format $ALICE_FRIENDS

│ "Alice" [
│     {
│         "knows": "Bob"
│     } [
│         salt: Salt
│     ]
│     {
│         "knows": "Carol"
│     } [
│         salt: Salt
│     ]
│     {
│         "knows": "Dan"
│     } [
│         salt: Salt
│     ]
│ ]
```

Alice provides just the root digest of her document to a third party. This is simply an envelope in which everything has been elided and nothing revealed.

```
ALICE_FRIENDS_ROOT=`envelope elide revealing '' $ALICE_FRIENDS`
envelope format $ALICE_FRIENDS_ROOT

│ ELIDED
```

Now Alice wants to prove to the third party that her document contains a "knows Bob" assertion. To do this, she produces a proof that is an envelope with the minimal structure of digests included so that the proof envelope has the same digest as the completely elided envelope, but also exposes the digest of the target of the proof.

Note that in the proof the digests of the two other elided "knows" assertions are present, but because they have been salted, the third party cannot easily guess who else she knows.

```
KNOWS_BOB_DIGEST=`envelope subject assertion string "knows" string "Bob" | envelope digest`
KNOWS_BOB_PROOF=`envelope proof create $KNOWS_BOB_DIGEST $ALICE_FRIENDS`
envelope format $KNOWS_BOB_PROOF

│ ELIDED [
│     ELIDED [
│         ELIDED
│     ]
│     ELIDED (2)
│ ]
```

The third party then uses the previously known and trusted root to confirm that the envelope does indeed contain a "knows bob" assertion.

```
envelope proof confirm --silent $KNOWS_BOB_PROOF $KNOWS_BOB_DIGEST $ALICE_FRIENDS_ROOT
```

There is no output because the proof succeeded.

## Example 2: Verifiable Credential

A verifiable credential is constructed such that elements that might be elided are also salted, making correlation between digest and message much more difficult. Other assertions like `.issuer` and `.controller` are left unsalted.

```
BOARD_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxmtveftkkuovlgrpdpyyldmtetbftptrddmztislgvttyplmtentkistpvazooymhtansgehdcxaszmletdcsoyrdctjlfnimcwcknsrnbelglamywtdtiymkrketftjzsghkktaydmbbsncmkp"
CREDENTIAL=`envelope subject type arid 4676635a6e6068c2ef3ffd8ff726dd401fd341036e920f136a1d8af5e829496d |
    envelope assertion add pred-obj known isA string "Certificate of Completion" |
    envelope assertion add pred-obj known issuer string "Example Electrical Engineering Board" |
    envelope assertion add pred-obj known controller string "Example Electrical Engineering Board" |
    envelope assertion add pred-obj string "firstName" string "James" --salted |
    envelope assertion add pred-obj string "lastName" string "Maxwell" --salted |
    envelope assertion add pred-obj string "issueDate" date 2020-01-01 --salted |
    envelope assertion add pred-obj string "expirationDate" date 2028-01-01 --salted |
    envelope assertion add pred-obj string "photo" string "This is James Maxwell's photo." --salted |
    envelope assertion add pred-obj string "certificateNumber" string 123-456-789 --salted |
    envelope assertion add pred-obj string "subject" string "RF and Microwave Engineering" --salted |
    envelope assertion add pred-obj string "continuingEducationUnits" number 1.5 |
    envelope assertion add pred-obj string "professionalDevelopmentHours" number 15 |
    envelope assertion add pred-obj string "topics" cbor 82695375626a6563742031695375626a6563742032 |
    envelope subject type wrapped |
    envelope sign --signer $BOARD_PRVKEYS |
    envelope assertion add pred-obj known note string "Signed by Example Electrical Engineering Board"`
envelope format $CREDENTIAL

│ {
│     ARID(4676635a) [
│         isA: "Certificate of Completion"
│         {
│             "certificateNumber": "123-456-789"
│         } [
│             salt: Salt
│         ]
│         {
│             "expirationDate": 2028-01-01
│         } [
│             salt: Salt
│         ]
│         {
│             "firstName": "James"
│         } [
│             salt: Salt
│         ]
│         {
│             "issueDate": 2020-01-01
│         } [
│             salt: Salt
│         ]
│         {
│             "lastName": "Maxwell"
│         } [
│             salt: Salt
│         ]
│         {
│             "photo": "This is James Maxwell's photo."
│         } [
│             salt: Salt
│         ]
│         {
│             "subject": "RF and Microwave Engineering"
│         } [
│             salt: Salt
│         ]
│         "continuingEducationUnits": 1.5
│         "professionalDevelopmentHours": 15
│         "topics": ["Subject 1", "Subject 2"]
│         controller: "Example Electrical Engineering Board"
│         issuer: "Example Electrical Engineering Board"
│     ]
│ } [
│     note: "Signed by Example Electrical Engineering Board"
│     signed: Signature
│ ]
```

```
CREDENTIAL_ROOT=`envelope elide revealing '' $CREDENTIAL`
envelope format $CREDENTIAL_ROOT

│ ELIDED
```

In this case the holder of a credential wants to prove a single assertion from it: the subject.

```
SUBJECT_DIGEST=`envelope subject assertion string "subject" string "RF and Microwave Engineering" | envelope digest`
SUBJECT_PROOF=`envelope proof create $SUBJECT_DIGEST $CREDENTIAL`
envelope format $SUBJECT_PROOF

│ {
│     ELIDED [
│         ELIDED [
│             ELIDED
│         ]
│         ELIDED (12)
│     ]
│ } [
│     ELIDED (2)
│ ]
```

The proof confirms the subject, as intended.

```
envelope proof confirm --silent $SUBJECT_PROOF $SUBJECT_DIGEST $CREDENTIAL_ROOT
```

Assertions without salt can be guessed at, and confirmed if the the guess is correct.

```
ISSUER_DIGEST=`envelope subject assertion known issuer string "Example Electrical Engineering Board" | envelope digest`
envelope proof confirm --silent $SUBJECT_PROOF $ISSUER_DIGEST $CREDENTIAL_ROOT
```

The proof cannot be used to confirm salted assertions.

```
FIRST_NAME_DIGEST=`envelope subject assertion string "firstName" string "James" | envelope digest`
envelope proof confirm --silent $SUBJECT_PROOF $FIRST_NAME_DIGEST $CREDENTIAL_ROOT

│ Error: Proof does not confirm target
```

## Example 3: Multiproofs

A single proof can be generated to reveal multiple target digests. In this example we prove the holder's `firstName` and `lastName` using a single proof, even though they are in different fields.

```
FIRST_NAME_DIGEST=`envelope subject assertion string "firstName" string "James" | envelope digest`
LAST_NAME_DIGEST=`envelope subject assertion string "lastName" string "Maxwell" | envelope digest`
NAME_PROOF=`envelope proof create "$FIRST_NAME_DIGEST $LAST_NAME_DIGEST" $CREDENTIAL`
envelope format $NAME_PROOF

│ {
│     ELIDED [
│         ELIDED [
│             ELIDED
│         ]
│         ELIDED [
│             ELIDED
│         ]
│         ELIDED (11)
│     ]
│ } [
│     ELIDED (2)
│ ]
```

Now we confirm the contents of both fields with a single command.

```
envelope proof confirm --silent $NAME_PROOF "$FIRST_NAME_DIGEST $LAST_NAME_DIGEST" $CREDENTIAL_ROOT
```

Inclusion proofs are a way to confirm the existence of a digest or set of digests within an envelope using minimal disclosure, but they are only one tool in the toolbox of techniques that Envelope provides. Real-life applications are likely to employ several of these tools. In the example above, we're assuming certain things such as the credential root being trusted and the signature on the envelope having been validated; these aren't provided for by the inclusion proof mechanism on its own. In addition, it's possible for a specific digest to appear in more than one place in the structure of an envelope, so proving that it exists in a single place where it's expected to exist also needs to be part of the process. Using tools that incorporate randomness, like salting, signing, and encryption, as well as the tree structure of the envelope provide a variety of ways to ensure that a specific digest occurs in exactly one place.
