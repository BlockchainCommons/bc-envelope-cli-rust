# Overview of the Commands

## Help

Help is available for the tool and its subcommands.

```
nvelope --help
```

```
A tool for manipulating the Gordian Envelope data type

Usage: nvelope <COMMAND>

Commands:
  assertion   Work with the envelope's assertions
  compress    Compress the envelope or its subject
  decrypt     Decrypt the envelope's subject using the provided key
  digest      Print the envelope's digest
  elide       Elide a subset of elements
  encrypt     Encrypt the envelope's subject using the provided key
  extract     Extract the subject of the input envelope
  format      Print the envelope in textual format
  generate    Utilities to generate and convert various objects
  salt        Add random salt to the envelope
  sign        Sign the envelope subject with the provided private key base
  sskr        Sharded Secret Key Reconstruction (SSKR)
  subject     Create an envelope with the given subject
  uncompress  Uncompress the envelope or its subject
  verify      Verify a signature on the envelope using the provided public key base
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Here is an example envelope we'll use in many of the examples below. The `envelope` tool expects input and produces output for a number of types it uses in UR format.

```
ALICE_KNOWS_BOB=ur:envelope/lftpcsihfpjziniaihoytpcsihjejtjlktjktpcsiafwjliddssngwct
```

## Format

**NOTE:** Unlike the Swift `envelope` tool, the Rust-based tool has no default commands. The downside of this is that some common commands are necessarily more verbose. The upside is that the meaning of commands is more explicit.

Without any options, the `format` command takes an envelope in UR format prints out its formatted contents in Envelope Notation:

```
nvelope format $ALICE_KNOWS_BOB
```

```
"Alice" [
    "knows": "Bob"
]
```

### Hex CBOR Output

The `format` command has several output format options specified using `--type`. For example, you can output the hexadecimal of the raw CBOR for the envelope:

```
nvelope format --type cbor $ALICE_KNOWS_BOB
```

```
d8c882d81865416c696365a1d818656b6e6f7773d81863426f62
```

### CBOR Diagnostic Notation Output

Or your can output the annotated CBOR diagnostic annotation of the envelope:

```
nvelope format --type diag $ALICE_KNOWS_BOB
```

```
200(   / envelope /
   [
      24("Alice"),   / leaf /
      {
         24("knows"):   / leaf /
         24("Bob")   / leaf /
      }
   ]
)
```

### Tree Output

The Envelope Tree Notation shows the structure of the envelope as a tree, including each element's digest:

```
nvelope format --type tree $ALICE_KNOWS_BOB
```

```
8955db5e NODE
    13941b48 subj "Alice"
    78d666eb ASSERTION
        db7dd21c pred "knows"
        13b74194 obj "Bob"
```

Note that internally, Envelope uses 256-bit SHA-256 digests, but the tree format only shows the first 32 bits of the digest.

With the `--hide-nodes` option, the structure of the Envelope is shown without digests and without the `NODE` element. This is useful for understanding the semantic structure of the Envelope:

```
nvelope format --type tree --hide-nodes $ALICE_KNOWS_BOB
```

```
"Alice"
    ASSERTION
        "knows"
        "Bob"
```

## Subject

The `subject type` subcommand creates a new envelope with a subject of the given type. You specify the data type of the subject, then the subject value itself.

```
nvelope subject type string "Hello."
```

```
ur:envelope/tpcsiyfdihjzjzjldmprrhtypk
```

When we feed this envelope back into the `format` comand, we get the envelope printed in Envelope Notation. This is why `"Hello."` is printed with quotes around it:

```
nvelope format ur:envelope/tpcsiyfdihjzjzjldmprrhtypk
```

```
"Hello."
```

Using the help command, you can see a listing of all the types supported:

```
nvelope subject type --help
```

```
...
Possible values:
- arid:     ARID: Apparently Random Identifier (ur:arid)
- bool:     Boolean (`true` or `false`)
- cbor:     CBOR data in hex
- data:     Binary byte string in hex
- date:     Date (ISO 8601)
- digest:   Cryptographic digest (ur:digest)
- envelope: Envelope (ur:envelope)
- known:    Known Value (number or string)
- number:   Numeric value,
- string:   UTF-8 String
- ur:       Uniform Resource (UR)
- uri:      URI
- uuid:     UUID
- wrapped:  Wrapped Envelope (ur:envelope)
...
```

## Extract

To extract the actual data of the envelope's subject, use the `extract` command:

```
nvelope extract string ur:envelope/tpcsiyfdihjzjzjldmprrhtypk
```

```
Hello.
```

In an envelope with assertions, the `extract` command just returns the subject without the assertions:

```bash
nvelope extract string $ALICE_KNOWS_BOB
```

```
Alice
```

If you want the subject returned as an envelope, use the `envelope` data type:

```bash
nvelope extract envelope $ALICE_KNOWS_BOB
```

```
ur:envelope/tpcsihfpjziniaihnsrsnyue
```

```bash
nvelope format ur:envelope/tpcsihfpjziniaihnsrsnyue
```

```
"Alice"
```
