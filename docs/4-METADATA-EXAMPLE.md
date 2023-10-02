# nvelope - Complex Metadata Example

Complex, tiered metadata can be added to an envelope.

In this example, we use ARIDs (Apparently Random Identifiers) to represent an author, whose known works may change over time, and a particular novel written by her, the data returned about which may change over time.

Assertions made about an ARID are considered part of a distributed set. Which assertions are returned depends on who resolves the ARID and when it is resolved. In other words, the referent of a ARID is mutable.

Start by creating an envelope that represents the author and what is known about her, including where to get more information using the author's ARID.

```bash
👉
AUTHOR=`nvelope subject type arid 9c747ace78a4c826392510dd6285551e7df4e5164729a1b36198e56e017666c8 | \
    nvelope assertion add pred-obj known dereferenceVia string LibraryOfCongress | \
    nvelope assertion add pred-obj known hasName string "Ayn Rand"`
nvelope format $AUTHOR
```

```
👈
ARID(9c747ace) [
    'dereferenceVia': "LibraryOfCongress"
    'hasName': "Ayn Rand"
]
```

Create two envelopes representing the name of the novel in two different languages, annotated with assertions that specify the language.

```bash
👉
NAME_EN=`nvelope subject type string "Atlas Shrugged" | \
    nvelope assertion add pred-obj known language string en`
nvelope format $NAME_EN
```

```
👈
"Atlas Shrugged" [
    'language': "en"
]
```

```bash
👉
NAME_ES=`nvelope subject type string "La rebelión de Atlas" | \
    nvelope assertion add pred-obj known language string es`
nvelope format $NAME_ES
```

```
👈
"La rebelión de Atlas" [
    'language': "es"
]
```

Create an envelope that specifies known information about the novel. This envelope embeds the previous envelopes we created for the author and the names of the work.

```bash
👉
WORK=`nvelope subject type arid 7fb90a9d96c07f39f75ea6acf392d79f241fac4ec0be2120f7c82489711e3e80 | \
    nvelope assertion add pred-obj known isA string Novel | \
    nvelope assertion add pred-obj string isbn string "9780451191144" | \
    nvelope assertion add pred-obj string author envelope $AUTHOR | \
    nvelope assertion add pred-obj known dereferenceVia string "LibraryOfCongress" | \
    nvelope assertion add pred-obj known hasName envelope $NAME_EN | \
    nvelope assertion add pred-obj known hasName envelope $NAME_ES`
nvelope format $WORK
```

```
👈
ARID(7fb90a9d) [
    'isA': "Novel"
    "author": ARID(9c747ace) [
        'dereferenceVia': "LibraryOfCongress"
        'hasName': "Ayn Rand"
    ]
    "isbn": "9780451191144"
    'dereferenceVia': "LibraryOfCongress"
    'hasName': "Atlas Shrugged" [
        'language': "en"
    ]
    'hasName': "La rebelión de Atlas" [
        'language': "es"
    ]
]
```

Create an envelope that refers to the digest of a particular digital embodiment of the novel, in EPUB format. Unlike ARIDs, which refer to mutable objects, this digest can only refer to exactly one unique digital object.

```bash
👉
BOOK_DATA="This is the entire book “Atlas Shrugged” in EPUB format."
BOOK_DIGEST=`nvelope generate digest $BOOK_DATA`
echo $BOOK_DIGEST
```

```
👈
ur:digest/hdcxdstihtykswvlcmamsrcwdtgdwscmtyemfdcyprclhtjzsameimtdbedidspkmuvtgdwzplwn
```

Create the final metadata object, which provides information about the object to which it refers, both as a general work and as a specific digital embodiment of that work.

```bash
👉
BOOK_METADATA=`nvelope subject type digest $BOOK_DIGEST | \
    nvelope assertion add pred-obj string "work" envelope $WORK | \
    nvelope assertion add pred-obj string format string EPUB | \
    nvelope assertion add pred-obj known dereferenceVia string "IPFS"`
nvelope format $BOOK_METADATA
```

```
👈
Digest(26d05af5) [
    "format": "EPUB"
    "work": ARID(7fb90a9d) [
        'isA': "Novel"
        "author": ARID(9c747ace) [
            'dereferenceVia': "LibraryOfCongress"
            'hasName': "Ayn Rand"
        ]
        "isbn": "9780451191144"
        'dereferenceVia': "LibraryOfCongress"
        'hasName': "Atlas Shrugged" [
            'language': "en"
        ]
        'hasName': "La rebelión de Atlas" [
            'language': "es"
        ]
    ]
    'dereferenceVia': "IPFS"
]
```
