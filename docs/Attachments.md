# `envelope` - Attachments

Attachments are defined in [BCR-2023-006](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-006-envelope-attachment.md). They are a standardized way to add discoverable third-party data to an envelope.

Attachments are assertions that can be built up by using various invocations of the `envelope` command line tool, but since they have a specific format, the `envelope` tool provides shortcuts for creating and working with them.

## Table of Contents

- [`envelope` - Attachments](#envelope---attachments)
  - [Table of Contents](#table-of-contents)
  - [Attachment Subcommands](#attachment-subcommands)
  - [Creating an Attachment](#creating-an-attachment)
  - [Querying an Attachment](#querying-an-attachment)
  - [Adding an Attachment](#adding-an-attachment)
  - [Multiple Attachments](#multiple-attachments)
  - [Enumerating Attachments](#enumerating-attachments)
  - [Finding Attachments](#finding-attachments)

## Attachment Subcommands

```
envelope attachment --help

│ Work with the envelope's attachments
│
│ Usage: envelope attachment <COMMAND>
│
│ Commands:
│   add          Add an assertion to the given envelope
│   all          Retrieve all the envelope's assertions
│   at           Get the attachment at the specified index
│   conforms-to  Get the optional conformance of the attachment
│   count        Print the count of the envelope's assertions
│   create       Create an attachment
│   payload      Get the payload of the attachment
│   vendor       Get the vendor of the attachment
│   find         Retrieve attachments having the specified attributes
│   help         Print this message or the help of the given subcommand(s)
```

## Creating an Attachment

The `attachment create` command lets you create an attachment by specifying its fields: `vendor`, `conforms-to` (optional), and `payload`.

```
envelope attachment create --help

│ Create an attachment
│
│ Usage: envelope attachment create [OPTIONS] <VENDOR> [PAYLOAD]
│
│ Arguments:
│   <VENDOR>
│           The vendor of the attachment. Usually a reverse domain name
│
│   [PAYLOAD]
│           The payload of the attachment. Entirely defined by the vendor.
│
│           If not supplied, it is read from stdin.
│
│ Options:
│   -c, --conforms-to <CONFORMS_TO>
│           An optional `conforms-to` value of the attachment. Usually a URI
│
│   -h, --help
│           Print help (see a summary with '-h')
│
│   -V, --version
│           Print version
```

But first we need an envelope that is our attachment "payload", i.e., the vendor-specific data that we want to attach to an envelope. This can any possible envelope of arbitrary complexity, but here we'll just use a simple string:

```
PAYLOAD_ENVELOPE=`envelope subject type string "this-is-the-payload"`
```

Now we create our attachment with our payload, and specify the vendor and conformance strings:

```
VENDOR="com.example"
CONFORMS_TO="https://example.com/attachment"
ATTACHMENT=`envelope attachment create $VENDOR --conforms-to $CONFORMS_TO $PAYLOAD_ENVELOPE`
```

Here's what our attachment looks like in envelope notation:

```
envelope format $ATTACHMENT

│ 'attachment': {
│     "this-is-the-payload"
│ } [
│     'conformsTo': "https://example.com/attachment"
│     'vendor': "com.example"
│ ]
```

Notice that attachments are "bare assertions," i.e., assertions without a subject. Recall that assertions have a predicate and an object: in this case the predicate is the known value `'attachment'` and the object is the payload, which is wrapped and then has its own `'vendor'` and `'conformsTo'` assertions added to it.

## Querying an Attachment

Now that we have an attachment, we can query it for its vendor:

```
envelope attachment vendor $ATTACHMENT

│ com.example
```

Or for its conformance:

```
envelope attachment conforms-to $ATTACHMENT

│ https://example.com/attachment
```

Or for its payload:

```
envelope attachment payload $ATTACHMENT

│ ur:envelope/tpsojkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsieghaoksaa
```

Note that since the payload is itself an envelope which can be arbitrarily complex, it is returned as a UR. So we can take the result of the above command and extract its subject as a string:

```
envelope extract string `envelope attachment payload $ATTACHMENT`

│ this-is-the-payload
```

## Adding an Attachment

Now that we have an attachment, let's create a simple envelope to add it to:

```
ENVELOPE=`envelope subject type string "this-is-the-envelope"`
```

Since we already have an attachment, we can add it to our envelope using the `attachment add envelope` command:

```
ENVELOPE_WITH_ATTACHMENT=`envelope attachment add envelope $ATTACHMENT $ENVELOPE`

envelope format $ENVELOPE_WITH_ATTACHMENT

│ "this-is-the-envelope" [
│     'attachment': {
│         "this-is-the-payload"
│     } [
│         'conformsTo': "https://example.com/attachment"
│         'vendor': "com.example"
│     ]
│ ]
```

## Multiple Attachments

Let's say the vendor releases a new version of the attachment spec. But for backwards compatibility, they want to support both the old and the new version. So they create a new attachment with the new version, and add it to the envelope. we start by creating the version 2 payload:

```
PAYLOAD_ENVELOPE_V2=`envelope subject type string "this-is-the-payload-v2"`
```

In the previous example we created the attachment and then added it to the envelope, but we can also do it in one step by using the `attachment add components` command:

```
ENVELOPE_WITH_TWO_ATTACHMENTS=`envelope attachment add components "com.example" --conforms-to "https://example.com/attachment-v2" $PAYLOAD_ENVELOPE_V2 $ENVELOPE_WITH_ATTACHMENT`

envelope format $ENVELOPE_WITH_TWO_ATTACHMENTS

│ "this-is-the-envelope" [
│     'attachment': {
│         "this-is-the-payload"
│     } [
│         'conformsTo': "https://example.com/attachment"
│         'vendor': "com.example"
│     ]
│     'attachment': {
│         "this-is-the-payload-v2"
│     } [
│         'conformsTo': "https://example.com/attachment-v2"
│         'vendor': "com.example"
│     ]
│ ]
```

## Enumerating Attachments

The `count`, `all`, and `at` commands let you enumerate the attachments in an envelope:

```
envelope attachment count $ENVELOPE_WITH_TWO_ATTACHMENTS

│ 2
```

```
envelope attachment all $ENVELOPE_WITH_TWO_ATTACHMENTS

│ ur:envelope/oycseylstpsptpsojkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsieoycseetpsoksckisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjyoycseotpsojeiajljndmihkshsjnjojziheyttoezm
│ ur:envelope/oycseylstpsptpsokojyisinjkdpinjkdpjyisihdpjohskkjzjlhsiedpkoeyoycseetpsoksclisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjydpkoeyoycseotpsojeiajljndmihkshsjnjojzihsslriaki
```

```
envelope attachment at 0 $ENVELOPE_WITH_TWO_ATTACHMENTS

│ ur:envelope/oycseylstpsptpsojkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsieoycseetpsoksckisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjyoycseotpsojeiajljndmihkshsjnjojziheyttoezm
```

## Finding Attachments

The `find` command lets you find attachments that match a given set of attributes. In these examples we pipe the results to `wc -l` to count the number of attachments that match the given attributes.

There are two attachments in our envelope, and both were added by the same vendor, so we can find them both by specifying the vendor:

```
envelope attachment find --vendor "com.example" $ENVELOPE_WITH_TWO_ATTACHMENTS | wc -l

│ 2
```

Each of these two attachments have different conformance URIs, so we can just find the version 2 attachment by specifying its conformance URI:

```
envelope attachment find --conforms-to "https://example.com/attachment-v2" $ENVELOPE_WITH_TWO_ATTACHMENTS | wc -l

│ 1
```
