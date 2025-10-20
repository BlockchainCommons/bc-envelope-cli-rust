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
  - [Replacing Nodes](#replacing-nodes)
    - [Basic Replacement](#basic-replacement)
    - [Replacing Elided Nodes](#replacing-elided-nodes)
    - [Replacing Multiple Different Nodes](#replacing-multiple-different-nodes)
    - [Replacement Validation](#replacement-validation)
  - [Using Target Filters](#using-target-filters)
  - [Complete Example](#complete-example)

## Overview

The `walk` command traverses an envelope's structure and provides several operations:

- **Default behavior**: List all node digests
- **`matching`**: Find nodes by obscuration type
- **`unelide`**: Restore elided nodes
- **`decrypt`**: Decrypt encrypted nodes
- **`decompress`**: Decompress compressed nodes
- **`replace`**: Replace nodes matching target digests with a replacement envelope

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

## Replacing Nodes

The `replace` subcommand allows you to replace nodes in an envelope that match specific digests with a replacement envelope. This is useful for transforming envelopes by substituting specific elements throughout the structure, regardless of whether those elements are plaintext or obscured (elided, encrypted, or compressed).

The `replace` subcommand requires the `--target` option to specify which digests to replace.

### Basic Replacement

Let's start by creating an envelope where "Bob" appears in multiple places:

```
BOB=$(envelope subject type string "Bob")
CHARLIE=$(envelope subject type string "Charlie")
ENVELOPE_WITH_BOB=$(envelope assertion add pred-obj string "likes" envelope $BOB $ALICE_KNOWS_BOB)
envelope format $ENVELOPE_WITH_BOB

│ "Alice" [
│     "knows": "Bob"
│     "likes": "Bob"
│ ]
```

Now let's replace all instances of "Bob" with "Charlie". First, we need Bob's digest:

```
BOB_DIGEST=$(envelope digest $BOB)
```

Now we can replace all occurrences:

```
REPLACED=$(envelope walk --target $BOB_DIGEST $ENVELOPE_WITH_BOB replace $CHARLIE)
envelope format $REPLACED

│ "Alice" [
│     "knows": "Charlie"
│     "likes": "Charlie"
│ ]
```

Both instances of "Bob" have been replaced with "Charlie" because they share the same digest.

### Replacing Elided Nodes

You can replace ELIDED nodes with envelopes that *do not* match the elided content. If you do this, you are no longer *transforming* the original content, which would leave the Merkle structure the same, but rather substituting new content in place of the elided parts. This will change all the digests in the envelope from the point of the change up to the envelope's root digest. Doing this is likely to invalidate any signatures on the envelope, so use this feature with caution.

Let's demonstrate this by first creating an envelope and checking its root digest:

```
ENVELOPE_WITH_BOB=$(envelope assertion add pred-obj string "likes" envelope $BOB $ALICE_KNOWS_BOB)
envelope format $ENVELOPE_WITH_BOB

│ "Alice" [
│     "knows": "Bob"
│     "likes": "Bob"
│ ]
```

```
envelope digest $ENVELOPE_WITH_BOB

│ ur:digest/hdcxbesedketplchinhntebnvevlsremctoxvsynckzcnychtnrekbwzhdvtzesoamidutsksekn
```

Now elide Bob and check the root digest again. Notice that it remains the same because elision preserves the Merkle structure:

```
ELIDED=$(envelope elide removing $BOB_DIGEST $ENVELOPE_WITH_BOB)
envelope format $ELIDED

│ "Alice" [
│     "knows": ELIDED
│     "likes": ELIDED
│ ]
```

```
envelope digest $ELIDED

│ ur:digest/hdcxbesedketplchinhntebnvevlsremctoxvsynckzcnychtnrekbwzhdvtzesoamidutsksekn
```

Now replace the elided nodes (which have Bob's digest) with Charlie. This substitutes different content, so the root digest will change:

```
REPLACED_ELIDED=$(envelope walk --target $BOB_DIGEST $ELIDED replace $CHARLIE)
envelope format $REPLACED_ELIDED

│ "Alice" [
│     "knows": "Charlie"
│     "likes": "Charlie"
│ ]
```

```
envelope digest $REPLACED_ELIDED

│ ur:digest/hdcxcytdknwzttswkiaetyylbdmncmrkemlbfhcljegaosayjnsgbdptfdcewkinmdfllksrbebt
```

The root digest has changed because we've substituted different content. This is fundamentally different from uneliding with the original content, which would preserve the original digest.

### Replacing Multiple Different Nodes

You can replace multiple different elements with the same replacement by providing multiple target digests. Let's create an envelope with both Bob and Carol, then replace both with "REDACTED":

```
CAROL=$(envelope subject type string "Carol")
REDACTED=$(envelope subject type string "REDACTED")
ENVELOPE=$(envelope assertion add pred-obj string "likes" envelope $CAROL $ALICE_KNOWS_BOB)
envelope format $ENVELOPE

│ "Alice" [
│     "knows": "Bob"
│     "likes": "Carol"
│ ]
```

Get both digests and replace them in one operation:

```
BOB_DIGEST=$(envelope digest $BOB)
CAROL_DIGEST=$(envelope digest $CAROL)
MULTI_REPLACED=$(envelope walk --target "$BOB_DIGEST $CAROL_DIGEST" $ENVELOPE replace $REDACTED)
envelope format $MULTI_REPLACED

│ "Alice" [
│     "knows": "REDACTED"
│     "likes": "REDACTED"
│ ]
```

This is useful for anonymizing or redacting multiple pieces of information in a structured way.

### Replacement Validation

The `replace` subcommand validates that replacements maintain envelope structure integrity. Specifically, assertions (elements in a node's assertions array) can only be replaced with other assertions or obscured elements (which are presumed to be obscured assertions).

Let's try to replace an entire assertion with a plain string, which should fail:

```
KNOWS_ASSERTION=$(envelope assertion at 0 $ALICE_KNOWS_BOB)
ASSERTION_DIGEST=$(envelope digest $KNOWS_ASSERTION)
CHARLIE=$(envelope subject type string "Charlie")
envelope walk --target $ASSERTION_DIGEST $ALICE_KNOWS_BOB replace $CHARLIE

│ Error: invalid format
```

This error occurs because "Charlie" is a plain string envelope, not an assertion. The envelope structure requires that elements in the assertions array be either:
- Actual assertions (predicate-object pairs)
- Obscured elements (ELIDED, ENCRYPTED, or COMPRESSED) which are presumed to be obscured assertions

This validation ensures that envelopes maintain their structural integrity even when performing transformations.

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
