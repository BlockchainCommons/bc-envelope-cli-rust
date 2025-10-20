# Walking Envelope Nodes

The `walk` command provides tools for navigating and manipulating envelope nodes based on their obscuration state (elided, encrypted, or compressed).

## Table of Contents

- [Walking Envelope Nodes](#walking-envelope-nodes)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Basic Walk](#basic-walk)
  - [Matching Obscured Nodes](#matching-obscured-nodes)
  - [Uneliding Nodes](#uneliding-nodes)
  - [Decrypting Nodes](#decrypting-nodes)
  - [Decompressing Nodes](#decompressing-nodes)
  - [Using Target Filters](#using-target-filters)
  - [Complete Example](#complete-example)

## Overview

The `walk` command traverses an envelope's structure and provides several operations:

- **Default behavior**: List all node digests
- **`matching`**: Find nodes by obscuration type
- **`unelide`**: Restore elided nodes
- **`decrypt`**: Decrypt encrypted nodes
- **`decompress`**: Decompress compressed nodes

All subcommands can optionally filter nodes using the `--target` option.

## Basic Walk

Without any subcommand, `walk` outputs all node digests in the envelope. First, let's set up our example envelope:

```
ALICE_KNOWS_BOB=ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns
```

Now let's walk the envelope to see all its node digests:

```
envelope walk $ALICE_KNOWS_BOB

│ ur:digest/hdcxbwmwcwfdkecauerfvsdirpwpfhfgtalfmulesnstvlrpoyfzuyenamdpmdcfutdlstyaqzrk ur:digest/hdcxbwrlfpmwnsemrovtnssrtnotcfgshdvezcjedlbbtypatiwtecoxjnjnhtcafhbysptsnsnl ur:digest/hdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboerhzebtws ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds ur:digest/hdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrddpjtgdfh
```

This is useful for getting a complete inventory of all nodes in an envelope. The output is space-separated digests, suitable for use in shell scripts.

## Matching Obscured Nodes

The `matching` subcommand finds nodes that match specific obscuration types. Let's create an envelope with an elided part to demonstrate. We'll elide the entire assertion:

```
KNOWS_ASSERTION=$(envelope assertion at 0 $ALICE_KNOWS_BOB)
ASSERTION_DIGEST=$(envelope digest $KNOWS_ASSERTION)
ELIDED=$(envelope elide removing $ASSERTION_DIGEST $ALICE_KNOWS_BOB)
envelope format $ELIDED

│ "Alice" [
│     ELIDED
│ ]
```

Now we can find all elided nodes:

```
envelope walk $ELIDED matching --elided

│ ur:digest/hdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboerhzebtws
```

You can also search for encrypted or compressed nodes. Let's create a compressed envelope:

```
COMPRESSED=$(envelope compress $ALICE_KNOWS_BOB)
envelope format $COMPRESSED

│ COMPRESSED
```

Find all compressed nodes:

```
envelope walk $COMPRESSED matching --compressed

│ ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds
```

You can combine multiple flags to find nodes that are either elided or compressed:

```
envelope walk $ELIDED matching --elided --compressed

│ ur:digest/hdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboerhzebtws
```

## Uneliding Nodes

The `unelide` subcommand restores elided nodes by providing the original envelopes. Let's restore our elided envelope by providing the original assertion:

```
RESTORED=$(envelope walk $ELIDED unelide $KNOWS_ASSERTION)
envelope format $RESTORED

│ "Alice" [
│     "knows": "Bob"
│ ]
```

The envelope is now fully restored. You can provide multiple envelopes to unelide different parts. Let's create an envelope with multiple elided parts - we'll elide both the subject "Alice" and the object "Bob":

```
ALICE_DIGEST=$(envelope subject type string "Alice" | envelope digest)
BOB_DIGEST=$(envelope subject type string "Bob" | envelope digest)
MULTI_ELIDED=$(envelope elide removing "$ALICE_DIGEST $BOB_DIGEST" $ALICE_KNOWS_BOB)
envelope format $MULTI_ELIDED

│ ELIDED [
│     "knows": ELIDED
│ ]
```

Now restore both parts by providing the individual envelopes:

```
ALICE_ENV=$(envelope subject type string "Alice")
BOB_ENV=$(envelope subject type string "Bob")
MULTI_RESTORED=$(envelope walk $MULTI_ELIDED unelide $ALICE_ENV $BOB_ENV)
envelope format $MULTI_RESTORED

│ "Alice" [
│     "knows": "Bob"
│ ]
```

## Decrypting Nodes

The `decrypt` subcommand decrypts encrypted nodes using symmetric keys. First, let's generate a key and encrypt an envelope:

Note that encrypting an envelope only encrypts its subject, not its assertions. To encrypt the whole envelope including assertions, you would need to wrap it first.

```
KEY=$(envelope generate key)
ENCRYPTED=$(envelope encrypt --key $KEY $ALICE_KNOWS_BOB)
envelope format $ENCRYPTED

│ ENCRYPTED [
│     "knows": "Bob"
│ ]
```

Now decrypt it using the walk command:

```
DECRYPTED=$(envelope walk $ENCRYPTED decrypt $KEY)
envelope format $DECRYPTED

│ "Alice" [
│     "knows": "Bob"
│ ]
```

You can provide multiple keys to decrypt different parts that were encrypted with different keys. Let's encrypt the assertion as well with a different key:

```
KEY2=$(envelope generate key)
ASSERTION=$(envelope assertion at 0 $ALICE_KNOWS_BOB)
ASSERTION_DIGEST=$(envelope digest $ASSERTION)
DOUBLE_ENCRYPTED=$(envelope elide removing $ASSERTION_DIGEST --action encrypt --key $KEY2 $ENCRYPTED)
envelope format $DOUBLE_ENCRYPTED

│ ENCRYPTED [
│     ENCRYPTED
│ ]
```

Now decrypt with both keys:

```
DOUBLE_DECRYPTED=$(envelope walk $DOUBLE_ENCRYPTED decrypt $KEY $KEY2)
envelope format $DOUBLE_DECRYPTED

│ "Alice" [
│     "knows": "Bob"
│ ]
```

## Decompressing Nodes

The `decompress` subcommand decompresses compressed nodes. We already created a compressed envelope earlier, so let's decompress it:

```
DECOMPRESSED=$(envelope walk $COMPRESSED decompress)
envelope format $DECOMPRESSED

│ "Alice" [
│     "knows": "Bob"
│ ]
```

The envelope is now fully decompressed and readable.

## Using Target Filters

All `walk` operations support the `--target` option to filter which nodes are processed. Let's get the top-level digest:

```
TOP_DIGEST=$(envelope digest $ALICE_KNOWS_BOB)
```

Now only process nodes matching that specific digest:

```
envelope walk --target $TOP_DIGEST $ALICE_KNOWS_BOB

│ ur:digest/hdcxldgouyhyadimzmpaeourhfsectvaskspdlotaxidiatbgydejnbwgskbhfrtwlwzneroatds
```

This is particularly useful for selective operations. Let's create an envelope with a large data field and selectively compress just that assertion:

```
LARGE_TEXT=$(envelope subject type string "$(printf 'A%.0s' {1..1000})")
ENVELOPE_WITH_LARGE=$(envelope assertion add pred-obj string "data" envelope $LARGE_TEXT $ALICE_KNOWS_BOB)
DATA_ASSERTION=$(envelope assertion find predicate string "data" $ENVELOPE_WITH_LARGE)
DATA_DIGEST=$(envelope digest $DATA_ASSERTION)
SELECTIVE_COMPRESSED=$(envelope elide removing $DATA_DIGEST --action compress $ENVELOPE_WITH_LARGE)
envelope format $SELECTIVE_COMPRESSED

│ "Alice" [
│     "knows": "Bob"
│     COMPRESSED
│ ]
```

Now we can decompress everything:

```
FULL_DECOMPRESSED=$(envelope walk $SELECTIVE_COMPRESSED decompress)
envelope format $FULL_DECOMPRESSED

│ "Alice" [
│     "data": "AAAAAAAAAA…AAAAAAAAAA"
│     "knows": "Bob"
│ ]
```

This selective compression and decompression is useful for revealing only specific parts of an envelope while keeping other parts obscured.

## Complete Example

Here's a complete workflow demonstrating how to work with mixed obscuration types. Let's create an envelope with some assertions and then apply different types of obscuration:

```
COMPLEX=$(envelope subject type string "Alice")
COMPLEX=$(envelope assertion add pred-obj string "name" string "Alice Smith" $COMPLEX)
COMPLEX=$(envelope assertion add pred-obj string "age" number 30 $COMPLEX)
BIO_TEXT=$(envelope subject type string "$(printf 'X%.0s' {1..1000})")
COMPLEX=$(envelope assertion add pred-obj string "bio" envelope $BIO_TEXT $COMPLEX)
envelope format $COMPLEX

│ "Alice" [
│     "age": 30
│     "bio": "XXXXXXXXXX…XXXXXXXXXX"
│     "name": "Alice Smith"
│ ]
```

Now let's apply mixed obscuration - elide the name, encrypt the age, and compress the bio:

```
NAME_DIGEST=$(envelope assertion find predicate string "name" $COMPLEX | envelope digest)
AGE_DIGEST=$(envelope assertion find predicate string "age" $COMPLEX | envelope digest)
BIO_DIGEST=$(envelope assertion find predicate string "bio" $COMPLEX | envelope digest)
ENCRYPT_KEY=$(envelope generate key)
MIXED=$(envelope elide removing $NAME_DIGEST $COMPLEX)
MIXED=$(envelope elide removing $AGE_DIGEST --action encrypt --key $ENCRYPT_KEY $MIXED)
MIXED=$(envelope elide removing $BIO_DIGEST --action compress $MIXED)
envelope format $MIXED

│ "Alice" [
│     COMPRESSED
│     ELIDED
│     ENCRYPTED
│ ]
```

Find all the different types of obscured nodes (note: the actual digest values will vary since they depend on the randomly-generated encryption key):

```
envelope walk $MIXED matching --elided

│ ur:digest/hdcxmktnlylaahtlfnbyndkepscpwztthetkknatrkltjewkftpyrylemdeojtnnnsrpjpzccmga
```

```
envelope walk $MIXED matching --encrypted

│ ur:digest/hdcxbarehnndlolujypllofdhgylwzjtrlwtaxlpcmpadmqdostnwkskylmnvaghwllsjzsbaocw
```

```
envelope walk $MIXED matching --compressed

│ ur:digest/hdcxsnfdlpaygsksjypkwzahoxurdyuorkmhlrkesfemldwmwnjygdotiansvegsztnddedyykje
```

Now let's restore everything step by step. First, unelide the name by providing the specific assertion:

```
NAME_ASSERTION=$(envelope assertion find predicate string "name" $COMPLEX)
NAME_RESTORED=$(envelope walk $MIXED unelide $NAME_ASSERTION)
envelope format $NAME_RESTORED

│ "Alice" [
│     "name": "Alice Smith"
│     COMPRESSED
│     ENCRYPTED
│ ]
```

Next, decrypt the age:

```
AGE_RESTORED=$(envelope walk $NAME_RESTORED decrypt $ENCRYPT_KEY)
envelope format $AGE_RESTORED

│ "Alice" [
│     "age": 30
│     "name": "Alice Smith"
│     COMPRESSED
│ ]
```

Finally, decompress the bio:

```
FULLY_RESTORED=$(envelope walk $AGE_RESTORED decompress)
envelope format $FULLY_RESTORED

│ "Alice" [
│     "age": 30
│     "bio": "XXXXXXXXXX…XXXXXXXXXX"
│     "name": "Alice Smith"
│ ]
```

Or do it all in one command:

```
ALL_RESTORED=$(envelope walk $MIXED unelide $NAME_ASSERTION | envelope walk decrypt $ENCRYPT_KEY | envelope walk decompress)
envelope format $ALL_RESTORED

│ "Alice" [
│     "age": 30
│     "bio": "XXXXXXXXXX…XXXXXXXXXX"
│     "name": "Alice Smith"
│ ]
```
