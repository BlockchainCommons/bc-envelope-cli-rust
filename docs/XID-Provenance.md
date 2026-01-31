# XID Provenance Marks

XID documents can include provenance marks that provide a verifiable chain of custody and state transitions. A provenance mark is a cryptographic proof that shows the document's history, using a hash-based chain structure. Each mark contains:

- A sequence number (starting from 0 for the genesis mark)
- A date timestamp
- Optional structured information (as any UR type)
- A chain code linking to previous marks

Provenance marks are particularly useful for tracking XID document updates over time, ensuring that modifications follow a verifiable sequence.

## Creating XID Documents with Provenance Marks

When creating a new XID document, you can include a genesis provenance mark using the `--generator` option:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --generator include | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(53929e33) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 1
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--generator include` option includes the provenance generator in the document. This generator contains the cryptographic state needed to create subsequent marks in the chain. The generator is included as a salted assertion on the provenance mark itself.

### Generator Options

The `--generator` option accepts several values:

- `omit` (default): Do not include a provenance mark
- `include`: Include a provenance mark with the generator in plaintext
- `encrypt`: Include a provenance mark with an encrypted generator (requires `--encrypt-password`)
- `elide`: Not supported for new documents (will produce an error)

### Custom Dates and Information

When creating a genesis mark, you can specify a custom date and attach structured information:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
DIGEST_UR=`envelope generate digest "Hello"`
envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15 --info $DIGEST_UR | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(2bbbd1e1) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 1
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--date` option accepts ISO 8601 format dates (e.g., "2025-01-15"). If omitted, the current date is used.

The `--info` option accepts any UR type (envelope, digest, ARID, etc.). This information is embedded in the provenance mark and can be used to attach context or metadata to the mark.

### Encrypted Generators

For additional security, the provenance generator can be encrypted:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
envelope xid new $ALICE_PRVKEYS --generator encrypt --encrypt-password "secret" | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(10bd3a28) [
│         {
│             'provenanceGenerator': ENCRYPTED [
│                 'hasSecret': EncryptedKey(Argon2id)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Encrypted generators protect the cryptographic state while still allowing the provenance mark itself to be read. To advance the mark later, you'll need to provide the decryption password.

## `xid provenance get`: Extract the Provenance Mark

The `xid provenance get` command extracts the provenance mark from a XID document and returns it as a standalone UR:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include`
envelope xid provenance get $XID_WITH_PROV

│ ur:provenance/lfaxhdimpkwlsektpsataagwutlpdspmbtjkfrprmoptdtlrftbwdkdlvalrytchlrtdsavtplvsltsgbdjlwyhfvawejlvtutfmondavarnylstbncplymtatmkjkpylrclpkrlaoisgoinmkpyssmydlotfrmhdszeftmwgsluclrerlnlwtemcholrlrojpiyhksplsldztspihlkbentgwfsdnmomtax
```

If the document does not have a provenance mark, the command returns an empty string:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_NO_PROV=`envelope xid new $ALICE_PRVKEYS`
envelope xid provenance get $XID_NO_PROV

│
```

This command works with both plaintext and encrypted generators. It also supports signature verification with the `--verify` option (see [Signed XID Documents](XID-Signing.md)).

## `xid provenance next`: Advance the Provenance Mark

The `xid provenance next` command advances the provenance mark to the next state in the chain. This creates a new mark with an incremented sequence number and a hash linking to the previous mark:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 $XID_WITH_PROV | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(bf27fca8) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Notice that the provenance mark's identifier has changed (from `53929e33` to `bf27fca8` in this example), and the `next-seq` value in the generator has incremented from 1 to 2.

### Using an Embedded Generator

When the XID document contains an embedded generator (either plaintext or encrypted), the `next` command uses it automatically:

- **Plaintext generators**: No password needed
- **Encrypted generators**: Require `--password` to decrypt

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
XID_ENC_GEN=`envelope xid new $ALICE_PRVKEYS --generator encrypt --encrypt-password "secret" --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 --password "secret" --encrypt-password "secret" $XID_ENC_GEN | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(8205b945) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

Note: When advancing with an encrypted generator, the `--password` option decrypts it for use, and `--encrypt-password` re-encrypts it in the output. If you want to change from encrypted to plaintext, you can omit `--encrypt-password`.

### Attaching Information to New Marks

Like the genesis mark, you can attach custom dates and structured information when advancing:

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ARID_UR=`envelope generate arid`
XID_WITH_PROV=`envelope xid new $ALICE_PRVKEYS --generator include --date 2025-01-15`
envelope xid provenance next --date 2025-01-20 --info $ARID_UR $XID_WITH_PROV | envelope format

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         {
│             'privateKey': PrivateKeys(8624d38b, SigningPrivateKey(096547df, SchnorrPrivateKey(74343f20)), EncapsulationPrivateKey(d8e2032d, X25519PrivateKey(d8e2032d)))
│         } [
│             'salt': Salt
│         ]
│         'allow': 'All'
│     ]
│     'provenance': ProvenanceMark(79a0d8f3) [
│         {
│             'provenanceGenerator': Bytes(32) [
│                 'isA': "provenance-generator"
│                 "next-seq": 2
│                 "res": 3
│                 "rng-state": Bytes(32)
│                 "seed": Bytes(32)
│             ]
│         } [
│             'salt': Salt
│         ]
│     ]
│ ]
```

The `--info` parameter can be any UR type, making it flexible for various use cases such as attaching transaction references, update notes, or other contextual data.
