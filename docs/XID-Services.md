# XID Services

```
envelope xid service --help

│ Work with a XID document's services
│
│ Usage: envelope xid service <COMMAND>
│
│ Commands:
│   add     Add a service to the XID document
│   all     Retrieve all the XID services
│   at      Retrieve the XID Document's service at the given index
│   count   Print the count of the XID document's services
│   find    Find all XID services matching the given criteria
│   remove  Remove the given service from the XID document
│   update  Updates the permissions, delegates, keys, capability identifer, or name of a service in a XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

Services are URI endpoints along with the keys, delegates, and permissions that are allowed to use them.

The keys and delegates in a Service declaration are references to keys and delegates that must already exist in the XID document.

```
ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`
CAROL_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxmorsytadihzswmckyltauyolecmevychhlwmtylbhsmdptfdrtuewnjtdkmnmkretansgehdcxhentsejphsfwclylihbwroaoisptaskegrimyldebecsdrrtbdlrrslazeursspmldtkmdds"
CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEYS`
```

## `xid service add`: Add a Service to a XID Document

Alice creates a basic XID document.

```
ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Alice adds Bob as a delegate.

```
BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`
ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

Alice adds a secure messaging service.

- Alice named the service "Messaging".
- The service is at `https://messaging.example.com`.
- The service provides the capability `com.example.messaging`.
- People can use the service to contact Alice (or Bob acting on behalf of Alice).
- The permissions on the service limit Alice and Bob's public keys to encrypting to Alice, and verifying signatures.
- Alice and Bob, as the holders of the private keys, can decrypt messages sent to them and sign messages they send.

```
ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Messaging' \
    --capability 'com.example.messaging' \
    --allow 'sign' \
    --allow 'encrypt' \
    --key $ALICE_PUBKEYS \
    --delegate $BOB_XID_DOC \
    "https://messaging.example.com" \
    $ALICE_XID_DOC`

envelope format $ALICE_XID_DOC_WITH_SERVICE

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://messaging.example.com) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'capability': "com.example.messaging"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Messaging"
│     ]
│ ]
```

Alice adds a second service for retrieving her status.

- Alice named the service "Status".
- The service is at `https://status.example.com/alice`.
- The service provides the capability `com.example.status`.
- The public key is the only one used to verify Alice's signatures.
- Alice, as the holder of the private key, can sign her status updates.

```
ALICE_XID_DOC_WITH_SERVICE=`envelope xid service add \
    --name 'Status' \
    --capability 'com.example.status' \
    --allow 'sign' \
    --key $ALICE_PUBKEYS \
    "https://status.example.com/alice" \
    $ALICE_XID_DOC_WITH_SERVICE`

envelope format $ALICE_XID_DOC_WITH_SERVICE

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://messaging.example.com) [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│         'capability': "com.example.messaging"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Messaging"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'Sign'
│         'capability': "com.example.status"
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

## `xid service count`: Count the Number of Services in a XID Document

```
envelope xid service count $ALICE_XID_DOC_WITH_SERVICE
```

```
2
```

## `xid service at`: Return the Service at the Specified Index

The indexes are zero-based, and in the order the service assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid service at 0 $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

```
envelope xid service at 1 $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

## `xid service all`: List All Services in a XID Document

```
envelope xid service all $ALICE_XID_DOC_WITH_SERVICE

│ ur:envelope/lttpsotpcxkscaisjyjyjojkftdldljnihjkjkhsioinjtiodmihkshsjnjojzihdmiajljnoycsfhtpsotanshkhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloybdtpsoingtihjkjkhsioinjtiooycsfxtpsokpiajljndmihkshsjnjojzihdmjnihjkjkhsioinjtiooyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdoycsfncsgagdvamume
│ ur:envelope/lptpsotpcxkscxisjyjyjojkftdldljkjyhsjykpjkdmihkshsjnjojzihdmiajljndlhsjziniaihoybdtpsoiygujyhsjykpjkoycsfxtpsojpiajljndmihkshsjnjojzihdmjkjyhsjykpjkoyaytpsotanshkhdcxsgpaaynbpdrdlbmkloykidfzmdtonnlngrtyrkbwcpfnmntyoyamuoetwydaremwoycsfncsfdglmhuenb
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

```
XID_SERVICES=($(envelope xid service all $ALICE_XID_DOC_WITH_SERVICE))
envelope format ${XID_SERVICES[1]}

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

```
envelope format ${XID_SERVICES[2]}

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

## `xid service find`: Find a Service by its URI

### `xid service find uri`: Find a Service by its URI

Returns at most one service envelope.

```
envelope xid service find uri 'https://status.example.com/alice' $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://status.example.com/alice) [
│     'allow': 'Sign'
│     'capability': "com.example.status"
│     'key': Reference(cab108a0)
│     'name': "Status"
│ ]
```

### `xid service find name`: Find a Service by its Name

May return multiple service envelopes.

```
envelope xid service find name 'Messaging' $ALICE_XID_DOC_WITH_SERVICE | envelope format

│ URI(https://messaging.example.com) [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│     'capability': "com.example.messaging"
│     'delegate': Reference(f1199a75)
│     'key': Reference(cab108a0)
│     'name': "Messaging"
│ ]
```

## `xid service remove`: Remove a Service from a XID Document

Alice removes the messaging service.

```
ALICE_XID_DOC_WITH_SERVICE_REMOVED=`envelope xid service remove 'https://messaging.example.com' $ALICE_XID_DOC_WITH_SERVICE`
envelope format $ALICE_XID_DOC_WITH_SERVICE_REMOVED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'Sign'
│         'capability': "com.example.status"
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

## `xid service update`: Update an Existing Service in an Existing XID Document

- To remove the name, use `--name ''`.
- To remove the capability, use `--capability ''`.
- Passing one or more `--key` options replaces the existing keys with the ones provided.
- Passing one or more `--delegate` options replaces the existing delegates with the ones provided.
- Passing one or more `--allow` options replaces the existing permissions with the ones provided.

Alice adds Bob as a delegate to the status service. This leaves Alices key and all other attributes of the service unchanged.

```
ALICE_XID_DOC_WITH_SERVICE_UPDATED=`envelope xid service update \
    --delegate $BOB_XID_DOC \
    'https://status.example.com/alice' \
    $ALICE_XID_DOC_WITH_SERVICE_REMOVED`

envelope format $ALICE_XID_DOC_WITH_SERVICE_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(f1199a75) [
│             'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     } [
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│     'service': URI(https://status.example.com/alice) [
│         'allow': 'All'
│         'capability': "com.example.status"
│         'delegate': Reference(f1199a75)
│         'key': Reference(cab108a0)
│         'name': "Status"
│     ]
│ ]
```

Removing a key or delegate from the XID that is referenced by a service is not allowed.

To remove a key or delegate that is referenced by a service, first remove the service.

```
envelope xid delegate remove $BOB_XID_DOC $ALICE_XID_DOC_WITH_SERVICE_UPDATED

│ Error: item is still referenced: delegate
```
