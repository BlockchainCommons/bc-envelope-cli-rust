# `nvelope` - Verifiable Credential Example

In this example we build a permanent resident card, which the holder then redacts to reveal only selected information necessary to prove his identity.

Envelopes can also be built to support verifiable credentials, supporting the core functionality of DIDs.

John Smith's identifier:

```bash
👉
JOHN_ARID=`nvelope generate arid`
echo $JOHN_ARID
78bc30004776a3905bccb9b8a032cf722ceaf0bbfb1a49eaf3185fab5808cadc
```

A photo of John Smith:

```bash
👉
JOHN_IMAGE=`nvelope subject type string "John Smith Smiling" | \
nvelope assertion add pred-obj known note string "This is an image of John Smith." | \
nvelope assertion add pred-obj known dereferenceVia uri https://exampleledger.com/digest/36be30726befb65ca13b136ae29d8081f64792c2702415eb60ad1c56ed33c999`
nvelope format $JOHN_IMAGE
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
ISSUER=`nvelope subject type ur $STATE_ARID | \
    nvelope assertion add pred-obj known note string "Issued by the State of Example" | \
    nvelope assertion add pred-obj known dereferenceVia uri https://exampleledger.com/arid/04363d5ff99733bc0f1577baba440af1cf344ad9e454fad9d128c00fef6505e8`

BIRTH_COUNTRY=`nvelope subject type string bs | \
    nvelope assertion add pred-obj known note string "The Bahamas"`

HOLDER=`nvelope subject type ur $JOHN_ARID | \
    nvelope assertion add pred-obj known isA string Person | \
    nvelope assertion add pred-obj known isA string "Permanent Resident" | \
    nvelope assertion add pred-obj string givenName string JOHN | \
    nvelope assertion add pred-obj string familyName string SMITH | \
    nvelope assertion add pred-obj string sex string MALE | \
    nvelope assertion add pred-obj string birthDate date 1974-02-18 | \
    nvelope assertion add pred-obj string image envelope $JOHN_IMAGE | \
    nvelope assertion add pred-obj string lprCategory string C09 | \
    nvelope assertion add pred-obj string birthCountry envelope $BIRTH_COUNTRY | \
    nvelope assertion add pred-obj string residentSince date 2018-01-07`

JOHN_RESIDENT_CARD=`nvelope subject type ur $JOHN_ARID | \
    nvelope assertion add pred-obj known isA string "credential" | \
    nvelope assertion add pred-obj string "dateIssued" date 2022-04-27 | \
    nvelope assertion add pred-obj known issuer envelope $ISSUER | \
    nvelope assertion add pred-obj known holder envelope $HOLDER | \
    nvelope assertion add pred-obj known note string "The State of Example recognizes JOHN SMITH as a Permanent Resident." | \
    nvelope subject type wrapped | \
    nvelope sign --prvkeys $STATE_PRVKEYS --note "Made by the State of Example."`

nvelope format $JOHN_RESIDENT_CARD
```

```
👈
{
    ARID(72998c48) [
        'isA': "credential"
        "dateIssued": 2022-04-27
        'holder': ARID(72998c48) [
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
TARGET+=(`nvelope digest $JOHN_RESIDENT_CARD`)

# Reveal everything about the state's signature on the card
TARGET+=(`nvelope assertion find predicate known verifiedBy $JOHN_RESIDENT_CARD | nvelope digest --depth deep`)

# Reveal the top level of the card.
TARGET+=(`nvelope digest $JOHN_RESIDENT_CARD --depth shallow`)
CARD=`nvelope extract wrapped $JOHN_RESIDENT_CARD`
TARGET+=(`nvelope digest $CARD`)
TARGET+=(`nvelope extract envelope $CARD | nvelope digest`)

# Reveal everything about the `isA` and `issuer` assertions at the top level of the card.
TARGET+=(`nvelope assertion find predicate known isA $CARD | nvelope digest --depth deep`)
TARGET+=(`nvelope assertion find predicate known issuer $CARD | nvelope digest --depth deep`)

# Reveal the `holder` assertion on the card, but not any of its sub-assertions.
HOLDER=`nvelope assertion find predicate known holder $CARD`
TARGET+=(`nvelope digest --depth shallow $HOLDER`)

# Within the `holder` assertion, reveal everything about just the `givenName`, `familyName`, and `image` assertions.
HOLDER_OBJECT=`nvelope extract object $HOLDER`
TARGET+=(`nvelope assertion find predicate string givenName $HOLDER_OBJECT | nvelope digest --depth deep`)
TARGET+=(`nvelope assertion find predicate string familyName $HOLDER_OBJECT | nvelope digest --depth deep`)
TARGET+=(`nvelope assertion find predicate string image $HOLDER_OBJECT | nvelope digest --depth deep`)

# Perform the elision
ELIDED_CARD=`nvelope elide revealing "$TARGET" $JOHN_RESIDENT_CARD`

# Show the elided card
nvelope format $ELIDED_CARD
```

```
👈
{
    ARID(72998c48) [
        'isA': "credential"
        'holder': ARID(72998c48) [
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
nvelope digest $JOHN_RESIDENT_CARD; nvelope digest $ELIDED_CARD
```

```
👈
ur:digest/hdcxlpkpltrebkbzhneojntbjtaeayrdclpdbwhdonglkpiyhnoemklpcpsskogufgtkzmtywsid
ur:digest/hdcxlpkpltrebkbzhneojntbjtaeayrdclpdbwhdonglkpiyhnoemklpcpsskogufgtkzmtywsid
```

Note that the state's signature on the elided card still verifies.

```bash
👉
nvelope verify --silent $ELIDED_CARD --pubkeys $STATE_PUBKEYS
```
