# XID Delegates

A *delegate* is XID document that is authorized to act on behalf of the *principal* XID document. A delegate can be granted any permissions, but its *effective* permissions will be a subset of the permissions of the principal XID document.

```
envelope xid delegate --help

│ Work with a XID document's delegates
│
│ Usage: envelope xid delegate <COMMAND>
│
│ Commands:
│   add     Add a delegate to the XID document
│   all     Retrieve all delegates from the XID document
│   at      Retrieve the XID document's delegate at the specified index
│   count   Print the count of the XID document's delegates
│   find    Find a delegate in the XID document
│   remove  Remove a delegate from the XID document
│   update  Update a delegate in the XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

## `xid delegate add`: Add a Delegate to a XID Document

This example:

- creates a XID documents for Alice, Bob, Carol, and Dave,
- grants Carol all permissions on behalf of Alice,
- grants Bob the ability to sign and encrypt on behalf of Alice,
- grants Dave the ability to elide data on behalf of Alice,
    - but only add's Dave's XID identifier to the XID document, which means it will have to be resolved to be used.

```
ALICE_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxdntswmjerdqdoxhnguzsdrhfcmjsfewkhkvezohkeycpasdysrvdgypeoemtgywztansgehdcxisespmvlhflnweksvyfnmhvofysnhyztpyhlftluweaoemenurstreckoybbfroektnncyls"
ALICE_PUBKEYS=`envelope generate pubkeys $ALICE_PRVKEYS`
BOB_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxhnlyeyzccpldfhsbmekkhspsmonlonctptenpkhettluhpzmteldssmejtdwbakttansgehdcxrkvapykpvalucwkgsalnmndefsfxfefsbwlujycebafybdqdpddwswswlktyzerfbeylotmk"
BOB_PUBKEYS=`envelope generate pubkeys $BOB_PRVKEYS`
CAROL_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxmorsytadihzswmckyltauyolecmevychhlwmtylbhsmdptfdrtuewnjtdkmnmkretansgehdcxhentsejphsfwclylihbwroaoisptaskegrimyldebecsdrrtbdlrrslazeursspmldtkmdds"
CAROL_PUBKEYS=`envelope generate pubkeys $CAROL_PRVKEYS`
DAVE_PRVKEYS="ur:crypto-prvkeys/lftansgohdcxsbqzasvdrpmuhegoaelekbwznnlfskkpyadrfhsncxlrmkihrecskpvapactresotansgehdcxflaxjtaskssogemtdpioaehpdytbtedyrtclkoceckbbadtlhlhljtensnylatvokkztwdny"
DAVE_PUBKEYS=`envelope generate pubkeys $DAVE_PRVKEYS`

ALICE_XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
BOB_XID_DOC=`envelope xid new --nickname 'Bob' $BOB_PUBKEYS`
CAROL_XID_DOC=`envelope xid new --nickname 'Carol' $CAROL_PUBKEYS`
DAVE_XID_DOC=`envelope xid new --nickname 'Dave' $DAVE_PUBKEYS`
DAVE_XID=`envelope xid id $DAVE_XID_DOC`

ALICE_XID_DOC=`envelope xid delegate add --allow 'all' $CAROL_XID_DOC $ALICE_XID_DOC`
ALICE_XID_DOC=`envelope xid delegate add --allow 'sign' --allow 'encrypt' $BOB_XID_DOC $ALICE_XID_DOC`
ALICE_XID_DOC=`envelope xid delegate add --allow 'elide' $DAVE_XID $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7) [
│             'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│                 'allow': 'All'
│                 'nickname': "Carol"
│             ]
│         ]
│     } [
│         'allow': 'All'
│     ]
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

## `xid delegate count`: Count the Number of Delegates in a XID Document

```
envelope xid delegate count $ALICE_XID_DOC

│ 3
```

## `xid delegate at`: Return the Delegate at the Specified Index

The indexes are zero-based, and in the order the delegate assertions appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid delegate at 0 $ALICE_XID_DOC | envelope format

│ {
│     XID(61b1f3c7) [
│         'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│             'allow': 'All'
│             'nickname': "Carol"
│         ]
│     ]
│ } [
│     'allow': 'All'
│ ]
```

```
envelope xid delegate at 1 $ALICE_XID_DOC | envelope format

│ {
│     XID(f1199a75) [
│         'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│             'allow': 'All'
│             'nickname': "Bob"
│         ]
│     ]
│ } [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│ ]
```

```
envelope xid delegate at 2 $ALICE_XID_DOC | envelope format

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]
```

## `xid delegate all`: List All Delegates in a XID Document

```
envelope xid delegate all $ALICE_XID_DOC

│ ur:envelope/lftpsplftpsotanshdhdcxhspawfstecswotwpbsweiowlsrmyfpwpskmeonrtjsrhetsrhnaxfwylvtvsuorkoyaylstpsotansgylftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwoycsfncsfgoycscstpsoihfxhsjpjljzoycsfncsfgknhpttwe
│ ur:envelope/lstpsplftpsotanshdhdcxwncfnykphhsekedagdsfqdihoysadpzmimrpgtrnlesansjtdshtkedyhlwdmngloyaylstpsotansgylftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwpheoycsfncsfgoycscstpsoiafwjlidoycsfncsfdoycsfncsgawnftoeoy
│ ur:envelope/lftpsptpsotanshdhdcxenenaefmosgecksalokgmnrhgrsemhhfnlfssroxbytkvllrvsrhgtgscpvswfveoycsfncsgegtgtyljt
```

Example capturing the above envelopes into a shell array. Note that newer shells like `zsh` use one-based indexing by default, but can be configured to use zero-based indexing.

```
XID_DELEGATES=($(envelope xid delegate all $ALICE_XID_DOC))
envelope format ${XID_DELEGATES[1]}

│ {
│     XID(61b1f3c7) [
│         'key': PublicKeys(eebd4add, SigningPublicKey(61b1f3c7, SchnorrPublicKey(8684e3e4)), EncapsulationPublicKey(0995c476, X25519PublicKey(0995c476))) [
│             'allow': 'All'
│             'nickname': "Carol"
│         ]
│     ]
│ } [
│     'allow': 'All'
│ ]
```

```
envelope format ${XID_DELEGATES[2]}

│ {
│     XID(f1199a75) [
│         'key': PublicKeys(e2c18423, SigningPublicKey(f1199a75, SchnorrPublicKey(f0638394)), EncapsulationPublicKey(4af6be52, X25519PublicKey(4af6be52))) [
│             'allow': 'All'
│             'nickname': "Bob"
│         ]
│     ]
│ } [
│     'allow': 'Encrypt'
│     'allow': 'Sign'
│ ]
```

```
envelope format ${XID_DELEGATES[3]}
```

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]

## `xid delegate find`: Find a Delegate by its XID Identifier

```
envelope xid delegate find $DAVE_XID $ALICE_XID_DOC | envelope format

│ {
│     XID(3636003e)
│ } [
│     'allow': 'Elide'
│ ]
```

## `xid delegate update`: Update an Existing Delegate in an Existing XID Document

- Replaces the existing delegate with the one provided, which must already exist in the XID document.
- Replaces the permissions of the existing delegate with the ones provided.

In this example:
- Carol's XID document is replaced with her bare XID, and
- her permissions are reduced.

```
CAROL_XID=`envelope xid id $CAROL_XID_DOC`
ALICE_XID_DOC_UPDATED=`envelope xid delegate update --allow 'auth' --allow 'encrypt' --allow 'sign' $CAROL_XID $ALICE_XID_DOC`
envelope format $ALICE_XID_DOC_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7)
│     } [
│         'allow': 'Authorize'
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
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

## `xid delegate remove`: Remove a Delegate from a XID Document

```
BOB_XID=`envelope xid id $BOB_XID_DOC`
ALICE_XID_DOC_UPDATED=`envelope xid delegate remove $BOB_XID $ALICE_XID_DOC_UPDATED`
envelope format $ALICE_XID_DOC_UPDATED

│ XID(93a4d4e7) [
│     'delegate': {
│         XID(3636003e)
│     } [
│         'allow': 'Elide'
│     ]
│     'delegate': {
│         XID(61b1f3c7)
│     } [
│         'allow': 'Authorize'
│         'allow': 'Encrypt'
│         'allow': 'Sign'
│     ]
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```
