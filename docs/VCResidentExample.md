# `envelope` - Verifiable Credential Example

In this example we build a permanent resident card, which the holder then redacts to reveal only selected information necessary to prove his identity.

Envelopes can also be built to support verifiable credentials, supporting the core functionality of DIDs.

John Smith's identifier:

```bash
👉
JOHN_ARID=`envelope generate arid`
echo $JOHN_ARID
ur:arid/hdcxrowefshyrpbapewtbwjowpvlimztemamzevoktdwdytnvldloltadeihwewschlflutinbfm
```

A photo of John Smith:

```bash
👉
JOHN_IMAGE=`envelope subject type string "John Smith Smiling" | \
envelope assertion add pred-obj known note string "This is an image of John Smith." | \
envelope assertion add pred-obj known dereferenceVia uri "https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999"`
envelope format $JOHN_IMAGE
```

```
👈
"John Smith Smiling" [
    'dereferenceVia': URI(https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999)
    'note': "This is an image of John Smith."
]
```

John Smith's Permanent Resident Card issued by the State of Example:

```bash
👉
STATE_ARID="ur:arid/hdcxaaenfsheytmseorfbsbzktrdrdfybkwntkeegetaveghzstattdertbswsihahvspllbghcp"

ISSUER=`envelope subject type ur $STATE_ARID | \
    envelope assertion add pred-obj known note string "Issued by the State of Example" | \
    envelope assertion add pred-obj known dereferenceVia uri "https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8"`

BIRTH_COUNTRY=`envelope subject type string "bs" | \
    envelope assertion add pred-obj known note string "The Bahamas"`

HOLDER=`envelope subject type ur $JOHN_ARID | \
    envelope assertion add pred-obj known isA string "Person" | \
    envelope assertion add pred-obj known isA string "Permanent Resident" | \
    envelope assertion add pred-obj string "givenName" string "JOHN" | \
    envelope assertion add pred-obj string "familyName" string "SMITH" | \
    envelope assertion add pred-obj string "sex" string "MALE" | \
    envelope assertion add pred-obj string "birthDate" date "1974-02-18" | \
    envelope assertion add pred-obj string "image" envelope $JOHN_IMAGE | \
    envelope assertion add pred-obj string "lprCategory" string "C09" | \
    envelope assertion add pred-obj string "birthCountry" envelope $BIRTH_COUNTRY | \
    envelope assertion add pred-obj string "residentSince" date "2018-01-07"`

JOHN_RESIDENT_CARD=`envelope subject type ur $JOHN_ARID | \
    envelope assertion add pred-obj known isA string "credential" | \
    envelope assertion add pred-obj string "dateIssued" date "2022-04-27" | \
    envelope assertion add pred-obj known issuer envelope $ISSUER | \
    envelope assertion add pred-obj known holder envelope $HOLDER | \
    envelope assertion add pred-obj known note string "The State of Example recognizes JOHN SMITH as a Permanent Resident." | \
    envelope subject type wrapped | \
    envelope sign --prvkeys $STATE_PRVKEYS --note "Made by the State of Example."`

envelope format $JOHN_RESIDENT_CARD
```

```
👈
{
    ARID(b8ed3d5e) [
        'isA': "credential"
        "dateIssued": 2022-04-27
        'holder': ARID(b8ed3d5e) [
            'isA': "Permanent Resident"
            'isA': "Person"
            "birthCountry": "bs" [
                'note': "The Bahamas"
            ]
            "birthDate": 1974-02-18
            "familyName": "SMITH"
            "givenName": "JOHN"
            "image": "John Smith Smiling" [
                'dereferenceVia': URI(https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999)
                'note': "This is an image of John Smith."
            ]
            "lprCategory": "C09"
            "residentSince": 2018-01-07
            "sex": "MALE"
        ]
        'issuer': ARID(04363d5f) [
            'dereferenceVia': URI(https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8)
            'note': "Issued by the State of Example"
        ]
        'note': "The State of Example recognizes JOHN SMITH as a Permanent Resident."
    ]
} [
    'verifiedBy': Signature [
        'note': "Made by the State of Example."
    ]
]
```

John wishes to identify himself to a third party using his government-issued credential, but does not wish to reveal more than his name, his photo, and the fact that the state has verified his identity.

Redaction is performed by building a set of digests that will be revealed. All digests not present in the reveal-set will be replaced with elision markers containing only the hash of what has been elided, thus preserving the hash tree including revealed signatures. If a higher-level object is elided, then everything it contains will also be elided, so if a deeper object is to be revealed, all of its parent objects also need to be revealed, even though not everything *about* the parent objects must be revealed.

```bash
👉
# Start a target set.
TARGET=()

# Reveal the card. Without this, everything about the card would be elided.
TARGET+=(`envelope digest $JOHN_RESIDENT_CARD`)

# Reveal everything about the state's signature on the card
TARGET+=(`envelope assertion find predicate known verifiedBy $JOHN_RESIDENT_CARD | envelope digest --depth deep`)

# Reveal the top level of the card.
TARGET+=(`envelope digest $JOHN_RESIDENT_CARD --depth shallow`)
CARD=`envelope extract wrapped $JOHN_RESIDENT_CARD`
TARGET+=(`envelope digest $CARD`)
TARGET+=(`envelope extract envelope $CARD | envelope digest`)

# Reveal everything about the `isA` and `issuer` assertions at the top level of the card.
TARGET+=(`envelope assertion find predicate known isA $CARD | envelope digest --depth deep`)
TARGET+=(`envelope assertion find predicate known issuer $CARD | envelope digest --depth deep`)

# Reveal the `holder` assertion on the card, but not any of its sub-assertions.
HOLDER=`envelope assertion find predicate known holder $CARD`
TARGET+=(`envelope digest --depth shallow $HOLDER`)

# Within the `holder` assertion, reveal everything about just the `givenName`, `familyName`, and `image` assertions.
HOLDER_OBJECT=`envelope extract object $HOLDER`
TARGET+=(`envelope assertion find predicate string givenName $HOLDER_OBJECT | envelope digest --depth deep`)
TARGET+=(`envelope assertion find predicate string familyName $HOLDER_OBJECT | envelope digest --depth deep`)
TARGET+=(`envelope assertion find predicate string image $HOLDER_OBJECT | envelope digest --depth deep`)

# Perform the elision
ELIDED_CARD=`envelope elide revealing "$TARGET" $JOHN_RESIDENT_CARD`

# Show the elided card
envelope format $ELIDED_CARD
```

```
👈
{
    ARID(b8ed3d5e) [
        'isA': "credential"
        'holder': ARID(b8ed3d5e) [
            "familyName": "SMITH"
            "givenName": "JOHN"
            "image": "John Smith Smiling" [
                'dereferenceVia': URI(https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999)
                'note': "This is an image of John Smith."
            ]
            ELIDED (7)
        ]
        'issuer': ARID(04363d5f) [
            'dereferenceVia': URI(https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8)
            'note': "Issued by the State of Example"
        ]
        ELIDED (2)
    ]
} [
    'verifiedBy': Signature [
        'note': "Made by the State of Example."
    ]
]
```

Print the number of digests in the target set.

```bash
👉
echo ${#TARGET[@]}
```

```
👈
46
```

Remove duplicates to get the number of unique digests in the target. Duplicates don't harm anything, but they might point to opportunities for optimization.

```bash
👉
UNIQUE_DIGESTS=( `for i in ${TARGET[@]}; do echo $i; done | sort -u` )
echo ${#UNIQUE_DIGESTS[@]}
```

```
👈
40
```

Note that the original card and the elided card have the same digest.

```bash
👉
envelope digest $JOHN_RESIDENT_CARD; envelope digest $ELIDED_CARD
```

```
👈
ur:digest/hdcxstcnlogtmnuraesttbnngojeryeebswtqznsbbgdjnfemucwonhlbbhfrywyadlyhpcftkft
ur:digest/hdcxstcnlogtmnuraesttbnngojeryeebswtqznsbbgdjnfemucwonhlbbhfrywyadlyhpcftkft
```

Note that the state's signature on the elided card still verifies.

```bash
👉
envelope verify --silent $ELIDED_CARD --pubkeys $STATE_PUBKEYS
```
