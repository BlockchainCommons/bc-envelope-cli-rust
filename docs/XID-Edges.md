# Working with XID Edges

XID Edges are verifiable claims that connect XID entities. They allow one entity (the source) to make a signed statement about another entity (the target), or about itself. Edges are defined in [BCR-2026-003](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2026-003-xid-edges.md).

An edge is a Gordian Envelope with:

- A **subject** — a locally unique identifier for the claim
- Exactly three assertions:
  - `'isA'` — the type of claim
  - `'source'` — the XID of the entity making the claim
  - `'target'` — the XID of the entity the claim is about

## Setup

We'll use Alice and Bob for our examples. First, set up their XID documents:

```
ALICE_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnwksorlbd"
ALICE_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls"
BOB_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwphejzwnmhhf"

XID_DOC=$(envelope xid new "$ALICE_PRVKEYS")
ALICE_XID=$(envelope xid id "$XID_DOC")
BOB_DOC=$(envelope xid new "$BOB_PUBKEYS")
BOB_XID=$(envelope xid id "$BOB_DOC")
```

## Edge Structure

An edge envelope has a string subject (the claim identifier) and exactly three required assertions. Here is how to create one using the `envelope` command:

```
EDGE=$(envelope subject type string "self-description")
EDGE=$(envelope assertion add pred-obj known isA string "foaf:Person" "$EDGE")
EDGE=$(envelope assertion add pred-obj known source ur "$ALICE_XID" "$EDGE")
EDGE=$(envelope assertion add pred-obj known target ur "$ALICE_XID" "$EDGE")

envelope format "$EDGE"

│ "self-description" [
│     'isA': "foaf:Person"
│     'source': XID(93a4d4e7)
│     'target': XID(93a4d4e7)
│ ]
```

In this self-description edge, Alice is both the source (claimant) and the target (subject of the claim).

## Adding Edges to XID Documents

Use `xid edge add` to add an edge to a XID document:

```
XID_DOC=$(envelope xid edge add "$EDGE" "$XID_DOC")

envelope format "$XID_DOC"

│ XID(93a4d4e7) [
│     'edge': "self-description" [
│         'isA': "foaf:Person"
│         'source': XID(93a4d4e7)
│         'target': XID(93a4d4e7)
│     ]
│     'key': PublicKeys(cab108a0, ...) [
│         ...
│     ]
│ ]
```

## Counting Edges

```
envelope xid edge count "$XID_DOC"

│ 1
```

## Adding Multiple Edges

You can add multiple edges to the same XID document. Each edge has its own unique subject identifier:

```
EDGE2=$(envelope subject type string "knows-bob")
EDGE2=$(envelope assertion add pred-obj known isA string "schema:colleague" "$EDGE2")
EDGE2=$(envelope assertion add pred-obj known source ur "$ALICE_XID" "$EDGE2")
EDGE2=$(envelope assertion add pred-obj known target ur "$BOB_XID" "$EDGE2")

XID_DOC=$(envelope xid edge add "$EDGE2" "$XID_DOC")

envelope xid edge count "$XID_DOC"

│ 2
```

## Retrieving Edges

### Get All Edges

```
envelope xid edge all "$XID_DOC"
```

This outputs one UR string per line, one for each edge.

### Get Edge at Index

```
envelope xid edge at 0 "$XID_DOC"
```

Returns the edge at the specified zero-based index.

## Finding Edges

The `xid edge find` command filters edges by optional criteria. All criteria are optional and can be combined.

### Find by Type

```
IS_A=$(envelope subject type string "foaf:Person")
envelope xid edge find --is-a "$IS_A" "$XID_DOC"
```

Returns only edges whose `'isA'` value matches.

### Find by Source

```
SOURCE=$(envelope subject type ur "$ALICE_XID")
envelope xid edge find --source "$SOURCE" "$XID_DOC"
```

### Find by Target

```
TARGET=$(envelope subject type ur "$BOB_XID")
envelope xid edge find --target "$TARGET" "$XID_DOC"
```

### Find by Subject Identifier

```
SUBJ=$(envelope subject type string "self-description")
envelope xid edge find --subject "$SUBJ" "$XID_DOC"
```

### Combined Filters

Multiple filters narrow the results. Only edges matching all criteria are returned:

```
envelope xid edge find --is-a "$IS_A" --subject "$SUBJ" "$XID_DOC"
```

## Removing Edges

```
XID_DOC=$(envelope xid edge remove "$EDGE" "$XID_DOC")

envelope xid edge count "$XID_DOC"

│ 1
```

The edge to remove is identified by its exact envelope.

## Working with Signed XID Documents

When working with signed XID documents, use `--verify inception` and `--sign inception` to maintain signature integrity:

```
XID_DOC=$(envelope xid new "$ALICE_PRVKEYS")
ALICE_XID=$(envelope xid id "$XID_DOC")

EDGE=$(envelope subject type string "self-description")
EDGE=$(envelope assertion add pred-obj known isA string "foaf:Person" "$EDGE")
EDGE=$(envelope assertion add pred-obj known source ur "$ALICE_XID" "$EDGE")
EDGE=$(envelope assertion add pred-obj known target ur "$ALICE_XID" "$EDGE")

XID_DOC=$(envelope xid edge add "$EDGE" --sign inception "$XID_DOC")

envelope xid id --verify inception "$XID_DOC"
```

The edge is included in the signature, so modifying or removing edges requires re-signing.

### Removing with Re-signing

```
XID_DOC=$(envelope xid edge remove "$EDGE" --verify inception --sign inception "$XID_DOC")
```

## Edges Persist Across Operations

Once added, edges persist through other XID document operations:

```
XID_DOC=$(envelope xid method add "https://example.com/resolve" --sign inception "$XID_DOC")

envelope xid edge count "$XID_DOC"

│ 1
```

## Relationship Edges

A relationship edge connects two different XIDs. Alice claims a relationship with Bob:

```
EDGE=$(envelope subject type string "alice-bob-colleague")
EDGE=$(envelope assertion add pred-obj known isA string "schema:colleague" "$EDGE")
EDGE=$(envelope assertion add pred-obj known source ur "$ALICE_XID" "$EDGE")
EDGE=$(envelope assertion add pred-obj known target ur "$BOB_XID" "$EDGE")

XID_DOC=$(envelope xid edge add "$EDGE" --sign inception "$XID_DOC")
```

## Third-Party Credentials

A third party (e.g., a university) can create and sign an edge, which the target entity then adds to their own XID document. This allows verifiable credentials from external issuers.

The edge should be constructed by the issuer, wrapped and signed with their key, then provided to the target entity for inclusion:

```
# Issuer creates and signs the edge
CREDENTIAL=$(envelope subject type string "degree-2024")
CREDENTIAL=$(envelope assertion add pred-obj known isA string "schema:EducationalOccupationalCredential" "$CREDENTIAL")
CREDENTIAL=$(envelope assertion add pred-obj known source ur "$ISSUER_XID" "$CREDENTIAL")
CREDENTIAL=$(envelope assertion add pred-obj known target ur "$BOB_XID" "$CREDENTIAL")
SIGNED_CREDENTIAL=$(envelope sign --signer "$ISSUER_PRVKEYS" "$CREDENTIAL")

# Bob adds the signed credential to his XID document
BOB_DOC=$(envelope xid edge add "$SIGNED_CREDENTIAL" "$BOB_DOC")
```

The signed edge remains independently verifiable even when extracted from the XID document.
