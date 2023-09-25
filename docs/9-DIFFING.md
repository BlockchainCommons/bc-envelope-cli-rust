# envelope - diffing

**NOTE:** Most of this documentation has *not* been updated to reflect the new command line syntax. In particular, this document describes functionality that is not yet supported.

## Introduction

Comparing two structures like text strings for the minimum number of changes necessary to convert one to the other is called the [edit distance problem](https://en.wikipedia.org/wiki/Edit_distance). The Unix `diff` command is used to compare two files (the "source" and "target") and produce a set of changes ("edits") that can be applied to the source (using the `patch` command) to reproduce the target. The set of edits is generally smaller than either the source or the target unless the source and/or target are very small, or the contents of the two are very different.

The reference implementation for Gordian Envelopes contains an implementation of diffing specifically for envelopes. This implementation uses the [Zhang-Shasha ordered tree edit distance algorithm](http://grantjenks.com/wiki/_media/ideas/simple_fast_algorithms_for_the_editing_distance_between_tree_and_related_problems.pdf) to produce a minimal set of edits needed to transform one envelope into the other.

## Example 1

Consider the following two envelopes:

```bash
👉
E1=`envelope subject "Alice" | \
    envelope assertion "knows" "Bob"`
envelope $E1
```

```
👈
"Alice" [
    "knows": "Bob"
]
```

```bash
👉
E2=`envelope subject "Carol" | \
    envelope assertion "knows" "Bob"`
envelope $E2
```

```
👈
"Carol" [
    "knows": "Bob"
]
```

There is one edit required to change the first envelope into the second: changing the subject of the envelope from "Alice" to "Carol":

```bash
👉
DIFF=`envelope diff create $E1 $E2`
E3=`envelope diff apply $E1 $DIFF`
envelope $E3
```

```
👈
"Carol" [
    "knows": "Bob"
]
```

```bash
👉
echo $E2; echo $E3
```

```
👈
ur:envelope/lftpsptpcsihfxhsjpjljztpspoytpsptpcsihjejtjlktjktpsptpcsiafwjlidemsgpyat
ur:envelope/lftpsptpcsihfxhsjpjljztpspoytpsptpcsihjejtjlktjktpsptpcsiafwjlidemsgpyat
```

Envelopes `E1` and `E2` are the source and target, `edits` is the list of edits needed, and `E3` is the result of applying the edits to `E1`. The last command line prints `E2` and `E3` together to show they are the same.

The edits themselves are conveyed as an envelope:

```bash
👉
envelope $DIFF
```

```
👈
edits: [1, [3, [0, "Carol"]]]
```

It is a bare assertion with the known value `edits` as its predicate and a CBOR array as its `object`. The first element of the array is the version number, each subsequent element of the array represents a deletion, renaming, or insert of a node into the tree. The specific format of the edits is considered opaque, and edits not starting with the version number `1` will be incompatible.

Because edits are themselves conveyed using an envelope, edits can be authenticated by signing, encrypted, or anything else that envelopes can do.

## Example 2

This is a somewhat more complex example that shows an initial envelope that already has one of its assertions encrypted and then which has several edits made to it, including deletion of some assertions, then being wrapped and signed, and then elided.


```bash
👉
KEY=`envelope generate key`
ENCRYPTED_ASSERTION=`envelope assertion create "knows" "Bob" | envelope encrypt --key $KEY`
CAROL_ASSERTION=`envelope assertion create "knows" "Carol"`
E1=`envelope subject "Alice" | \
    envelope assertion add envelope $CAROL_ASSERTION | \
    envelope assertion add "knows" "Edward" | \
    envelope assertion add "knows" "Geraldine" | \
    envelope assertion add envelope $ENCRYPTED_ASSERTION`
envelope $E1
```

```
👈
"Alice" [
    "knows": "Carol"
    "knows": "Edward"
    "knows": "Geraldine"
    ENCRYPTED
]
```

```bash
👉
PRVKEYS=`envelope generate prvkeys`
CAROL_ASSERTION_DIGEST=`envelope digest $CAROL_ASSERTION`
E2=`envelope assertion "knows" "frank" $E1 | \
    envelope assertion remove "knows" "Edward" | \
    envelope assertion remove "knows" "Geraldine" | \
    envelope subject --wrapped | \
    envelope sign --prvkeys $PRVKEYS`

E2=`envelope elide removing $E2 $CAROL_ASSERTION_DIGEST`
envelope $E2
```

```
👈
{
    "Alice" [
        "knows": "frank"
        ELIDED
        ENCRYPTED
    ]
} [
    verifiedBy: Signature
]
```

```bash
👉
DIFF=`envelope diff create $E1 $E2`
envelope $DIFF
```

```
👈
edits: [1, 5, 4, [10, [1]], [1, [0, "frank"]], [3, [6, Digest(4012caf2)]], [7, [2, verifiedBy]], [8, [0, Signature]], [21, [0, "Alice"], 10, 0, 2, [2, 0, 1, 3, 6]]]
```

```bash
👉
E3=`envelope diff apply $E1 $DIFF`
envelope $E3
```

```
👈
{
    "Alice" [
        "knows": "frank"
        ELIDED
        ENCRYPTED
    ]
} [
    verifiedBy: Signature
]
```

```bash
👉
echo $E2; echo $E3
```

```
👈
ur:envelope/lftpsplrtpcsihfpjziniaihoytpcsihjejtjlktjktpcsihiyjphsjtjehdcxfzbgsgwztajewfmtdabbrfzctklgtsbnecchecuestdwlpjtsksntkdmvlhlimmetansfwlrgylnjniehsetfxgecapfaonysronclteynwygscewzvlldiazelbnlkpflkilogdjzhyntonjldmnstyfefytnrpstsaeepthddatansfphdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboeoyaxtpcstansghhdfzfzmdureyioguzttosgesrkfsbkhfesbybsiomtdtbetlwtetpkgwgrlrehsokpztrochdnahjprdhnfsstbakeihbbchiereamspdwkiaoiawpmwtobzdagyykfdpfuohgdadalo
ur:envelope/lftpsplrtpcsihfpjziniaihoytpcsihjejtjlktjktpcsihiyjphsjtjehdcxfzbgsgwztajewfmtdabbrfzctklgtsbnecchecuestdwlpjtsksntkdmvlhlimmetansfwlrgylnjniehsetfxgecapfaonysronclteynwygscewzvlldiazelbnlkpflkilogdjzhyntonjldmnstyfefytnrpstsaeepthddatansfphdcxkstbiywmmygsasktnbfwhtrppkclwdcmmugejesokejlbnftrdwspsmdcechbboeoyaxtpcstansghhdfzfzmdureyioguzttosgesrkfsbkhfesbybsiomtdtbetlwtetpkgwgrlrehsokpztrochdnahjprdhnfsstbakeihbbchiereamspdwkiaoiawpmwtobzdagyykfdpfuohgdadalo
```
