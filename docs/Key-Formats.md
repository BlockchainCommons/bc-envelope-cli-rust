# CLI Key Formats

The `envelope` CLI uses UR-encoded deterministic CBOR for keys. The two bundle
types most users see are `ur:crypto-prvkeys` and `ur:crypto-pubkeys`.

## Key Bundles

`PrivateKeys` is encoded as `ur:crypto-prvkeys` with CBOR tag `40013`. It
contains two keys:

- A signing private key, encoded with CBOR tag `40021`
- An encapsulation private key, usually X25519

`PublicKeys` is encoded as `ur:crypto-pubkeys` with CBOR tag `40017`. It also
contains two keys:

- A signing public key, encoded with CBOR tag `40022`
- An encapsulation public key, usually X25519

The CLI defaults to Schnorr signing and X25519 encapsulation for ordinary
generated keypairs:

```sh
PRVKEYS=$(envelope generate prvkeys)
PUBKEYS=$(envelope generate pubkeys "$PRVKEYS")
```

`envelope generate pubkeys` derives the public bundle from the private bundle.
It does not need access to a separate public key file.

## Inspecting Key URs

Wrap the UR as an envelope subject and format it to see the decoded structure:

```sh
PRVKEYS=ur:crypto-prvkeys/lftansgohdcxrnndemlygabamwnblbttsrpfztbksfwybtskbavlvtdnpsrkmeknkpiybbfhonhdtansgehdcxbsgrcfdlteseresbhfiyjkhnctbagamdbyvsgmmdltuttysgnbbnpmamtphswkjsztjszemu
envelope subject type ur "$PRVKEYS" | envelope format
```

```text
PrivateKeys(7b8fdc6e, SigningPrivateKey(eda4f47a, SchnorrPrivateKey(3a2297f7)), EncapsulationPrivateKey(d1dcb383, X25519PrivateKey(d1dcb383)))
```

Then derive and inspect the public bundle:

```sh
PUBKEYS=$(envelope generate pubkeys "$PRVKEYS")
envelope subject type ur "$PUBKEYS" | envelope format
```

```text
PublicKeys(1c3eb3ea, SigningPublicKey(b82350cf, SchnorrPublicKey(d263d0fa)), EncapsulationPublicKey(b5b44154, X25519PublicKey(b5b44154)))
```

The short hexadecimal values in formatted output are object references, not the
full key bytes.

## Signing Key Encodings

Signing keys are carried inside the key bundles. The signing key CBOR payload
depends on the signing scheme:

| Scheme | Private payload | Public payload |
| ------ | --------------- | -------------- |
| Schnorr | 32-byte byte string | 32-byte x-only public key byte string |
| ECDSA/secp256k1 | Array `[1, bytes]` where `bytes` is the 32-byte private key | Array `[1, bytes]` where `bytes` is the 33-byte compressed public key |
| Ed25519 | Array `[2, bytes]` where `bytes` is the 32-byte private key | Array `[2, bytes]` where `bytes` is the 32-byte public key |
| SSH | Tagged OpenSSH text | Tagged OpenSSH public key text |

For ECDSA interoperability, the important details are the discriminator `1`,
the 32-byte secp256k1 private key, and the 33-byte compressed secp256k1 public
key.

## Native Formats

The CLI can import OpenSSH private keys, public keys, and signatures:

```sh
envelope import < ~/.ssh/id_ed25519
envelope import "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA..."
```

The CLI can export SSH signing private keys, SSH signing public keys,
`PublicKeys` bundles that contain SSH public keys, and SSH signatures:

```sh
envelope export "$SSH_PRVKEY_UR"
envelope export "$SSH_PUBKEY_UR"
```

This SSH support is separate from the lower-level ECDSA/secp256k1 encoding
described above. The CLI does not currently import or export non-SSH ECDSA keys
as native JWK, PEM, or DER files; those keys are represented directly in the UR
CBOR structure.
