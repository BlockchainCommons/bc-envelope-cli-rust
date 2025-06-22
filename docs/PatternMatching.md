# `envelope` - Pattern Matching

The `envelope` tool lets you use a powerful pattern matching system inspired by regular expressions to match and extract data from Gordian Envelopes. This is particularly useful for working with complex data structures like credentials, where you may want to extract specific fields or validate the structure of the data.

You use the `match` subcommand to perform these operations. The basic form of the command takes a pattern and an envelope to match against that pattern. It returns zero or more *paths* starting from the root of the envelope that match the pattern. Several options let you control the output format of the paths.

👉
```bash
envelope match --help
```

👈
```
Match the envelope subject against a pattern

Usage: envelope match [OPTIONS] <PATTERN> [ENVELOPE]

Arguments:
  <PATTERN>
          The pattern to be matched

  [ENVELOPE]
          The envelope to process.

          If the envelope is not supplied on the command line, it is read from stdin.

Options:
      --no-indent
          Disable indentation of path elements

      --last-only
          Format only the last element of each path

      --envelopes
          Format path elements as envelope URs

      --digests
          Format path elements as digest URs

      --summary
          Format path elements as summary

      --max-length <MAX_LENGTH>
          Maximum length for summary truncation

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Basic Usage

The entire syntax is described in the [Pattern Syntax](PatternSyntax.md) document, but let's start with a simple example to illustrate how it works. A forthcoming chapter in [The CBOR, dCBOR, and Gordian Envelope Book](https://cborbook.com) will provide a more detailed explanation of the syntax and how to use it effectively.

Let's use a simple envelope we've used before:

👉
```bash
ALICE_KNOWS_BOB=ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns
```

We'll want to refer to both the envelope notation and the tree format of this envelope:

👉
```bash
envelope format $ALICE_KNOWS_BOB
```

👈
```envelope
"Alice" [
    "knows": "Bob"
]
```

👉
```bash
envelope format --type tree $ALICE_KNOWS_BOB
```

👈
```envelope
8955db5e NODE
    13941b48 subj "Alice"
    78d666eb ASSERTION
        db7dd21c pred "knows"
        13b74194 obj "Bob"
```

Let's start with a simple pattern that matches the subject of the envelope:

```envpat
SUBJECT
```

👉
```
$ envelope match 'SUBJECT' $ALICE_KNOWS_BOB
```

👈
```envelope
8955db5e NODE "Alice" [ "knows": "Bob" ]
    13941b48 LEAF "Alice"
```

- What is returned is a *path* starting from the root of the envelope that matches the pattern.
- Each line begins with 8 hex digits that are the start of the path element's digest, and are usually sufficient to uniquely identify the element. You can match the digest to the envelope's tree format to see an element's full position with the tree.
- The first line of the path shows that the root node is a `NODE`. `NODE`s always have one or more assertions.
- The second line shows that the subject of the envelope is a `LEAF` with the value "Alice". A `LEAF` can be any dCBOR object. In this case, it is a string.
- Each element in the path is indented by 4 spaces to show the progression down the tree.
- The `match` subcommand may return multiple paths, but in this case, there is only one path that matches the `SUBJECT` pattern.

What if our pattern was just `NODE`?

```envpat
NODE
```

👉
```
$ envelope match 'NODE' $ALICE_KNOWS_BOB
```

👈
```envelope
8955db5e NODE "Alice" [ "knows": "Bob" ]
```

Here you see the path stops with just the root node. This is because the `NODE` pattern matches the root node, which is a `NODE`. It never makes it to the subject, which is a `LEAF`.

What about an even simpler envelope: one that just has a subject and no assertions?

👉
```bash
$ ALICE=`envelope subject type string "Alice"`
$ envelope format $ALICE
```

👈
```envelope
"Alice"
```

Will our first pattern match?

👉
```bash
$ envelope match 'SUBJECT' $ALICE
```

👈
```envelope
8955db5e LEAF "Alice"
```

Yes, it does. The subject of the envelope is a `LEAF`, and the pattern `SUBJECT` matches that `LEAF`. The path returned is just the root node, which is a `LEAF`. What about the `NODE` pattern?

👉
```bash
$ envelope match 'NODE' $ALICE
```

👈
```
Error: No match
```

The `NODE` pattern does not match the root node, which is a `LEAF`. So no path is returned.

Let's make things a bit more interesting. What if we want to find every text value in the envelope? The `SEARCH` pattern lets us do that. It visits every node in the envelope tree and matches the specified pattern against each node, returning paths for each match.

```envpat
SEARCH(TEXT)
```

👉
```bash
$ envelope match 'SEARCH(TEXT)' $ALICE_KNOWS_BOB
```

👈
```envelope
8955db5e NODE "Alice" [ "knows": "Bob" ]
8955db5e NODE "Alice" [ "knows": "Bob" ]
    13941b48 LEAF "Alice"
8955db5e NODE "Alice" [ "knows": "Bob" ]
    78d666eb ASSERTION "knows": "Bob"
        db7dd21c LEAF "knows"
8955db5e NODE "Alice" [ "knows": "Bob" ]
    78d666eb ASSERTION "knows": "Bob"
        13b74194 LEAF "Bob"
```

Here we see that 4 paths are returned, each showing a different part of the envelope that matches the `TEXT` pattern. The last (most deeply indented) element of each path is a `LEAF` with a text value. Recall that `TEXT` matches the *subject* of the envelope, so the first path is just the root `NODE`, which has a subject of `"Alice"`, so it matches.

What if we want to find *just* the paths to the text values, and exclude `NODE`s whose subjects are text values? This pattern says that a matching element must be a `LEAF` with a text value:

```envpat
SEARCH(LEAF&TEXT)
```

👉
```bash
$ envelope match 'SEARCH(LEAF&TEXT)' $ALICE_KNOWS_BOB
```

👈
```envelope
8955db5e NODE "Alice" [ "knows": "Bob" ]
    13941b48 LEAF "Alice"
8955db5e NODE "Alice" [ "knows": "Bob" ]
    78d666eb ASSERTION "knows": "Bob"
        db7dd21c LEAF "knows"
8955db5e NODE "Alice" [ "knows": "Bob" ]
    78d666eb ASSERTION "knows": "Bob"
        13b74194 LEAF "Bob"
```

This returns 3 paths, each showing a `LEAF` with a text value. The first path is the subject of the envelope, which is a `LEAF` with the value "Alice". The second and third paths are the `LEAF`s for the assertion predicate and object, which are "knows" and "Bob", respectively.

Knowing the full path to each matching element is sometimes useful, but often you just want the last element of each path. You can use the `--last-only` option to do this:

👉
```bash
$ envelope match --last-only 'SEARCH(LEAF&TEXT)' $ALICE_KNOWS_BOB
```

👈
```envelope
13941b48 LEAF "Alice"
db7dd21c LEAF "knows"
13b74194 LEAF "Bob"
```

Now let's put these results to use: let's say we want to elide every text value in the envelope.

First we get the same results as above, but with the `--digests` option to format the paths as digest URs:

👉
```bash
$ TARGET_SET=`envelope match --digests --last-only 'SEARCH(LEAF&TEXT)' $ALICE_KNOWS_BOB`
$ echo $TARGET_SET
```

👈
```dcbor
ur:digest/hdcxbwmwcwfdkecauerfvsdirpwpfhfgtalfmulesnstvlrpoyfzuyenamdpmdcfutdlstyaqzrk ur:digest/hdcxuykitdcegyinqzlrlgdrcwsbbkihcemtchsntabdpldtbzjepkwsrkdrlernykrddpjtgdfh ur:digest/hdcxbwrlfpmwnsemrovtnssrtnotcfgshdvezcjedlbbtypatiwtecoxjnjnhtcafhbysptsnsnl
```

Notice that the output is a single line of space-separated digest URs, each representing the last element of a path that matches the pattern.

As it turns out, this is exactly the format we need to work with the `elide` subcommand. We can use the `elide removing` subcommand to elide all of the text values in the envelope:

👉
```bash
$ ELIDED=`envelope elide removing $TARGET_SET $ALICE_KNOWS_BOB`
$ envelope format $ELIDED
```

👈
```envelope
ELIDED [
    ELIDED: ELIDED
]
```

By comparing the digests shown in the tree format of the original envelope with those from the elided envelope, we can see that they both represent the same data, but in the elided envelope, only the digests remain.

👉
```bash
$ envelope format --type tree $ALICE_KNOWS_BOB && \
    echo "" && \
    envelope format --type tree $ELIDED
```

👈
```envelope
8955db5e NODE
    13941b48 subj "Alice"
    78d666eb ASSERTION
        db7dd21c pred "knows"
        13b74194 obj "Bob"

8955db5e NODE
    13941b48 subj ELIDED
    78d666eb ASSERTION
        db7dd21c pred ELIDED
        13b74194 obj ELIDED
```
