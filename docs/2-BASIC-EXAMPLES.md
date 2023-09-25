# envelope - Basic Examples

**NOTE:** Most of this documentation has *not* been updated to reflect the new command line syntax.

This document walks you through a set of basic examples using the `envelope` command line tool. There are several companion documents that contain more complex examples.

**See Associated Video:**

[![Gordian Envelope CLI - 2 - Examples](https://img.youtube.com/vi/LYjtuBO1Sgw/mqdefault.jpg)](https://www.youtube.com/watch?v=LYjtuBO1Sgw)

## Notation

Throughout this document, commands entered into the shell are annotation with a right-pointing hand:

```bash
👉
echo Hello.
```

Shell output is annotated with a left-pointing hand:

```
👈
Hello.
```

Data sent through the Internet is annotated like this:

Alice ➡️ ☁️ ➡️ Bob

## Common structures used by the examples

These examples define a common plaintext, and `ARID`s and `PrivateKeyBase` objects for *Alice*, *Bob*, *Carol*, *ExampleLedger*, and *The State of Example*, each with a corresponding `PublicKeyBase`.

```bash
👉
PLAINTEXT_HELLO="Hello."

ALICE_ARID="ur:arid/hdcxtygshybkzcecfhflpfdlhdonotoentnydmzsidmkindlldjztdmoeyishknybtbswtgwwpdi"
ALICE_SEED="ur:crypto-seed/oyadgdlfwfdwlphlfsghcphfcsaybekkkbaejkhphdfndy"
ALICE_PRVKEYS="ur:crypto-prvkeys/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct"
ALICE_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnwksorlbd"

BOB_ARID="ur:arid/hdcxdkreprfslewefgdwhtfnaosfgajpehhyrlcyjzheurrtamfsvolnaxwkioplgansesiabtdr"
BOB_SEED="ur:crypto-seed/oyadgdcsknhkjkswgtecnslsjtrdfgimfyuykglfsfwtso"
BOB_PRVKEYS="ur:crypto-prvkeys/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva"
BOB_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwphejzwnmhhf"

CAROL_ARID="ur:arid/hdcxamstktdsdlplurgaoxfxdijyjysertlpehwstkwkskmnnsqdpfgwlbsertvatbbtcaryrdta"
CAROL_SEED="ur:crypto-seed/oyadgdlpjypepycsvodtihcecwvsyljlzevwcnmepllulo"
CAROL_PRVKEYS="ur:crypto-prvkeys/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos"
CAROL_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwsbkbotbs"

LEDGER_ARID="ur:arid/hdcxbatnhhvdnydnhfcfvlltwkmhlncydmjpbygomhdtqdqdintkmkzojyndterdnyhlvdnbenft"
LEDGER_SEED="ur:crypto-seed/oyadgdtbjkknqdglgllupfhpimrtecytzopdcyjeoediae"
LEDGER_PRVKEYS="ur:crypto-prvkeys/gdtbjkknqdglgllupfhpimrtecytzopdcyzthnltdl"
LEDGER_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxmdoyskbbkshtkbgdmtqdmwtadiecmysksrfyoegdcfhnrkctrehfemwdtswdmugstansgrhdcxssdwgwtipdgazmaeftgeaddaiyectikbjebtckfzsbzoqdolrhwfwmihprgdecdtemryfdnt"

STATE_ARID="ur:arid/hdcxaaenfsheytmseorfbsbzktrdrdfybkwntkeegetaveghzstattdertbswsihahvspllbghcp"
STATE_SEED="ur:crypto-seed/oyadgdfmmojswkjzuylpotrelrvdcpbdmsincssfolqdpk"
STATE_PRVKEYS="ur:crypto-prvkeys/gdfmmojswkjzuylpotrelrvdcpbdmsincshpiebwlp"
STATE_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxsegystjeisrshnyattgsswclpdmnsfzsgwcphgskdyuyahhecfrlhyvddsonbbsatansgrhdcxrfretiztzsoectchdsdizslpwyticsleonoxwliywfvsmhclrdplcplsfrnnptishywnpfdt"
```

## Example 1: Plaintext

In this example no signing or encryption is performed.

Alice sends a plaintext message to Bob.

```bash
👉
HELLO_ENVELOPE=`envelope subject $PLAINTEXT_HELLO`
echo $HELLO_ENVELOPE
```

```
👈
ur:envelope/tpcsiyfdihjzjzjldmprrhtypk
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and reads the message.

```bash
👉
envelope extract $HELLO_ENVELOPE
```

```
👈
Hello.
```

## Example 2: Signed Plaintext

This example demonstrates the signature of a plaintext message.

Alice sends a signed plaintext message to Bob.

```bash
👉
SIGNED_ENVELOPE=`envelope subject $PLAINTEXT_HELLO | envelope sign --prvkeys $ALICE_PRVKEYS`
echo $SIGNED_ENVELOPE
```

```
👈
ur:envelope/lftpcsiyfdihjzjzjldmoyaxtpcstansghhdfzlgtnuennvwryhfzsjkcxmuylotykmsemgswfdeaycngekpoewyldostdfdfmmolsvltewfftbapytotolgioteswatwkkpbgdrmohfdimejltesahfzerytylytentrtwycpahih
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope $SIGNED_ENVELOPE
```

```
👈
"Hello." [
    verifiedBy: Signature
]
```

Bob verifies Alice's signature. Note that the `--silent` flag is used to generate no output on success.

```bash
👉
envelope verify --silent $SIGNED_ENVELOPE --pubkeys $ALICE_PUBKEYS
```

Bob extracts the message.

```bash
👉
envelope extract $SIGNED_ENVELOPE
```

```
👈
Hello.
```

Confirm that it wasn't signed by Carol. Note that a failed verification results in an error condition.

```bash
👉
envelope verify $SIGNED_ENVELOPE --pubkeys $CAROL_PUBKEYS
```

```
👈
Error: unverifiedSignature
```

Confirm that it was signed by Alice OR Carol.

```bash
👉
envelope verify --silent $SIGNED_ENVELOPE --threshold 1 --pubkeys $ALICE_PUBKEYS --pubkeys $CAROL_PUBKEYS
```

Confirm that it was not signed by Alice AND Carol.

```bash
👉
envelope verify $SIGNED_ENVELOPE --threshold 2 --pubkeys $ALICE_PUBKEYS --pubkeys $CAROL_PUBKEYS
```

```
👈
Error: unverifiedSignature
```

## Example 3: Multisigned Plaintext

This example demonstrates a plaintext message signed by more than one party.

Alice and Carol jointly send a signed plaintext message to Bob.

```bash
👉
MULTISIGNED_ENVELOPE=`envelope subject $PLAINTEXT_HELLO | envelope sign --prvkeys $ALICE_PRVKEYS --prvkeys $CAROL_PRVKEYS`
echo $MULTISIGNED_ENVELOPE
```

```
👈
ur:envelope/lstpcsiyfdihjzjzjldmoyaxtpcstansghhdfzdmgusawmqzsakictrdoxbkayclsgvyeoemamtifdcehlzoimdnwflnteeswsndptqdgatdjtmshlfebtwzutcmjnzmwfwtpkmtosfhplahjlwyhgwzbsmkfxcatptpndoyaxtpcstansghhdfzcmoytlluprtkiocxveehbbdtjlpmuovlwlgoeylnflprwlltinemeshgcsgocldiinsplfkbvovlsesesrcksttefehyonltptbwrtbgvoaelffgbkahjlchmnbnfytibwsbbykn
```

Alice & Carol ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.
```bash
👉
envelope $MULTISIGNED_ENVELOPE
```

```
👈
"Hello." [
    verifiedBy: Signature
    verifiedBy: Signature
]
```

Bob verifies the message was signed by both Alice and Carol.

```bash
👉
envelope verify --silent $MULTISIGNED_ENVELOPE --pubkeys $ALICE_PUBKEYS --pubkeys $CAROL_PUBKEYS
```

Bob extracts the message.

```bash
👉
envelope extract $MULTISIGNED_ENVELOPE
```

```
👈
Hello.
```

## Example 4: Symmetric Encryption

This examples debuts the idea of an encrypted message, based on a symmetric key shared between two parties.

Alice and Bob have agreed to use this key:

```bash
👉
KEY=`envelope generate key`
echo $KEY
```

```
👈
ur:crypto-key/hdcxgrpkwdceueltmkdwrsjnfmsgftzctirdltlgfwsakiiaheckdmrplbwectsnjslrislnaohk
```

Alice sends a message encrypted with the key to Bob.

```bash
👉
ENCRYPTED_ENVELOPE=`envelope subject $PLAINTEXT_HELLO | envelope encrypt --key $KEY`
echo $ENCRYPTED_ENVELOPE
```

```
👈
ur:envelope/tansfwlrgrcmdrathhnbhylpindnkkkkgsmkndmtkgjobbcmtodepltdckgddmlbgudloejzbgtatsdigssastdtwznnhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhksbjymdbk
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope $ENCRYPTED_ENVELOPE
```

```
👈
ENCRYPTED
```

Bob decrypts the message and extracts its subject.

```bash
👉
DECRYPTED=`envelope decrypt $ENCRYPTED_ENVELOPE --key $KEY`
envelope extract $DECRYPTED
```

```
👈
Hello.
```

Can't read with incorrect key.

```bash
👉
envelope decrypt $ENCRYPTED_ENVELOPE --key `envelope generate key`
```

```
👈
Error: invalidAuthentication
```

## Example 5: Sign-Then-Encrypt

This example combines the previous ones, first signing, then encrypting a message with a symmetric key.

Alice and Bob have agreed to use this key.
```bash
👉
KEY=`envelope generate key`
echo $KEY
```

```
👈
ur:crypto-key/hdcxvspyfginmoamlkskonkpecctjyjemwchpynybzutfzfltpcxdpfrkkcleosgineylpwybege
```

Alice signs a plaintext message, wraps it so her signature will also be encrypted, then encrypts it.

```bash
👉
SIGNED_ENCRYPTED=`envelope subject $PLAINTEXT_HELLO | envelope sign --prvkeys $ALICE_PRVKEYS | envelope subject --wrapped | envelope encrypt --key $KEY`
echo $SIGNED_ENCRYPTED
```

```
👈
ur:envelope/tansfwlrhdhgkbiydpatutptjybdaxynutvoaaqzrddlktsbjssncfdnbgrhzezoonnbcldtjkfelslowdaevezsbzimkowznsembdbnrkdkzmkkihbnoxtauobbhpnnwmoykeisvodrhnwndmptkkdigstefefteeeourmhpsyllscspsjojlnncagslbgsrtlnkitdtkbaehvsvtmtgdynrhvljthklbgesnehfwmelemwtyztfehddatansfphdcxlnfxhyrkdlidecfgksehbyvyrtjsoxeymunlqzcywpnbgmbkdwpkzccmbnnnfpmncactjsbe
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope, and examines its contents.

```bash
👉
envelope $SIGNED_ENCRYPTED
```

```
👈
ENCRYPTED
```

Bob decrypts it using the shared key, and then examines the decrypted envelope's contents.

```bash
👉
DECRYPTED=`envelope decrypt $SIGNED_ENCRYPTED --key $KEY`
envelope $DECRYPTED
```

```
👈
{
    "Hello." [
        verifiedBy: Signature
    ]
}
```

Bob unwraps the inner envelope, verifies Alice's signature, and then extracts the message.

```bash
👉
envelope extract --wrapped $DECRYPTED | envelope verify --pubkeys $ALICE_PUBKEYS | envelope extract
```

```
👈
Hello.
```

Attempting to verify the wrong key exits with an error.

```bash
👉
envelope extract --wrapped $DECRYPTED | envelope verify --pubkeys $CAROL_PUBKEYS
```

```
👈
Error: unverifiedSignature
```

## Example 6: Encrypt-Then-Sign

It doesn't actually matter whether the `encrypt` or `sign` command comes first, as the `encrypt` command transforms the subject into its encrypted form, which carries a digest of the plaintext subject, while the `sign` command only adds an assertion with the signature of the hash as the object of the assertion.

Similarly, the `decrypt` method used below can come before or after the `verify` command, as `verify` checks the signature against the subject's digest, which is explicitly present when the subject is in encrypted form and can be calculated when the subject is in plaintext form. The `decrypt` command transforms the subject from its encrypted form to its plaintext form, and also checks that the decrypted plaintext has the same hash as the one associated with the encrypted subject.

The end result is the same: the subject is encrypted and the signature can be checked before or after decryption.

The main difference between this order of operations and the sign-then-encrypt order of operations is that with sign-then-encrypt, the decryption *must* be performed first before the presence of signatures can be known or checked. With this order of operations, the presence of signatures is known before decryption, and may be checked before or after decryption.

Alice and Bob have agreed to use this key.

```bash
👉
KEY=`envelope generate key`
echo $KEY
```

```
👈
ur:crypto-key/hdcxrldltdpsdynswnmeuobawkkofecturmyrtpfrfbeaerosfaywpktfnlyylntprssosrhgags
```

Alice encrypts a plaintext message, then signs it.

```bash
👉
ENCRYPTED_SIGNED=`envelope subject $PLAINTEXT_HELLO | envelope encrypt --key $KEY | envelope sign --prvkeys $ALICE_PRVKEYS`
echo $ENCRYPTED_SIGNED
```

```
👈
ur:envelope/lftansfwlrgrgoahfeptadlpbzvtisltyngstkglwlrdrohdptbapfehwycngdpltighsbonwtmdwskofseckkltssykdphddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyaxtpcstansghhdfzvykbnbuysfsbwfvsfxmomklrfynbnnbemwzovtwmwzuykpuyneweurpkemuyknfhcxbneygyiybgldjphdkolnpahdglwyeeykjolftlpmwmvdldwfjyisgwjpsopsmtdebbtnde
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope $ENCRYPTED_SIGNED
```

```
👈
ENCRYPTED [
    verifiedBy: Signature
]
```

Bob verifies Alice's signature, decrypts the message, then extracts the message.

```bash
👉
envelope verify $ENCRYPTED_SIGNED --pubkeys $ALICE_PUBKEYS | \
    envelope decrypt --key $KEY | \
    envelope extract
```

```
👈
Hello.
```

## Example 7: Multi-Recipient Encryption

This example demonstrates an encrypted message sent to multiple parties.

Alice encrypts a message so that it can only be decrypted by Bob or Carol.

```bash
👉
ENVELOPE_TO=`envelope subject $PLAINTEXT_HELLO | envelope encrypt --recipient $BOB_PUBKEYS --recipient $CAROL_PUBKEYS`
echo $ENVELOPE_TO
```

```
👈
ur:envelope/lstansfwlrgrjtjsuopmswjpjngumtoypygsfhfyroskcsjpqdwskbzcjyzmgdgmasctynlpmohtsktsntaayarendwymkhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyahtpcstansgulftansfwlshddagdbafwmhrnmkpedaimbyfnrnenspsolraycngoswssdwaoswbkzmdkrhdehylufhkbskihlyoygsayrendwliycliywkwlnynesggdaakbmseonssbftzswykoesmhemsrcnpstansgrhdcxhhdtnnsputhepfnlwklrkbisvdaxbecppscatyfygdwzlnveoxmtpltsssqzssayoyahtpcstansgulftansfwlshddahhtkjlcpimbtztmhtdlrenftcsrechfsseytosbwzsnlcprddwlnspbbmeprbwgababecwrlwzgsvwtelgtonyoxhsflpllpkbemgdsrfxrneolyclnlcnvegmkngrdtktfwvwtansgrhdcxoncwlupeltdpprsgiantlaprlosrcfgytynnoeynetpamdaxfgossbvacnbzfnbbskknjzde
```

Alice ➡️ ☁️ ➡️ Bob

Alice ➡️ ☁️ ➡️ Carol

Bob receives the envelope and examines its structure:

```bash
👉
envelope $ENVELOPE_TO
```

```
👈
ENCRYPTED [
    hasRecipient: SealedMessage
    hasRecipient: SealedMessage
]
```

Bob decrypts and reads the message.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $BOB_PRVKEYS | envelope extract
```

```
👈
Hello.
```

Carol decrypts and reads the message.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $CAROL_PRVKEYS | envelope extract
```

```
👈
Hello.
```

Alice didn't encrypt it to herself, so she can't read it.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $ALICE_PRVKEYS
```

```
👈
Error: invalidRecipient
```

## Example 8: Signed Multi-Recipient Encryption

This example demonstrates a signed, then encrypted message, sent to multiple parties.

Alice signs a message, and then encrypts it so that it can only be decrypted by Bob or Carol.

```bash
👉
ENVELOPE_SIGNED_TO=`envelope subject $PLAINTEXT_HELLO | envelope sign --prvkeys $ALICE_PRVKEYS | envelope encrypt --recipient $BOB_PUBKEYS --recipient $CAROL_PUBKEYS`
echo $ENVELOPE_SIGNED_TO
```

```
👈
ur:envelope/lrtansfwlrgrvlwmceehwliyaekswkspnlgsbzgwcfmektrleslpwymwmhmtgdwfdybacwtkpemnwdfnvosoknvswpissbhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyahtpcstansgulftansfwlshddaqzhpvdckmsghpllniavoneprstbkoygedwwnknmwidtebzmujpotskecmogebtnbrsgoaskedpgsontstbdpbzknisoyqdryvwjegdckbazchlhsmwpkgulpmunywzlovoatgttansgrhdcxwlsovwyagswnhhsolopeosbgloswwfrtidgukkwtglmkttzofzaoonvwdstbckahoyahtpcstansgulftansfwlshddalbaxtaaofnhtbyurmoftpabefmnldwhyqzditernhtjnwmlkjpskvtjtlgosjlctjphnbsfnyngsbekbhdqdaehdurwfdityathlgdvdjpvdprbynswzpeghrknnutfwuesplptansgrhdcxbsrykelsaybaykjlaossnlwntyfrrtjztiguehwmhdmolnfgrystahztpegecpieoyaxtpcstansghhdfzvlqdknntatgogmjogyaybwtpyacxvebacpfdmdztvdgrdsndpfindpbwgtfdwfflkotosfrflkoyidfhhfzcdlvddarpsndtfplfdlrnroldzouyfpbwryosahiddpurdmdmimiy
```

Alice ➡️ ☁️ ➡️ Bob

Alice ➡️ ☁️ ➡️ Carol

Bob receives the envelope and examines its structure:

```bash
👉
envelope $ENVELOPE_SIGNED_TO
```

```
👈
ENCRYPTED [
    hasRecipient: SealedMessage
    hasRecipient: SealedMessage
    verifiedBy: Signature
]
```

Bob verifies Alice's signature, then decrypts and reads the message

```bash
👉
envelope verify $ENVELOPE_SIGNED_TO --pubkeys $ALICE_PUBKEYS | envelope decrypt --recipient $BOB_PRVKEYS | envelope extract
```

```
👈
Hello.
```
