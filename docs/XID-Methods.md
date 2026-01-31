# XID Methods

Resolution methods are URIs that describe how to resolve a XID. They are used to find the complete, most up-to-date version of a XID document.

```
envelope xid method --help

│ Work a XID document's resolution methods
│
│ Usage: envelope xid method <COMMAND>
│
│ Commands:
│   add     Add a resolution method to a XID document
│   all     Retrieve all the XID document's resolution methods
│   at      Retrieve the resolution method at the given index
│   count   Print the count of the XID document's resolution methods
│   remove  Remove the given resolution method from the XID document
│   help    Print this message or the help of the given subcommand(s)
│
│ Options:
│   -h, --help     Print help
│   -V, --version  Print version
```

## `xid method add`: Add a Resolution Method to a XID Document

```
XID_DOC=`envelope xid new --nickname 'Alice' $ALICE_PUBKEYS`
XID_DOC_WITH_RESOLVERS=`envelope xid method add 'https://resolver.example.com/' $XID_DOC | \
    envelope xid method add 'btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6'`
envelope format $XID_DOC_WITH_RESOLVERS

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
│     'dereferenceVia': URI(https://resolver.example.com/)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```

## `xid method count`: Count the Number of Resolution Methods in a XID Document

```
envelope xid method count $XID_DOC_WITH_RESOLVERS

│ 2
```

## `xid method at`: Return the Resolution Method at the Specified Index

The indexes are zero-based, and in the order the resolution methods appear in the XID document's Gordian Envelope, which is not necessarily the order they appear via `envelope format`.

```
envelope xid method at 0 $XID_DOC_WITH_RESOLVERS

│ https://resolver.example.com/
```

```
envelope xid method at 1 $XID_DOC_WITH_RESOLVERS

│ btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

## `xid method all`: List All Resolution Methods in a XID Document

```
envelope xid method all $XID_DOC_WITH_RESOLVERS
```

```
https://resolver.example.com/
btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6
```

## `xid method remove`: Remove a Resolution Method from a XID Document

```
envelope xid method remove 'https://resolver.example.com/' $XID_DOC_WITH_RESOLVERS | envelope format

│ XID(93a4d4e7) [
│     'dereferenceVia': URI(btc:5e54156cfe0e62d9a56c72b84a5c40b84e2fd7dfe786c7d5c667e11ab85c45c6)
│     'key': PublicKeys(cab108a0, SigningPublicKey(93a4d4e7, SchnorrPublicKey(26712894)), EncapsulationPublicKey(00b42db3, X25519PublicKey(00b42db3))) [
│         'allow': 'All'
│         'nickname': "Alice"
│     ]
│ ]
```
