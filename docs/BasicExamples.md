# `envelope` - Basic Examples

This document walks you through a set of basic examples using the `envelope` command line tool. There are several companion documents that contain more complex examples.

**See Associated Video:**

**NOTE:** These videos show the command line syntax of the Swift `envelope` command line tool. The Rust-based `envelope` tool has a slightly different syntax, but the meaning of the commands is the same.

[![Gordian Envelope CLI - 1 - Overview](https://img.youtube.com/vi/1Z3Z4Z2Z2Z4/mqdefault.jpg)](https://www.youtube.com/watch?v=1Z3Z4Z2Z2Z4)
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

These examples define a common plaintext, and `ARID`s and `PrivateKeyBase` objects for *Alice*, *Bob*, *Carol*, *ExampleLedger*, and *The State of Example*, each with a corresponding `PublicKeys`.

```bash
👉
PLAINTEXT_HELLO="Hello."

ALICE_ARID="ur:arid/hdcxtygshybkzcecfhflpfdlhdonotoentnydmzsidmkindlldjztdmoeyishknybtbswtgwwpdi"
ALICE_SEED="ur:seed/oyadgdlfwfdwlphlfsghcphfcsaybekkkbaejkhphdfndy"
ALICE_PRVKEY_BASE="ur:crypto-prvkey-base/gdlfwfdwlphlfsghcphfcsaybekkkbaejksfnynsct"
ALICE_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxrdhgfsfsfsosrloebgwmfrfhsnlskegsjydecawybniadyzovehncacnlbmdbesstansgrhdcxytgefrmnbzftltcmcnaspaimhftbjehlatjklkhktidrpmjobslewkfretcaetbnwksorlbd"

BOB_ARID="ur:arid/hdcxdkreprfslewefgdwhtfnaosfgajpehhyrlcyjzheurrtamfsvolnaxwkioplgansesiabtdr"
BOB_SEED="ur:seed/oyadgdcsknhkjkswgtecnslsjtrdfgimfyuykglfsfwtso"
BOB_PRVKEY_BASE="ur:crypto-prvkey-base/gdcsknhkjkswgtecnslsjtrdfgimfyuykgbzbagdva"
BOB_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxndctnnflynethhhnwdkbhtehhdosmhgoclvefhjpehtaethkltsrmssnwfctfggdtansgrhdcxtipdbagmoertsklaflfhfewsptrlmhjpdeemkbdyktmtfwnninfrbnmwonetwphejzwnmhhf"

CAROL_ARID="ur:arid/hdcxamstktdsdlplurgaoxfxdijyjysertlpehwstkwkskmnnsqdpfgwlbsertvatbbtcaryrdta"
CAROL_SEED="ur:seed/oyadgdlpjypepycsvodtihcecwvsyljlzevwcnmepllulo"
CAROL_PRVKEY_BASE="ur:crypto-prvkey-base/gdlpjypepycsvodtihcecwvsyljlzevwcnamjzdnos"
CAROL_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxeckpgwvyasletilffeeekbtyjlzeimmtkslkpadrtnnytontpyfyeocnecstktkttansgrhdcxoyndtbndhspebgtewmgrgrgriygmvwckkkaysfzozclbgendfmhfjliorteenlbwsbkbotbs"

LEDGER_ARID="ur:arid/hdcxbatnhhvdnydnhfcfvlltwkmhlncydmjpbygomhdtqdqdintkmkzojyndterdnyhlvdnbenft"
LEDGER_SEED="ur:seed/oyadgdtbjkknqdglgllupfhpimrtecytzopdcyjeoediae"
LEDGER_PRVKEY_BASE="ur:crypto-prvkey-base/gdtbjkknqdglgllupfhpimrtecytzopdcyzthnltdl"
LEDGER_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxmdoyskbbkshtkbgdmtqdmwtadiecmysksrfyoegdcfhnrkctrehfemwdtswdmugstansgrhdcxssdwgwtipdgazmaeftgeaddaiyectikbjebtckfzsbzoqdolrhwfwmihprgdecdtemryfdnt"

STATE_ARID="ur:arid/hdcxaaenfsheytmseorfbsbzktrdrdfybkwntkeegetaveghzstattdertbswsihahvspllbghcp"
STATE_SEED="ur:seed/oyadgdfmmojswkjzuylpotrelrvdcpbdmsincssfolqdpk"
STATE_PRVKEY_BASE="ur:crypto-prvkey-base/gdfmmojswkjzuylpotrelrvdcpbdmsincshpiebwlp"
STATE_PUBKEYS="ur:crypto-pubkeys/lftanshfhdcxsegystjeisrshnyattgsswclpdmnsfzsgwcphgskdyuyahhecfrlhyvddsonbbsatansgrhdcxrfretiztzsoectchdsdizslpwyticsleonoxwliywfvsmhclrdplcplsfrnnptishywnpfdt"
```

## Example 1: Plaintext

In this example no signing or encryption is performed.

Alice sends a plaintext message to Bob.

```bash
👉
HELLO_ENVELOPE=`envelope subject type string $PLAINTEXT_HELLO`
echo $HELLO_ENVELOPE
```

```
👈
ur:envelope/tpsoiyfdihjzjzjldmksbaoede
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and reads the message.

```bash
👉
envelope extract string $HELLO_ENVELOPE
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
SIGNED_ENVELOPE=`envelope subject type string $PLAINTEXT_HELLO | envelope sign -s $ALICE_PRVKEY_BASE`
echo $SIGNED_ENVELOPE
```

```
👈
ur:envelope/lftpsoiyfdihjzjzjldmoyaxtpsotansghhdfztimhtswewlimbszsbnlrrsperkwznlmsehghlpbejooxdtdiyazmglcxiekblplrfrmwgajllnnnrfnbcxmswmgtaoykvdcnsalblgfhgekobsjzotsawsprhkeybshlidplgesr
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope format $SIGNED_ENVELOPE
```

```
👈
"Hello." [
    'signed': Signature
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
envelope extract string $SIGNED_ENVELOPE
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
Error: could not verify a signature
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
Error: could not verify a signature
```

## Example 3: Multi-signed Plaintext

This example demonstrates a plaintext message signed by more than one party.

Alice and Carol jointly send a signed plaintext message to Bob.

```bash
👉
MULTISIGNED_ENVELOPE=`envelope subject type string $PLAINTEXT_HELLO | envelope sign -s $ALICE_PRVKEY_BASE --prvkeys $CAROL_PRVKEY_BASE`
echo $MULTISIGNED_ENVELOPE
```

```
👈
ur:envelope/lstpsoiyfdihjzjzjldmoyaxtpsotansghhdfzbgqdbglkfhjlrpimaawndiiortahtiethdrdotrkmerfaxatayoyjnkgmsmwgspegduehhswfnprtsayesrtrpjpvlmnaslpnsryqdflfhhypasgpaguidstfpbnhkmkoyaxtpsotansghhdfzksdedssrtdcywsneluvthelnadoxvydmfnfwrlfpwpqddltbetleoyvstalsykfgmngllonegrlefptivyglhttkvdeopkytyatpfrdibsvydseeurticnbkjtgeyncflsmdctrl
```

Alice & Carol ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope format $MULTISIGNED_ENVELOPE
```

```
👈
"Hello." [
    'signed': Signature
    'signed': Signature
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
envelope extract string $MULTISIGNED_ENVELOPE
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
ur:crypto-key/hdcxndcpaxtllrgtclpasaytgoynadhnssbnlaytyluokelebggokpkkgodyfpchlnwnnehpmtte
```

Alice sends a message encrypted with the key to Bob.

```bash
👉
ENCRYPTED_ENVELOPE=`envelope subject type string $PLAINTEXT_HELLO | envelope encrypt --key $KEY`
echo $ENCRYPTED_ENVELOPE
```

```
👈
ur:envelope/tansfwlrgrlubnrsgykoyntaskktbbrsgssbynrfkiosdtkpwmvsylytrngdwskgtdlarycsntpdvtvoguurgedlosprhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhksprsbghn
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope format $ENCRYPTED_ENVELOPE
```

```
👈
ENCRYPTED
```

Bob decrypts the message and extracts its subject.

```bash
👉
DECRYPTED=`envelope decrypt $ENCRYPTED_ENVELOPE --key $KEY`
envelope extract string $DECRYPTED
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
Error: decrypt failed
```

## Example 5: Password Encryption

Alice encrypts a message using a password she chooses.

```bash
👉
ENCRYPTED_ENVELOPE=`envelope subject type string $PLAINTEXT_HELLO | envelope encrypt --password`
```

```
👈
Encryption password: <Alice enters 'test'>
```

```bash
👉
echo $ENCRYPTED_ENVELOPE
```

```
👈
ur:envelope/lftansfwlrgrlbhykttnrnluldfemointygsbkaorolupsehetkoytbtkslsgdrnolwmfyuednmocmmhwzbkytgldrlurdhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoybwtpsotanshptansfwlrhdcxfxpdnshtpresbdskveremkmdqdvyzmftckdyhtuocsjtlrtopfyljoykgensbemwgsluskplhsbzaewnryjteedmolgdqzlovdsssgknclrogtnyiolnmhdthfnthflfaxtansgmgdbsdimwlegesbvduymsutkbrdghdnghrsvotsndda
```

Later, Alice decrypts the message using the same password.

```bash
👉
DECRYPTED=`envelope decrypt $ENCRYPTED_ENVELOPE --password`
```

```
👈
Decryption password: <Alice enters 'test' again>
```

```bash
👉
envelope extract string $DECRYPTED
```

```
👈
Hello.
```

## Example 6: SSH Agent Encryption

Alice generates an Ed25519 key pair and adds it to her SSH agent.

```bash
👉
ssh-keygen -t ed25519 -f ~/.ssh/alice_ed25519 -C "alice@localhost"
```

```
👈
Generating public/private ed25519 key pair.
Enter passphrase for "/Users/alice/.ssh/alice_ed25519" (empty for no passphrase):
Enter same passphrase again:
Your identification has been saved in /Users/alice/.ssh/alice_ed25519
Your public key has been saved in /Users/alice/.ssh/alice_ed25519.pub
The key fingerprint is:
SHA256:yufAfhk0j/2Z20IhlBr1ujmB6Otys43V1DBEMRbEk8M alice@localhost
The key's randomart image is:
+--[ED25519 256]--+
|         *X=     |
|        .oEo     |
|         +oo.    |
|       .+..=.    |
|      ..S=+...   |
|     + .ooo+.    |
|      * oo=o o   |
|    ..oOo  .=.   |
|     +*+o   .o.  |
+----[SHA256]-----+
```

Alice adds her private key to the SSH agent.

```bash
👉
ssh-add ~/.ssh/alice_ed25519
```

```
👈
Identity added: /Users/alice/.ssh/alice_ed25519 (alice@localhost)
```

Alice lists the keys in her SSH agent to confirm it was added.

```bash
👉
ssh-add -L
```

```
👈
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILvLgo1Ep8md2MTAxLCM+e9MLFw0vDdfa5N4BGwvk893 alice@localhost
```

Alice encrypts a message using her SSH agent.

```bash
👉
ENCRYPTED=`envelope subject type string $PLAINTEXT_HELLO | envelope encrypt --ssh-id alice@localhost`
echo $ENCRYPTED
```

```
👈
ur:envelope/lftansfwlrgrzotttdghayfyfsdpnbiymtgsgartdytlptsntpielerenshfgdhyvowkcaatfmehrdfyweiebbieeekkgwhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoybwtpsotanshptansfwlrhdcxtirdvoqdberhinrewklahemotspkonuyzeytnebewpemjkrtylbndleoinynsppagslboeahutvtdlplynylwkbtingdnykofmmuhphnhsgacfihzobkgoaxwnkghddslsaatansgmgdmnceetcajllfrnkkotverohlpelrlrltjlhsjziniaihfzjzjliahsjzisjljkjydatkhenl
```

```bash
👉
envelope format $ENCRYPTED
```

```
👈
ENCRYPTED [
    'hasSecret': EncryptedKey(SSHAgent("alice@localhost"))
]
```

Later, Alice decrypts the message using her SSH agent.

```bash
👉
DECRYPTED=`envelope decrypt $ENCRYPTED --ssh-id alice@localhost`
envelope extract string $DECRYPTED
```

```
👈
Hello.
```

## Example 7: Sign-Then-Encrypt

This example combines the previous ones, first signing, then encrypting a message with a symmetric key.

Alice and Bob have agreed to use this key.

```bash
👉
KEY=`envelope generate key`
echo $KEY
```

```
👈
ur:crypto-key/hdcxhkwenldmtpvyuyjthgjnwpurdndrhhesatlsqddrrkbtclfmdreooscmamdsdsvyvyrpmojn
```

Alice signs a plaintext message, wraps it so her signature will also be encrypted, then encrypts it.

```bash
👉
SIGNED_ENCRYPTED=`envelope subject type string $PLAINTEXT_HELLO | envelope sign -s $ALICE_PRVKEY_BASE | envelope subject type wrapped | envelope encrypt --key $KEY`
echo $SIGNED_ENCRYPTED
```

```
👈
ur:envelope/tansfwlrhdhgjszmsfgdynpmhdgtbwkemyoerflnfxmdesgridjzatlpzebnlahhjoldetecmtpabklkotckvacnvehgfwntvagttkwzgonemklnasiotblkwlgliaoxcskezetprhwyttcfnncfdsdtwzchfdvahgseutvwgagtsbpyvtwfjtqddigsckrnnyclwzpmbntprdntonmdgdsslfdnoefgkspfpdbyfpathyhncndawthddatansfphdcxpfzeltkkrpwsesbnnylrzceymdatdkpeiegowliossnyoxsrveieltftytnslgbyknbeuorl
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope, and examines its contents.

```bash
👉
envelope format $SIGNED_ENCRYPTED
```

```
👈
ENCRYPTED
```

Bob decrypts it using the shared key, and then examines the decrypted envelope's contents.

```bash
👉
DECRYPTED=`envelope decrypt $SIGNED_ENCRYPTED --key $KEY`
envelope format $DECRYPTED
```

```
👈
{
    "Hello." [
        'signed': Signature
    ]
}
```

Bob unwraps the inner envelope, verifies Alice's signature, and then extracts the message.

```bash
👉
envelope extract wrapped $DECRYPTED | envelope verify --pubkeys $ALICE_PUBKEYS | envelope extract string
```

```
👈
Hello.
```

Attempting to verify the wrong key exits with an error.

```bash
👉
envelope extract wrapped $DECRYPTED | envelope verify --pubkeys $CAROL_PUBKEYS
```

```
👈
Error: could not verify a signature
```

## Example 8: Encrypt-Then-Sign

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
ur:crypto-key/hdcxvtbafhhncnmywyckttntsawypyayhlgalypluodlcmnysalbaxaajefsromestdpdnplwyns
```

Alice encrypts a plaintext message, then signs it.

```bash
👉
ENCRYPTED_SIGNED=`envelope subject type string $PLAINTEXT_HELLO | envelope encrypt --key $KEY | envelope sign -s $ALICE_PRVKEY_BASE`
echo $ENCRYPTED_SIGNED
```

```
👈
ur:envelope/lftansfwlrgrinhkwkbbdevduevosatkwlgsmyzoiadwotdmsonsplolptgmgdpesrrhvwvoosfxlpztsaisndtpbburcwhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyaxtpsotansghhdfzjyqzbkfgiepyrfhlkbzclphdhgtnbbehadtoetrkgwdtcawsynrohkoscpssylcyylkifltakoztflptcyrswlsafeynrdmolurdftfnbnsfmsuoswwschsakghlrsylhysaaxad
```

Alice ➡️ ☁️ ➡️ Bob

Bob receives the envelope and examines its contents.

```bash
👉
envelope format $ENCRYPTED_SIGNED
```

```
👈
ENCRYPTED [
    'signed': Signature
]
```

Bob verifies Alice's signature, decrypts the message, then extracts the message.

```bash
👉
envelope verify $ENCRYPTED_SIGNED --pubkeys $ALICE_PUBKEYS | \
    envelope decrypt --key $KEY | \
    envelope extract string
```

```
👈
Hello.
```

## Example 9: Multi-Recipient Encryption

This example demonstrates an encrypted message sent to multiple parties.

Alice encrypts a message so that it can only be decrypted by Bob or Carol.

```bash
👉
ENVELOPE_TO=`envelope subject type string $PLAINTEXT_HELLO | envelope encrypt --recipient $BOB_PUBKEYS --recipient $CAROL_PUBKEYS`
echo $ENVELOPE_TO
```

```
👈
ur:envelope/lstansfwlrgrgysgbseowfndfxwklfghjzgsfeahftztbggrjkssdewlmkcmgddmeszcnstepyctfzlacmpkpyimotrkrdhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyahtpsotansgulftansfwlshddapypswkgedrfmmyketsnyjpnynnaasodmdtnylpyaemsbneaxseethhgersaopstniorshgsaolgsyltndscylkasfesamowyjzesgdbwaaztdizogundzcsfwemnskntdihngltansgrhdcxdlgegoneftkpvlfrpsiaprlnkgfechzclspdkbihwswtcpuodrtbfgwmehttswdroyahtpsotansgulftansfwlshddaprpebzghlnpmgtcyctkbfdframolfxdefrwlrsqzplenbdckuesojostctlsdlceasplrppfjtgsdpztcwwyzoctlbstndpsjlaygdrefhnerdfhgotinytblgcxcxdppdlsnttansgrhdcxsnlabgaychstbsrlsavetblbfxwlfsuyrnoxsfbzghhnwzpfwtpmtkfslkyklnieldcfgywf
```

Alice ➡️ ☁️ ➡️ Bob

Alice ➡️ ☁️ ➡️ Carol

Bob receives the envelope and examines its structure:

```bash
👉
envelope format $ENVELOPE_TO
```

```
👈
ENCRYPTED [
    'hasRecipient': SealedMessage
    'hasRecipient': SealedMessage
]
```

Bob decrypts and reads the message.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $BOB_PRVKEY_BASE | envelope extract string
```

```
👈
Hello.
```

Carol decrypts and reads the message.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $CAROL_PRVKEY_BASE | envelope extract string
```

```
👈
Hello.
```

Alice didn't encrypt it to herself, so she can't read it.

```bash
👉
envelope decrypt $ENVELOPE_TO --recipient $ALICE_PRVKEY_BASE
```

```
👈
Error: unknown recipient
```

## Example 10: Signed Multi-Recipient Encryption

This example demonstrates a signed, then encrypted message, sent to multiple parties.

Alice signs a message, and then encrypts it so that it can only be decrypted by Bob or Carol.

```bash
👉
ENVELOPE_SIGNED_TO=`envelope subject type string $PLAINTEXT_HELLO | envelope sign -s $ALICE_PRVKEY_BASE | envelope encrypt --recipient $BOB_PUBKEYS --recipient $CAROL_PUBKEYS`
echo $ENVELOPE_SIGNED_TO
```

```
👈
ur:envelope/lrtansfwlrgrbbsrpegwhtlyztatlfzeglgsmybsfttiknfpintpnyqztskpgdcevacyqdihasececdwcwimpskttbtnbkhddatansfphdcxlksojzuyktbykovsecbygebsldeninbdfptkwebtwzdpadglwetbgltnwdmwhlhkoyahtpsotansgulftansfwlshddaeclypmidwzjtatdrgunlyaghpkplmntnoetldrvlaepeemgdtkynaxwdaecmnsiyjspsmknywngswztktldnvtnlbeskmnonzorkgddsfzrflyiedybgcpsaghisjpchytdnvetansgrhdcxfwytuegmrozeyaeoasuotofrttndkktdoefygozeeeoxhncywlghbssklnemckfloyaxtpsotansghhdfzgannghwpfzwskbzehekidwihgrvafhjkynbzlftbmktbltveutwkaagtsorszsctwllezogodrtlzeisdrdndrglmsdisaosspcpihtdbawktabbfdmhnsesecemwdcmoyahtpsotansgulftansfwlshddahlfenbtsltgsamidbsldhhutlptpplemltpaeszeehctpttkfptlfprljnfrbbenamgegwcnaegsoykerhvygezcchjeetyapdssgdzobdwtmkfwlfrnzmztknoejtwmrnbgketansgrhdcxhliegdcxosbgnywsinjkaadtwttnjyylsrlsluwygtcnmtgyonwnronepklsbaaaiarklyeh
```

Alice ➡️ ☁️ ➡️ Bob

Alice ➡️ ☁️ ➡️ Carol

Bob receives the envelope and examines its structure:

```bash
👉
envelope format $ENVELOPE_SIGNED_TO
```

```
👈
ENCRYPTED [
    'hasRecipient': SealedMessage
    'hasRecipient': SealedMessage
    'signed': Signature
]
```

Bob verifies Alice's signature, then decrypts and reads the message

```bash
👉
envelope verify $ENVELOPE_SIGNED_TO --pubkeys $ALICE_PUBKEYS | envelope decrypt --recipient $BOB_PRVKEY_BASE | envelope extract string
```

```
👈
Hello.
```
