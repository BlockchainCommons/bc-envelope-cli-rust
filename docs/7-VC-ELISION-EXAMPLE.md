# envelope - Elision/Redaction Example

In this example, The employer of an employee with a continuing education credential uses elision to warrant to a third-party that their employee has such a credential, without revealing anything else about the employee.

This example goes into detail about creating the target set for elision, building it up piece by piece.

**See Associated Video:**

[![Gordian Envelope CLI - 3 - Elision in Detail](https://img.youtube.com/vi/3G70mUYQB18/mqdefault.jpg)](https://www.youtube.com/watch?v=3G70mUYQB18)

First we need keys that represent the Example Electrical Engineering Board.

```bash
👉
BOARD_PRVKEYS="ur:crypto-prvkeys/hdcxynlntpsbfrbgjkcetpzorohgsafsihcnhyrtoebzwegtvyzolbgtdaskcsldfgadtldmrkld"
BOARD_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxzcjpcycfstoyengahyzecppefwvtghmstkyklsoeiovtfzasbdbakepdseaehsiatansgrhdcxkelsaetygrwtdtwzytkoielytschleptdsmwahwtvlwlwdpmadoydwltmsasidfrhnasptgm"

EMPLOYER_PRVKEYS="ur:crypto-prvkeys/hdcxpkyneedreyhyvshfmygwplrfrhclfwenkoetwnvagyescezsnnsobyfhtkghgypsrhdmjnko"
EMPLOYER_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxdnknjkmsmstypasfonchmyrktpgdesdasarlpyselbhnfenesofmplkopsntnnmotansgrhdcxbwwtrpwfvdjnhlrhlejolgwfhykpndknswdwlflgotiofdtpcsgmdljnihsgbwksathtplcp"
```

Now we can compose the credential.

One of the fields of the credential is a CBOR array of strings. The Envelope type can handle raw CBOR in any position but the `envelope` command line tool isn't for composing general purpose CBOR, so we turn to the [cbor-cli tool](https://www.npmjs.com/package/cbor-cli) to do that:

```
👉
cbor
cbor> ["Subject 1", "Subject 2"]
0x82695375626a6563742031695375626a6563742032
```

We use the CBOR hex with the `topics` assertion below:

```bash
👉
CREDENTIAL=`envelope subject --arid 4676635a6e6068c2ef3ffd8ff726dd401fd341036e920f136a1d8af5e829496d |
    envelope assertion --known isA "Certificate of Completion" |
    envelope assertion --known issuer "Example Electrical Engineering Board" |
    envelope assertion --known controller "Example Electrical Engineering Board" |
    envelope assertion firstName James |
    envelope assertion lastName Maxwell |
    envelope assertion --string issueDate --date 2020-01-01 |
    envelope assertion --string expirationDate --date 2028-01-01 |
    envelope assertion photo "This is James Maxwell's photo." |
    envelope assertion certificateNumber 123-456-789 |
    envelope assertion subject "RF and Microwave Engineering" |
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
        "certificateNumber": "123-456-789"
        "continuingEducationUnits": 1.5
        "expirationDate": 2028-01-01
        "firstName": "James"
        "issueDate": 2020-01-01
        "lastName": "Maxwell"
        "photo": "This is James Maxwell's photo."
        "professionalDevelopmentHours": 15
        "subject": "RF and Microwave Engineering"
        "topics": ["Subject 1", "Subject 2"]
        controller: "Example Electrical Engineering Board"
        issuer: "Example Electrical Engineering Board"
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

Every part of an envelope generates a digest, and these together form a Merkle tree. So when eliding a document, we can decide what to remove or reveal by identifying a subset of all the digests that make up the tree. This set is known as the *target*. Normally we would create the target and then perform the elision in a single operation, but in this example we are going to build up the target in increments, showing the result of each step.

Here we create a shell array variable to hold our target set of digests. It starts out empty.

```bash
👉
TARGET=()
```

If we use this empty target to elide the target using the `elide revealing` command:

```bash
👉
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

...then the entire envelope will be elided:

```
👈
ELIDED
```

We've essentially said, "Elide everything." Obviously this isn't very useful, so we now start to work our way down the tree to the parts we want to reveal.

## Revealing the Top-Level Structure

The first digest we need is the top-level digest of the envelope. This reveals the "macro structure" of the envelope.

```bash
👉
TARGET+=`envelope digest $CREDENTIAL`
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
ELIDED [
    ELIDED (2)
]
```

This shows us that the envelope has a subject, which is still elided, and two assertions, both of which are still elided. The subject is the actual credential, and the assertions are the signature and the note.

## Revealing the Signature

To reveal the two assertions, we iterate through them and add their "deep digests" to the target. Using `envelope digest --deep` means that *everying* about the revealed assertions will be revealed, including any assertions they may have, recursively.

```bash
👉
TARGET+=(`envelope assertion at 0 $CREDENTIAL | envelope digest --deep`)
TARGET+=(`envelope assertion at 1 $CREDENTIAL | envelope digest --deep`)
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
ELIDED [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

There are two important things to note in the above commands, one dealing with the shell commands and one dealing with the nature of the Envelope type.

Regarding the shell, in the above commands we needed the outer parentheses to ensure that the shell adds each of the digests to the array as a separate element, instead of adding them all as a single element.

Regarding the envelope type, the `assertion at N` command will retrieve the assertion at the index it is stored in the envelope structure, *not* the order it prints in envelope notation. In envelope notation, the assertions are in lexicographic order with known values coming last. But in the envelope structure itself, assertions are always in lexicographic order by *digest*. So if you use the `envelope format` command to print an envelope, you usually won't get the same assertion at a given index using the `assertion at N` command. Generally the `assertion at N` command is for when you want to iterate through all assertions. If you want to work with a specific assertion, use the `assertion find` to locate it by the content of its predicate or object.

At this point, if one had the proper public keys, the receiver of this redacted credential could verify the signature, even without knowing anything else about the contents of the credential.

```bash
👉
envelope verify --silent $REDACTED_CREDENTIAL --pubkeys $BOARD_PUBKEYS
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

## Revealing the Subject

The subject of the envelope, containing all the holder's information is still elided. So now we add the subject itself to the target:

```bash
👉
TARGET+=`envelope extract $CREDENTIAL --envelope | envelope digest`
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
{
    ELIDED
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

Comparing to the results of the previous step, we see a new pair of braces has appeared. This is because the subject of the document is *another* envelope that has been wrapped in its entirety to be signed. Notice the invocation of the `envelope subject --wrapped` command at the start of this example above.

## Revealing the Content

So now we need to reveal the unwrapped content:

```bash
👉
CONTENT=`envelope extract --wrapped $CREDENTIAL`
TARGET+=`envelope digest $CONTENT`
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
{
    ELIDED [
        ELIDED (13)
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

Now it looks like we're getting somewhere! The wrapped envelope has a still-elided subject (the holder's ARID) and ten assertions, all of which are still currently elided.

## Revealing the ARID

We want to reveal the ARID representing the issuing authority's unique reference to the credential holder. This is because the warranty the employer is making is that a specific identifiable employee has the credential, *without* actually revealing their identity. This allows the entire document to be identified and unredacted should a dispute ever arise.

```bash
👉
TARGET+=`envelope extract --envelope $CONTENT | envelope digest`
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
{
    ARID(4676635a) [
        ELIDED (13)
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

## Revealing the Claims

The only actual assertions we want to reveal are, `isA`, `issuer`, `subject` and `expirationDate`, so we do this by finding those specific assertions by their predicate. The `envelope digest --shallow` command returns just a necessary set of attributes to reveal the assertion, its predicate, and its object (yes, all three of them need to be revealed) but *not* any deeper assertions on them.

```bash
👉
TARGET+=(`envelope assertion find --known isA $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find --known issuer $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "subject" $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "expirationDate" $CONTENT | envelope digest --shallow`)
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`; envelope $REDACTED_CREDENTIAL
```

```
👈
{
    ARID(4676635a) [
        isA: "Certificate of Completion"
        "expirationDate": 2028-01-01
        "subject": "RF and Microwave Engineering"
        issuer: "Example Electrical Engineering Board"
        ELIDED (9)
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

Finally, the employer wants to enclose this envelope, add some non-repudiable assertions of it's own, then sign it. This is the employer's *warranty*.

```bash
👉
WARRANTY=`envelope subject --wrapped $REDACTED_CREDENTIAL |
    envelope assertion --string employeeHiredDate --date 2022-01-01 |
    envelope assertion employeeStatus active |
    envelope subject --wrapped |
    envelope assertion --known note "Signed by Employer Corp." |
    envelope sign --prvkeys $EMPLOYER_PRVKEYS`
envelope $WARRANTY
```

```
👈
{
    {
        {
            ARID(4676635a) [
                isA: "Certificate of Completion"
                "expirationDate": 2028-01-01
                "subject": "RF and Microwave Engineering"
                issuer: "Example Electrical Engineering Board"
                ELIDED (9)
            ]
        } [
            note: "Signed by Example Electrical Engineering Board"
            verifiedBy: Signature
        ]
    } [
        "employeeHiredDate": 2022-01-01
        "employeeStatus": "active"
    ]
} [
    note: "Signed by Employer Corp."
    verifiedBy: Signature
]
```

If we take all the command lines from above and compose them into a single script that starts with the credential and ends with the warranty, we have:

```bash
👉
TARGET=()
TARGET+=`envelope digest $CREDENTIAL`
TARGET+=(`envelope assertion at 0 $CREDENTIAL | envelope digest --deep`)
TARGET+=(`envelope assertion at 1 $CREDENTIAL | envelope digest --deep`)
TARGET+=`envelope extract $CREDENTIAL --envelope | envelope digest`
CONTENT=`envelope extract --wrapped $CREDENTIAL`
TARGET+=`envelope digest $CONTENT`
TARGET+=`envelope extract --envelope $CONTENT | envelope digest`
TARGET+=(`envelope assertion find --known isA $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find --known issuer $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "firstName" $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "lastName" $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "subject" $CONTENT | envelope digest --shallow`)
TARGET+=(`envelope assertion find "expirationDate" $CONTENT | envelope digest --shallow`)
REDACTED_CREDENTIAL=`envelope elide revealing $CREDENTIAL $TARGET`
WARRANTY=`envelope subject --wrapped $REDACTED_CREDENTIAL |
    envelope assertion --string employeeHiredDate --date 2022-01-01 |
    envelope assertion employeeStatus active |
    envelope subject --wrapped |
    envelope assertion --known note "Signed by Employer Corp." |
    envelope sign --prvkeys $EMPLOYER_PRVKEYS`
```

## Compression and Encryption

The same command that is used to elide a target set of digests can also be used to compress or encrypt the target:

```
👉
envelope elide revealing --compress $CREDENTIAL $TARGET | envelope
```

```
👈
{
    ARID(4676635a) [
        isA: "Certificate of Completion"
        "expirationDate": 2028-01-01
        "firstName": "James"
        "lastName": "Maxwell"
        "subject": "RF and Microwave Engineering"
        issuer: "Example Electrical Engineering Board"
        COMPRESSED (7)
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```

When you encrypt, you must also supply a symmetric key with the `--key` option.

```
👉
envelope elide revealing --encrypt --key ur:crypto-key/hdcxcnqzoeuobzdksphpfxonrlkemsislfloahurgygojnkblfktrkvdpyrklykbiawynncmtlpl $CREDENTIAL $TARGET | envelope
```

```
👈
{
    ARID(4676635a) [
        isA: "Certificate of Completion"
        "expirationDate": 2028-01-01
        "firstName": "James"
        "lastName": "Maxwell"
        "subject": "RF and Microwave Engineering"
        issuer: "Example Electrical Engineering Board"
        ENCRYPTED (7)
    ]
} [
    note: "Signed by Example Electrical Engineering Board"
    verifiedBy: Signature
]
```
