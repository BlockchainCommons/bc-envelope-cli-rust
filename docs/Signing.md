# `envelope` - Signing Envelopes

The `envelope` tool can sign and verify envelopes using several different algorithms. This document describes the signing primitives and commands.

## Table of Contents

- [Signing Algorithms](#signing-algorithms)
- [Signing Primitives](#signing-primitives)
  - [Derivations](#derivations)
  - [Signers and Verifiers](#signers-and-verifiers)
  - [Seed](#seed)
  - [Private Keybase](#private-keybase)
  - [X25519 Keys](#x25519-keys)
  - [Public Keybase](#public-keybase)
  - [Signing Private Key](#signing-private-key)
  - [Signing Public Key](#signing-public-key)
  - [Signature](#signature)
- [Basic Signing](#basic-signing)
- [Signing with SSH](#signing-with-ssh)
  - [Generating an SSH Signing Key from a Private Key Base](#generating-an-ssh-signing-key-from-a-private-key-base)
  - [Importing an SSH Signing Key from an Existing Key File](#importing-an-ssh-signing-key-from-an-existing-key-file)
  - [Signing with the SSH Key](#signing-with-the-ssh-key)
  - [Generating an SSH Verifier from an SSH Signing Key](#generating-an-ssh-verifier-from-an-ssh-signing-key)
  - [Verifying the SSH Signature](#verifying-the-ssh-signature)
- [Exporting SSH Keys](#exporting-ssh-keys)
    - [Exporting an SSH Private Key](#exporting-an-ssh-private-key)
    - [Exporting an SSH Public Key](#exporting-an-ssh-public-key)
    - [Exporting an SSH Signature](#exporting-an-ssh-signature)
- [Getting Information about SSH Keys](#getting-information-about-ssh-keys)

## Signing Algorithms

Envelopes may be signed by several algorithms. The default is Schnorr, which is appropriate for most cases, but ECDSA, Ed25519, and several SSH variants are also supported for special purposes.

| Algorithm | Description |
|-----------|-------------|
| `schnorr` | Schnorr |
| `ecdsa` | ECDSA |
| `ed25519` | Ed25519 |
| `ssh-ed25519` | SSH-Ed25519 |
| `ssh-rsa-sha256` | SSH-RSA SHA-256 |
| `ssh-rsa-sha512` | SSH-RSA SHA-512 |
| `ssh-dsa` | SSH-DSA |
| `ssh-ecdsa-p256` | SSH-ECDSA NIST P-256 |
| `ssh-ecdsa-p384` | SSH-ECDSA NIST P-384 |

**Note:** The `ssh-ecdsa-p521` variant is not currently supported because of a [bug in the `ssh-key` crate](https://github.com/RustCrypto/SSH/issues/232).

## Signing Primitives

| Type | UR type | Signer | Verifier | Note |
|------|---------|--------|----------|------|
| Seed | `ur:seed` | | | Used to derive other objects. |
| Private key base | `ur:prvkeys` | ✅ | ✅ | Directly signs and verifies Schnorr only. |
| Public key base | `ur:pubkeys` | | ✅ | Contains a signing public key and an encapsulation public key. |
| Signing private key | `ur:signing-private-key` | ✅ | ✅ | Supports Schnorr, ECDSA, Ed25519, and SSH variants. |
| Signing public key | `ur:signing-public-key` | | ✅ | Supports Schnorr, ECDSA, Ed25519, and SSH variants. |

### Derivations

```mermaid
graph LR
    Seed --> PrivateKeyBase
    random([Random]) --> PrivateKeyBase
    PrivateKeyBase --> SigningPrivateKey
    PrivateKeyBase --> SigningPublicKey
    PrivateKeyBase --> X25519PrivateKey
    X25519PrivateKey --> X25519PublicKey
    PrivateKeyBase --> PublicKeyBase
    PublicKeyBase --> SigningPublicKey
    PublicKeyBase --> X25519PublicKey
```

### Signers and Verifiers

A *signer* is any cryptographic object that can be combined with a message to produce a signature. Signers include private key bases and signing private keys.

A *verifier* is any cryptographic object that can be combined with a message and a signature to confirm that the signature was made by a particular verifier. Verifiers include private key bases, public key bases, signing private keys, and signing public keys.

### Seed

A cryptographic seed (`ur:seed`) is a sequence of random numbers from which other primitives can be derived. A seed UR may contain other metadata such as its name, its creation date, a human-readable note, and an output descriptor for use with Bitcoin.

A seed is neither a signer nor a verifier. It is used solely for the purpose of deriving other objects. The `envelope` tool can derive a private key base from a seed or generate a random private key base.

### Private Keybase

A private key base (`ur:prvkeys`) is private key material from which other private keys may be derived. A private keybase may be generated randomly or derived from a seed.

When used as a signer, the `envelope` tool dervives a Schnorr signing private key to make the actual signature. When used as a verifier, the `envelope` tool derives a Schnorr signing public key to verify the signature.

### X25519 Keys

*X25519 keys* (private and public) are not used for signing, but are mentioned here to distinguish them from *signing keys*. X25519 keys are used to perform key agreement for public key encryption. Like signing keys, they are derived from key material provided by a private key base.

### Public Keybase

A public key base (`ur:pubkeys`) contains a signing public key and an encapsulation public key. The signing public key is used to verify signatures from a sender, and the encapsulation public key is used to encrypt messages to the same entity as recipient. The signing public key may support any of the signing algorithms listed above. The purpose of a public key base is to provide a single structure for both signature verification and encryption to the owner of the public key.

### Signing Private Key

A signing private key (`ur:signing-private-key`) is a private key used to sign messages. It is derived from a private key base. A signing private key supports a single signing algorithm, and may also be used as a verifier.

### Signing Public Key

A signing public key (`ur:signing-public-key`) is a public key used to verify signatures. It is derived from a signing private key or a public key base. A signing public key supports a single signing algorithm.

### Signature

A signature (`ur:signature`) is a cryptographic object that is produced by combining a message with a signer. It can be verified by combining the message, the signature, and the correct verifier. A signature is algorithm-specific and cannot be verified by a verifier that does not support the same algorithm.

## Basic Signing

```bash
👉
ALICE_KNOWS_BOB=ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns
envelope format $ALICE_KNOWS_BOB
```

```bash
👈
"Alice" [
    "knows": "Bob"
]
```

The `envelope` tool can add a signature to a message using a private key base or a signing private key. The signature is added as an assertion on the subject of the envelope.

To generate a private key base randomly:

```bash
👉
envelope generate prvkeys
```

```
👈
ur:crypto-prvkeys/hdcxhdvsaelylaaesfqdwzghfmsswfrlzsfgytbbnecpkshekstbhdwzrkktasknztkecycaotda
```

Cryptographic seeds can also be used as a starting point. For more about seeds, see the [Gordian Seed Tool iOS app](https://apps.apple.com/us/app/gordian-seed-tool/id1545088229) or the [`seedtool` command line tool](https://github.com/BlockchainCommons/seedtool-cli-rust).

If you wish to use a seed to generate a private key base:

```bash
👉
SEED=ur:seed/oyadgdmdeefejoaonnatcycefxjedrfyaspkiakionamgl
PRVKEYS=`envelope generate prvkeys --seed $SEED`
echo $PRVKEYS
```

```
👈
ur:crypto-prvkeys/gdmdeefejoaonnatcycefxjedrfyaspkiawdioolhs
```

We can use the private key base as-is to sign an envelope using Schnorr. We can also use it to derive the actual Schnorr signing private key:

```bash
👉
SIGNER=`envelope generate signer $PRVKEYS`
echo $SIGNER
```

```bash
👈
ur:signing-private-key/hdcxasfymwaxcpktaowpatotolckatgrhnceveasueskwereprcyfrmstpfgflaahnwlbewlqdga
```

Later we'll see how to derive signing private keys of other types, like SSH.

Of course, we'll also want to distribute the public key base, so the signature can be verified:

```bash
👉
PUBKEYS=`envelope generate pubkeys $PRVKEYS`
echo $PUBKEYS
```

```
👈
ur:crypto-pubkeys/lftanshfhdcxweplrnkpsruepkaeahnetppsteaojtdlgudetlyksrlbzoiduoglpemujydnsrattansgrhdcximbgoskbjpgtluwededpjywdlkfwksjpglsrfdcaurdahycfasmtylihpfrsfgkblomttisr
```

Recall that a public key base actually contains two public keys: one for verifying signatures and one for encryption to a recipient. The signing public key can be extracted from the public key base:

```bash
👉
VERIFIER=`envelope generate verifier $PUBKEYS`
echo $VERIFIER
```

```bash
👈
ur:signing-public-key/hdcxweplrnkpsruepkaeahnetppsteaojtdlgudetlyksrlbzoiduoglpemujydnsratyadyptla
```

Or the signing public key can be derived from the signing private key:

```bash
👉
VERIFIER=`envelope generate verifier $SIGNER`
echo $VERIFIER
```

```bash
👈
ur:signing-public-key/hdcxweplrnkpsruepkaeahnetppsteaojtdlgudetlyksrlbzoiduoglpemujydnsratyadyptla
```

Again: So far we're just using Schnorr, the default. If the signer is of type Schnorr, the verifier will be as well. If the signer is of type SSH, the verifier will be as well.

Now we can sign our envelope:

```bash
👉
SIGNED=`envelope sign --signer $PRVKEYS $ALICE_KNOWS_BOB`
```

Let's see what it looks like when formatted now:

```bash
👉
envelope format $SIGNED
```

```
👈
"Alice" [
    "knows": "Bob"
    'signed': Signature
]
```

OK... there's a signature there now, but it's a new assertion on the subject of the envelope, "Alice". This means that any of the assertions can still be altered without invalidating the signature on the subject. But what if we want to sign the *whole* envelope, including the fact that she knows Bob?

Wrapping to the rescue! First we wrap the entire envelope in a new envelope, then we sign the wrapped envelope:

```bash
👉
WRAPPED_SIGNED=`envelope subject type wrapped $ALICE_KNOWS_BOB | envelope sign --signer $PRVKEYS`
envelope format $WRAPPED_SIGNED
```

```
👈
{
    "Alice" [
        "knows": "Bob"
    ]
} [
    'signed': Signature
]
```

Now the entire contents of the envelope are signed, and if we send it to someone who has our public key, they can verify the signature:

```bash
👉
envelope verify --verifier $PUBKEYS $WRAPPED_SIGNED
```

```
👈
ur:envelope/lftpsplftpcsihfpjziniaihoytpcsihjejtjlktjktpcsiafwjlidoyaxtpcstansghhdfznltbglechtrkecemfhahkbrkcfzcasfnbbkpktzmsrvewtksknahmnpkinguktdwkgfrdklfrtdwpssamujtidcteovyongeamayftfxiaesfwceecoxueimmhwfrsyaidiycwdl
```

To facilitate piping commands, the `verify` command prints the input envelope if the validation is successful (unless the `--silent` flag is provided), and exits with an error condition if it is unsuccessful. Lets produce some incorrect public keys and try this:

```bash
👉
BAD_PUBKEYS=`envelope generate prvkeys | envelope generate pubkeys`
envelope verify --verifier $BAD_PUBKEYS $WRAPPED_SIGNED
```

```
👈
Error: could not verify a signature
```

Note that signing uses randomness. So even if you sign the same envelope twice with the same signer, the two resulting envelopes will not be the same although both signatures will verify against the same verifier.

## Signing with SSH

Specific applications may want to sign envelopes using SSH (Secure Shell) keys. The `envelope` tool supports several SSH key types, including Ed25519, RSA, DSA, and ECDSA. The following example demonstrates how to sign an envelope using an Ed25519 key.

First, we can't sign directly with a private key base, since it has no inherent type and is therefore only usable for signing with the Schnorr algorithm. So we need to acquire a signing private key of the appropriate type. This can be done by generating it from a private key base, or by importing it from an existing SSH key file.

### Generating an SSH Signing Key from a Private Key Base

Although it isn't required, it is best practice to include a comment when generating an SSH key. This comment is included in the public key and can be used to identify the key's owner.

```bash
👉
SSH_SIGNER=`envelope generate signer --type ssh-ed25519 --comment "wolf@Wolfs-MacBook-Pro.local" $PRVKEYS`
echo $SSH_SIGNER
```

```bash
ur:signing-private-key/tanehnkkadotdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfxioidiajnhdhsiseyetjshtdyetiofehtjygmjtgaksjzeckoimjyfxetjygokkiajyesgskpeegwisfggrfgktfpfpfpgrfxhdihetgwiyjzeokofybkjtktfpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfxioidiajnhdhsiseyetjshtdyetiofehtjygmjtgaksjzeckoimjyfxetjygokkiajyesgskpeegwisfggrfgktbkfpfpfpfefyishtfejsknhdeefwgafpgsdlktgakkdlfdidfgfpihjyhtenfghkiojlgrkphtimgheofweojtgdgokpghfpgrfwjykkhtiejsfdidkkjojtghkkfpgmjnehfliaimflhdbkjndngwdygskkehghgekkeodykpemioenfegojlhdfpfpfpfpfdfdiekoidflhtfphfeyesjkhtjtgtjyghhgfgimgyjneskohskkehgyiajnetkpidflesimhkhgktfwbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkptlelymy
```

### Importing an SSH Signing Key from an Existing Key File

If you have an existing SSH key file, you can import it into the `envelope` tool. The following example demonstrates how to import an Ed25519 key from an existing file. In this example, the key is stored in a file named `test_ed25519` and is encrypted with the password `test`. You can either provide the encryption password using the `--password` command-line argument or by typing it when prompted.

```bash
👉
SSH_SIGNER=`envelope import <./ssh_objects/test_ed25519`
Key decryption password: test
echo $SSH_SIGNER
```

```bash
ur:signing-private-key/tanehnkkadotdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpfpfpfpgrfpkoecimimgwgsdnhkeebkkniofpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpbkfpfpfpfefpemdldlecgrhkkokoenfgjliminktjsdngrfeisgaksgmjnfpiejeksjeeciogthdgseejkjojsknfwiogagtehkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflbkgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiofpfpfpfpfdfdiekoidflhtfphfeyesjkhtjtgtjyghhgfgimgyjneskohskkehgyiajnetkpidflesimhkhgktfwbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkgtctcyrd
```

### Signing with the SSH Key

Now that we have an SSH signing key, we can use it to sign an envelope. The following example demonstrates how to sign an envelope using the Ed25519 key we generated earlier.

Note that when signing with SSH keys, two additional options may be used: `--namespace` and `--hash-type`. The default namespace is `envelope`, and the default hash type is `sha256`. These defaults are fine for most cases.

```bash
👉
WRAPPED=`envelope subject type wrapped $ALICE_KNOWS_BOB`
SSH_SIGNED=`envelope sign --signer $SSH_SIGNER $WRAPPED`
envelope format $SSH_SIGNED
```

```bash
👈
{
    "Alice" [
        "knows": "Bob"
    ]
} [
    'signed': Signature
]
```

This signed envelope looks just like the one we signed with Schnorr. The difference is that the signature was made with an SSH key, and must be verified with the corresponding SSH verifier.

### Generating an SSH Verifier from an SSH Signing Key

To verify the signature, we need to generate an SSH verifier from the SSH public key. The following example demonstrates how to generate an SSH verifier from the Ed25519 key we generated earlier.

```bash
👉
SSH_VERIFIER=`envelope generate verifier $SSH_SIGNER`
echo $SSH_VERIFIER
```

```bash
👈
ur:signing-public-key/tanehsksjnjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafgkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiocxktjljziyfzhgjljziyjkdpgthsiafwjljljedpgdjpjldmjzjliahsjzmybngyfs
```

### Verifying the SSH Signature

Now that we have an SSH verifier, we can use it to verify the signature. The following example demonstrates how to verify the signature using the Ed25519 verifier we generated earlier.

```bash
👉
envelope verify --silent --verifier $SSH_VERIFIER $SSH_SIGNED
```

```bash
👈
<nothing printed>
```

Since the verification was successful, and we used the `--silent` flag, the command did not print the input envelope.

## Exporting SSH Keys

The `envelope` tool can export SSH keys to Open SSH format.

### Exporting an SSH Private Key

The following example demonstrates how to export an SSH private key to stdout. The private key can be saved to a file by redirecting the output to a file.

```bash
👉
envelope export $SSH_SIGNER
```

```bash
👈
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYAAAAKAv5jjOL+Y4
zgAAAAtzc2gtZWQyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYA
AAAEA7//5KYvv6Fojiwq+KEhIxRmAdkxk5gMXL4spqzBgIM1uMSVOimmADR7iCnLS7wO5G
KTzybWCBkZWnO2d4KoBgAAAAHHdvbGZAV29sZnMtTWFjQm9vay1Qcm8ubG9jYWwB
-----END OPENSSH PRIVATE KEY-----
```

If desired, the private key can be encrypted with a password. To encrypt, add the `--encrypt` switch and either provide the password on the command line using the `--password` option or type it when prompted.

```bash
👉
envelope export --encrypt --password "test" $SSH_SIGNER
```

```bash
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABAIMtmOvv
zzr7yu5OT+C6tOAAAAEAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iC
nLS7wO5GKTzybWCBkZWnO2d4KoBgAAAAoP7dlfXGRYBRsJgg0D2L9M/SUwyIjLXzhZajj0
a0jRjueShpPOZ/r8HK99Jrq7IeCzxcmBqdZZxEPkDHPRNzVvZZRUYDFtNiszqsaZoLlf+r
nGTfaTzo72XxjVCzlRG2QwDBj3WeSfi3IijauuJtopdwz+OmMNNwJG6Ba4fcbATInXtHlk
UNt/6cpHT7g7Y6BKJBfAXCVato3ThQqbRlunE=
-----END OPENSSH PRIVATE KEY-----
```

### Exporting an SSH Public Key

The following example demonstrates how to export an SSH public key to stdout. The public key can be saved to a file by redirecting the output to a file.

```bash
👉
envelope export $SSH_VERIFER
```

```bash
👈
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iCnLS7wO5GKTzybWCBkZWnO2d4KoBg wolf@Wolfs-MacBook-Pro.local
```

### Exporting an SSH Signature

If you have a `ur:signature` object that was generated with an SSH key, you can export it to a file in OpenSSH format. The following example demonstrates how to export an SSH signature to stdout. The signature can be saved to a file by redirecting the output to a file.

```bash
👉
SSH_SIGNATURE="ur:signature/taneidkkaddsdpdpdpdpdpfwfeflgaglcxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkgoehglgagodyjzfdfpfpfpfpfpgyfpfpfpfygtfpfpfpfpgsiaeogljlgshghfjegtimgoehgtghjefpfpfpfpiohgeeksgegoengrhshkfpglfdkpgagriajygskofpemjehkjogdgdbkgejyhkgaflgmjzhsiaemhteoiojsioflfpfpfpfpfpfehtjnjzjkhtgyfpfpfpfpfpfpfpfpfpfliaeyisisglghfekkfpfpfpfpgoktfpfpfpfpjykniaeyiojyhthggykkglghgoksbkgwgyfpfpfpfefpenglkseeflhkghknjkeogegsdlfedldljpkpgeiyeyfpjlidfyiyjkjtiogmishtetgyghdlfwhshtjtgsjliyiejpgmfpgogagtgdideegresjyjlfxfxgaghimflbkguiofxkpgyjkhfdydlisgogojpiaimemkofwfxiohkgrbkdpdpdpdpdpfeglfycxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkcllebeje"
envelope export $SSH_SIGNATURE
```

```bash
👈
-----BEGIN SSH SIGNATURE-----
U1NIU0lHAAAAAQAAADMAAAALc3NoLWVkMjU1MTkAAAAgW4xJU6KaYANHuIKctLvA7kYpPP
JtYIGRlac7Z3gqgGAAAAAEZmlsZQAAAAAAAAAGc2hhNTEyAAAAUwAAAAtzc2gtZWQyNTUx
OQAAAEA6Nx4GYTzs3JL/E//ruJf2AobDfsngRhZ8QT/BaZnLofdrRAUIMPb4K9toCCITjG
SgCuQsV0/hUUrcj7vBCgYK
-----END SSH SIGNATURE-----
```

## Getting Information about SSH Keys

You can use the `info` subcommand to retrieve information about an SSH key or signature, including its type, comment, and fingerprint.

```bash
👉
envelope info $SSH_SIGNER
```

```
👈
Format: ur:signing-private-key
Description: SSH Signing Private Key
Algorithm: ssh-ed25519
Fingerprint: SHA256:wonJHWbmVYaZyry76VO/QM50PRqBKbFB1y3oBAGRtuY
+----[ED25519]----+
|    o=*o.*o.     |
|    o  =*=o .    |
|   . +B++ ..     |
|   .oX+=.  o     |
|   o+ *.S o o    |
|    E .*.. o .   |
|       o+..      |
|      o. ..      |
|     .+o  ..     |
+----[SHA256]-----+
```
