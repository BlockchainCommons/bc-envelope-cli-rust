# `envelope` - Pattern Matching

The `envelope` tool lets you use a powerful pattern matching system inspired by regular expressions to match and extract data from Gordian Envelopes. This is particularly useful for working with complex data structures like credentials, where you may want to extract specific fields or validate the structure of the data.

## Basic Usage

Let's use a simple envelope we've used before:


```bash
👉
ALICE_KNOWS_BOB=ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns
```

```bash
👈
envelope format $ALICE_KNOWS_BOB
```

```envelope
"Alice" [
    "knows": "Bob"
]
```
