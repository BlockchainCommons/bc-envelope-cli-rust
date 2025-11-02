# `envelope` - Signing Envelopes

The `envelope` tool can sign and verify envelopes using several different algorithms. This document describes the signing primitives and commands.

## Table of Contents

- [`envelope` - Signing Envelopes](#envelope---signing-envelopes)
  - [Table of Contents](#table-of-contents)
  - [Signing Algorithms](#signing-algorithms)
  - [Signing Primitives](#signing-primitives)
    - [Derivations](#derivations)
    - [Signers and Verifiers](#signers-and-verifiers)
    - [Seed](#seed)
    - [PrivateKeys](#privatekeys)
    - [PublicKeys](#publickeys)
    - [Signature](#signature)
  - [Basic Signing](#basic-signing)
  - [Signing with SSH](#signing-with-ssh)
    - [Generating the SSH Keys](#generating-the-ssh-keys)
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

Envelopes may be signed by several algorithms. The default is Schnorr, which is appropriate for most cases, but ECDSA, Ed25519, several SSH variants, and ML-KEM (a post-quantum algorithm) are also supported.

| Algorithm        | Description          |
| ---------------- | -------------------- |
| `schnorr`        | Schnorr              |
| `ecdsa`          | ECDSA                |
| `ed25519`        | Ed25519              |
| `ssh-ed25519`    | SSH-Ed25519          |
| `ssh-dsa`        | SSH-DSA              |
| `ssh-ecdsa-p256` | SSH-ECDSA NIST P-256 |
| `ssh-ecdsa-p384` | SSH-ECDSA NIST P-384 |
| `mlkem`          | ML-KEM               |

## Signing Primitives

| Type          | UR type             | Signer | Verifier | Note                            |
| ------------- | ------------------- | ------ | -------- | ------------------------------- |
| Seed          | `ur:seed`           |        |          | Used to derive other objects.   |
| `PrivateKeys` | `ur:crypto-prvkeys` | ✅      |          | Contains a signing private key. |
| `PublicKeys`  | `ur:crypto-pubkeys` |        | ✅        | Contains a signing public key.  |

### Derivations

### Signers and Verifiers

A *signer* is any cryptographic object that can be combined with a message to produce a signature. Signers include private key bases and signing private keys.

A *verifier* is any cryptographic object that can be combined with a message and a signature to confirm that the signature was made by a particular verifier. Verifiers include private key bases, `PublicKeys`, signing private keys, and signing public keys.

### Seed

A cryptographic seed (`ur:seed`) is a sequence of random numbers from which other primitives can be derived. A seed UR may contain other metadata such as its name, its creation date, a human-readable note, and an output descriptor for use with Bitcoin.

A seed is neither a signer nor a verifier. It is used solely for the purpose of deriving other objects. The `envelope` tool can derive a private key base from a seed or generate a random private key base.

### PrivateKeys

A `PrivateKeys` (`ur:crypto-prvkeys`) contains a signing private key. It can be generated from a sed. The signing private key may support any of the signing algorithms listed above. The purpose of `PrivateKeys` is to provide a single structure for both signing and decryption to the owner of the private key.

### PublicKeys

A `PublicKeys` (`ur:crypto-pubkeys`) contains a signing public key. The signing public key is used to verify signatures from a sender. The signing public key may support any of the signing algorithms listed above. The purpose of `PublicKeys` is to provide a single structure for both signature verification and encryption to the owner of the public key.

### Signature

A signature (`ur:signature`) is a cryptographic object that is produced by combining a message with a signer. It can be verified by combining the message, the signature, and the correct verifier. A signature is algorithm-specific and cannot be verified by a verifier that does not support the same algorithm.

## Basic Signing

```
ALICE_KNOWS_BOB=ur:envelope/lftpsoihfpjziniaihoytpsoihjejtjlktjktpsoiafwjlidutgmnnns
envelope format $ALICE_KNOWS_BOB

│ "Alice" [
│     "knows": "Bob"
│ ]
```

The `envelope` tool can add a signature to a message using a private key base or a signing private key. The signature is added as an assertion on the subject of the envelope.

To generate a set of private keys and their associated public keys, we can use the `generate keypairs` command. This will produce a `ur:crypto-prvkeys` and a `ur:crypto-pubkeys` on a single line, separated by a space:

```
envelope generate keypairs

│ ur:crypto-prvkeys/lftansgohdcxidykjlbzsgwnzsfelbmtlkhechlotbuoueeyiosblkcfztvtnsbahylskibzrogrtansgehdcxadmkvdlbgawsgsgddiaabwwkveptdlahdrcholmupareghtdetseeehsrlwzhkjycxztmslb ur:crypto-pubkeys/lftanshfhdcxmhltemlgfpgamyhdetbsmywmksmyrhtbsowlhlvedyhthsbndtcmgawmfwidzswltansgrhdcxwdhfeesatsnnplpttdrohslslynlskaowpahzechcnjowyrkbyonemckpllghdfgmntydmly
```

To separate both of these out into their own variables, we can use command substitution:

```
envelope generate keypairs | read PRVKEYS PUBKEYS

echo $PRVKEYS
│ ur:crypto-prvkeys/lftansgohdcxlkmdcscaayplntderopyzcbdrfioutfgticwjlbzuowfdwjybalelsrtvygllkpdtansgehdcxenwswpqzjyylflreztutmwwsotaacynlkedkplpkkncttyzsmetpyljlcygyldsscsbyjork

echo $PUBKEYS

│ ur:crypto-pubkeys/lftanshfhdcxcxatbyrpyapstsroaaskkettlsmkwztbamnnctemlffgecwdwfuyrpgdrycylkrytansgrhdcxythkcsenrdiehgsnoxfyaofntluyhlnnwkhpothnndglldjztipajnollgwtwzftsadkbwwm
```

Cryptographic seeds can also be used as a starting point. For more about seeds, see the [Gordian Seed Tool iOS app](https://apps.apple.com/us/app/gordian-seed-tool/id1545088229) or the [`seedtool` command line tool](https://github.com/BlockchainCommons/seedtool-cli-rust).

If you wish to use a seed to generate a set of private keys, you can do so like this:

```
SEED=ur:seed/oyadgdmdeefejoaonnatcycefxjedrfyaspkiakionamgl
PRVKEYS=`envelope generate prvkeys ur:seed/oyadgdmdeefejoaonnatcycefxjedrfyaspkiakionamgl`
echo $PRVKEYS

│ ur:crypto-prvkeys/lftansgohdcxasfymwaxcpktaowpatotolckatgrhnceveasueskwereprcyfrmstpfgflaahnwltansgehdcxsftngernmnplghvectctjnctwzonotgopfosylmokphdessnzmldgodewphpsedsgewlmthh
```

From there, we can derive the corresponding public keys:

```
PUBKEYS=`envelope generate pubkeys $PRVKEYS`
echo $PUBKEYS

│ ur:crypto-pubkeys/lftanshfhdcxweplrnkpsruepkaeahnetppsteaojtdlgudetlyksrlbzoiduoglpemujydnsrattansgrhdcximbgoskbjpgtluwededpjywdlkfwksjpglsrfdcaurdahycfasmtylihpfrsfgkblomttisr
```

As suggested by their names `ur:crypto-prvkeys` and `ur:crypto-pubkeys` each contain more than one key: a signing key and an encryption key (also called an *encapsulation key*). And each of these keys is "tuned" to a particular algorithm. By default, the signing key is a Schnorr key, and the encryption key is an X25519 key. To see the types of keys, we can convert them to an envelope and then show the formatted output:

```
envelope subject type ur $PRVKEYS | envelope format

│ PrivateKeys(20083bdd, SigningPrivateKey(bba0298a, SchnorrPrivateKey(9993e6c0)), EncapsulationPrivateKey(8b45d46d, X25519PrivateKey(8b45d46d)))
```

If we look at the output hierarchically, we can see that a `PrivateKeys` contains both a `SigningPrivateKey` and an `EncapsulationPrivateKey`. The particular kind of signing private key is `SchnorrPrivateKey`, and the particular kind of encapsulation private key is `X25519PrivateKey`. Each of these objects has its own unique identifier:

```
PrivateKeys: 20083bdd
  SigningPrivateKey: bba0298a
    SchnorrPrivateKey: 9993e6c0
  EncapsulationPrivateKey: 8b45d46d
    X25519PrivateKey: 8b45d46d
```

The `ur:crypto-pubkeys` has the same structure, but with public keys instead of private keys:

```
envelope subject type ur $PUBKEYS | envelope format

│ PublicKeys(866d11d2, SigningPublicKey(7612fa81, SchnorrPublicKey(386ce8c0)), EncapsulationPublicKey(c8fc78a2, X25519PublicKey(c8fc78a2)))
```

We can use the private keys to sign an envelope using Schnorr.

Later we'll see how to produce signing private keys of other types, like SSH.

Now we can sign our envelope:

```
SIGNED=`envelope sign --signer $PRVKEYS $ALICE_KNOWS_BOB`
```

Let's see what it looks like when formatted now:

```
envelope format $SIGNED

│ "Alice" [
│     "knows": "Bob"
│     'signed': Signature
│ ]
```

OK... there's a signature there now, but it's a new assertion on the subject of the envelope, "Alice". This means that any of the assertions can still be altered without invalidating the signature on the subject. But what if we want to sign the *whole* envelope, including the fact that she knows Bob?

Wrapping to the rescue! First we wrap the entire envelope in a new envelope, then we sign the wrapped envelope:

```
WRAPPED_SIGNED=`envelope subject type wrapped $ALICE_KNOWS_BOB | envelope sign --signer $PRVKEYS`
envelope format $WRAPPED_SIGNED

│ {
│     "Alice" [
│         "knows": "Bob"
│     ]
│ } [
│     'signed': Signature
│ ]
```

Now the entire contents of the envelope are signed, and if we send it to someone who has our public key, they can verify the signature:

```
envelope verify --verifier $PUBKEYS $WRAPPED_SIGNED

│ ur:envelope/lftpsplftpcsihfpjziniaihoytpcsihjejtjlktjktpcsiafwjlidoyaxtpcstansghhdfznltbglechtrkecemfhahkbrkcfzcasfnbbkpktzmsrvewtksknahmnpkinguktdwkgfrdklfrtdwpssamujtidcteovyongeamayftfxiaesfwceecoxueimmhwfrsyaidiycwdl
```

To facilitate piping commands, the `verify` command prints the input envelope if the validation is successful (unless the `--silent` flag is provided), and exits with an error condition if it is unsuccessful. Lets produce some incorrect public keys and try this:

```
BAD_PUBKEYS=`envelope generate prvkeys | envelope generate pubkeys`
envelope verify --verifier $BAD_PUBKEYS $WRAPPED_SIGNED

│ Error: could not verify a signature
```

Note that signing uses randomness. So even if you sign the same envelope twice with the same signer, the two resulting envelopes will not be the same although both signatures will verify against the same verifier.

## Signing with SSH

Specific applications may want to sign envelopes using SSH (Secure Shell) keys. The `envelope` tool supports several SSH key types, including Ed25519, RSA, DSA, ECDSA, and ML-DSA. The following example demonstrates how to sign an envelope using an Ed25519 key.

### Generating the SSH Keys

First we generate a `ur:crypto-prvkeys` containing an SSH signing private key:

```
SSH_PRVKEYS=`envelope generate prvkeys --signing ssh-ed25519`
echo $SSH_PRVKEYS

│ ur:crypto-prvkeys/lftansgotanehnkkadlsdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfxhkgyieemiohfinfpgtimhfjsgmjngskojljtdnfwiakpgegefxgleyeyjtfliefldnecgmgdihhtgaenioidfpfpfpfpgaiohgecksjlkofgkpiahsbkgsktfpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfxhkgyieemiohfinfpgtimhfjsgmjngskojljtdnfwiakpgegefxgleyeyjtfliefldnecgmgdihhtgaenioidfpbkfpfpfpfefwflihdngaimgakoesesgafgfgkseyeegmhdjyfdfdgmfdfeehjzenfyioeniojegmecisdldyflgrhfhkehdyecisfweokpfwhggafpkkglhgjoflhkkpdniniyeefgkkeebkjejegaeoidhsiahtdyidemjzfeesecjeimjsfwjkfpfpfpfpfpfpfefxfpktgyfgbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbktansgehdcxcyvtgsksmolttnfsnyrkspbnreclmdjehyfgfzjpmehgdlgmbzuehhintkceltfhpsvlnyks
```

Then we derive the corresponding `ur:crypto-pubkeys` containing the SSH signing public key:

Although it isn't required, it is best practice to include a comment when generating an SSH key. This comment is included in the public key and can be used to identify the key's owner.

```
SSH_PUBKEYS=`envelope generate pubkeys $SSH_PRVKEYS --comment "wolf@Wolfs-MacBook-Pro.local"`
echo $SSH_PUBKEYS

│ ur:crypto-pubkeys/lftanshftanehsksjnjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgageisfweokpfwhggafpkkglhgjoflhkkpdniniyeefgkkeejejegaeoidhsiahtdyidemjzfeesecjeimjsfwjkcxktjljziyfzhgjljziyjkdpgthsiafwjljljedpgdjpjldmjzjliahsjztansgrhdcxrlpskbchtposhlwzwyaasagspdweksswpkzczetehkfylysgbsbddttlpdwploftaorhfsgs
```

### Importing an SSH Signing Key from an Existing Key File

If you have an existing SSH key file, you can import it into the `envelope` tool. The following example demonstrates how to import an Ed25519 key from an existing file. In this example, the key is stored in a file named `test_ed25519` and is encrypted with the password `test`. You can either provide the encryption password using the `--password` command-line argument or by typing it when prompted.

```
cat ./ssh_objects/test_ed25519

│ -----BEGIN OPENSSH PRIVATE KEY-----
│ b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABAAFiGpGp
│ tFlJLG9vpkh+AcAAAAGAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iC
│ nLS7wO5GKTzybWCBkZWnO2d4KoBgAAAAoOtDEwxXcRHJWAxcYY5iJVdBCl5UGfLYYPK+Gb
│ ybsn7Oz1WlEL4RVorR854HqXRwch5BQ5d3KXYm5vEj5kiu4cHLOHqkFoSRrwY7F7yOwgYr
│ fNPS6xZvrhxx2spEtB95QROjGbgjEa1tNI4vXYArmK70tlpaEgsFMLfuXVZmlUZZS2M2eh
│ 2L7leSuWLZDPVlVSsNqEXD/bVVGHGw3c1Tf8Y=
│ -----END OPENSSH PRIVATE KEY-----
```

```
SSH_SIGNER=`envelope import <./ssh_objects/test_ed25519`
Key decryption password: test
echo $SSH_SIGNER

│ ur:signing-private-key/tanehnkkadotdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpfpfpfpgrfpkoecimimgwgsdnhkeebkkniofpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfwidimfejzghjljojoiofpdyiheeiojokkdykpetfykpgminjeetetjnehioiohtflhfjoknjyjtihfxjsfphkfpbkfpfpfpfefpemdldlecgrhkkokoenfgjliminktjsdngrfeisgaksgmjnfpiejeksjeeciogthdgseejkjojsknfwiogagtehkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflbkgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiofpfpfpfpfdfdiekoidflhtfphfeyesjkhtjtgtjyghhgfgimgyjneskohskkehgyiajnetkpidflesimhkhgktfwbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkgtctcyrd
```

Note that this is just the signing private key, not the full `ur:crypto-prvkeys` structure. However, it can still be used as a signer for signing envelopes.

### Signing with the SSH Key

Now that we have an SSH signing key, we can use it to sign an envelope. The following example demonstrates how to sign an envelope using the Ed25519 key we generated earlier.

Note that when signing with SSH keys, two additional options may be used: `--namespace` and `--hash-type`. The default namespace is `envelope`, and the default hash type is `sha256`. These defaults are fine for most cases.

```
WRAPPED=`envelope subject type wrapped $ALICE_KNOWS_BOB`
SSH_SIGNED=`envelope sign --signer $SSH_SIGNER $WRAPPED`
envelope format $SSH_SIGNED

│ {
│     "Alice" [
│         "knows": "Bob"
│     ]
│ } [
│     'signed': Signature(SshEd25519)
│ ]
```

This signed envelope looks just like the one we signed with Schnorr, except the signature type is `Signature(SshEd25519)`. Since the signature was made with an SSH key, it must be verified with the corresponding SSH verifier.

### Generating an SSH Verifier from an SSH Signing Key

To verify the signature, we need to generate an SSH verifier from the SSH public key. The following example demonstrates how to generate an SSH verifier from the Ed25519 key we generated earlier.

```
SSH_VERIFIER=`envelope generate pubkeys $SSH_SIGNER`
echo $SSH_VERIFIER

│ ur:signing-public-key/tanehsksjnjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafgkpgtguhfgwinjnjnfpfygmeminfxjtgsguemktgwecflgrghknkkidhgfxfwjehthgjtgweyieeegrjlfwiocxktjljziyfzhgjljziyjkdpgthsiafwjljljedpgdjpjldmjzjliahsjzmybngyfs
```

### Verifying the SSH Signature

Now that we have an SSH verifier, we can use it to verify the signature. The following example demonstrates how to verify the signature using the Ed25519 verifier we generated earlier.

```
envelope verify --silent --verifier $SSH_VERIFIER $SSH_SIGNED
```

## Exporting SSH Keys

The `envelope` tool can export SSH keys to Open SSH format.

### Exporting an SSH Private Key

The following example demonstrates how to export an SSH private key to stdout. The private key can be saved to a file by redirecting the output to a file.

```
envelope export $SSH_SIGNER

│ -----BEGIN OPENSSH PRIVATE KEY-----
│ b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
│ QyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYAAAAKAv5jjOL+Y4
│ zgAAAAtzc2gtZWQyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYA
│ AAAEA7//5KYvv6Fojiwq+KEhIxRmAdkxk5gMXL4spqzBgIM1uMSVOimmADR7iCnLS7wO5G
│ KTzybWCBkZWnO2d4KoBgAAAAHHdvbGZAV29sZnMtTWFjQm9vay1Qcm8ubG9jYWwB
│ -----END OPENSSH PRIVATE KEY-----
```

If desired, the private key can be encrypted with a password. To encrypt, add the `--encrypt` switch and either provide the password on the command line using the `--password` option or type it when prompted.

```
envelope export --encrypt --password "test" $SSH_SIGNER

│ -----BEGIN OPENSSH PRIVATE KEY-----
│ b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABAkeiUPXe
│ 5luC/l0qmvQKmjAAAAEAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iC
│ nLS7wO5GKTzybWCBkZWnO2d4KoBgAAAAoNnC0+/P6YpAqW+Q9veI09QztWUUuOPJUriP7S
│ 9tGQ25b3+MT3j4KUD6UUaE88LrmXPtNJNBWsZswpdpNEiqJahDAkeH/elnQft15jFB5Alf
│ /GUsxHcPxlHu+I590t/j3mRVVVfzKw/+d+pJdm8VfgZH+j5Zn66bVa5xoxVhVa51IBK7st
│ BYyNv4/ibNLxYpOt4aCpaqoOmdFflYAjy36j8=
│ -----END OPENSSH PRIVATE KEY-----
```

### Exporting an SSH Public Key

The following example demonstrates how to export an SSH public key to stdout. The public key can be saved to a file by redirecting the output to a file.

```
envelope export $SSH_VERIFIER

│ ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFuMSVOimmADR7iCnLS7wO5GKTzybWCBkZWnO2d4KoBg wolf@Wolfs-MacBook-Pro.local
```

### Exporting an SSH Signature

If you have a `ur:signature` object that was generated with an SSH key, you can export it to a file in OpenSSH format. The following example demonstrates how to export an SSH signature to stdout. The signature can be saved to a file by redirecting the output to a file.

```
SSH_SIGNATURE="ur:signature/taneidkkaddsdpdpdpdpdpfwfeflgaglcxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkgoehglgagodyjzfdfpfpfpfpfpgyfpfpfpfygtfpfpfpfpgsiaeogljlgshghfjegtimgoehgtghjefpfpfpfpiohgeeksgegoengrhshkfpglfdkpgagriajygskofpemjehkjogdgdbkgejyhkgaflgmjzhsiaemhteoiojsioflfpfpfpfpfpfehtjnjzjkhtgyfpfpfpfpfpfpfpfpfpfliaeyisisglghfekkfpfpfpfpgoktfpfpfpfpjykniaeyiojyhthggykkglghgoksbkgwgyfpfpfpfefpenglkseeflhkghknjkeogegsdlfedldljpkpgeiyeyfpjlidfyiyjkjtiogmishtetgyghdlfwhshtjtgsjliyiejpgmfpgogagtgdideegresjyjlfxfxgaghimflbkguiofxkpgyjkhfdydlisgogojpiaimemkofwfxiohkgrbkdpdpdpdpdpfeglfycxgugufdcxgugaflglfpghgogmfedpdpdpdpdpbkcllebeje"
envelope export $SSH_SIGNATURE

│ -----BEGIN SSH SIGNATURE-----
│ U1NIU0lHAAAAAQAAADMAAAALc3NoLWVkMjU1MTkAAAAgW4xJU6KaYANHuIKctLvA7kYpPP
│ JtYIGRlac7Z3gqgGAAAAAEZmlsZQAAAAAAAAAGc2hhNTEyAAAAUwAAAAtzc2gtZWQyNTUx
│ OQAAAEA6Nx4GYTzs3JL/E//ruJf2AobDfsngRhZ8QT/BaZnLofdrRAUIMPb4K9toCCITjG
│ SgCuQsV0/hUUrcj7vBCgYK
│ -----END SSH SIGNATURE-----
```

## Getting Information about SSH Keys

You can use the `info` subcommand to retrieve information about an SSH key or signature, including its type, comment, and fingerprint.

```
envelope info $SSH_SIGNER

│ Format: ur:signing-private-key
│ Description: SSH Signing Private Key
│ Algorithm: ssh-ed25519
│ Fingerprint: SHA256:wonJHWbmVYaZyry76VO/QM50PRqBKbFB1y3oBAGRtuY
│ +----[ED25519]----+
│ |    o=*o.*o.     |
│ |    o  =*=o .    |
│ |   . +B++ ..     |
│ |   .oX+=.  o     |
│ |   o+ *.S o o    |
│ |    E .*.. o .   |
│ |       o+..      |
│ |      o. ..      |
│ |     .+o  ..     |
│ +----[SHA256]-----+
```
QyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYAAAAKAv5jjOL+Y4
zgAAAAtzc2gtZWQyNTUxOQAAACBbjElToppgA0e4gpy0u8DuRik88m1ggZGVpztneCqAYA
AAAEA7//5KYvv6Fojiwq+KEhIxRmAdkxk5gMXL4spqzBgIM1uMSVOimmADR7iCnLS7wO5G
KTzybWCBkZWnO2d4KoBgAAAAHHdvbGZAV29sZnMtTWFjQm9vay1Qcm8ubG9jYWwB
-----END OPENSSH PRIVATE KEY-----
```
