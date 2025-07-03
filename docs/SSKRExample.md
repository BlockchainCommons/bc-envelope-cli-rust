# `envelope` - SSKR Example

This example demonstrates the use of SSKR to shard a symmetric key that encrypted a message. The shares are then enclosed in individual envelopes and the seed can be recovered from those shares, allowing the future decryption of the message.

Dan has a cryptographic seed he wants to backup using a social recovery scheme. The seed includes metadata he wants to back up also, making it too large to fit into a basic SSKR share.

```
DAN_SEED=ur:seed/oxadgdhkwzdtfthptokigtvwnnjsqzcxknsktdaosezofptpbtlnlyjzkefmaxkpfyhsjpjecxgdkpjpjojzihcxfpjskphscxgsjlkoihaakskggsjljpihjncxinjojkkpjncxiejljzjljpcxjkinjycxhsjnihjydwcxiajljtjkihiajyihjykpjpcxhsieinjoinjkiainjtiocxihjzinjydwcxjkihiecxiejlcxihinkpjkjnjliecxjyihjnjojljpcxinjtiainieiniekpjtjycxkpjycxjzhsidjljpihcxihjycxiejljzjljpihcxjnhsiojthscxhsjzinjskphsdmluwmoxny
```

Dan encloses his seed in an envelope.

```
DAN_ENVELOPE=`envelope subject type ur $DAN_SEED`
echo $DAN_ENVELOPE

│ ur:envelope/tpsotantjzoxadgdhkwzdtfthptokigtvwnnjsqzcxknsktdaosezofptpbtlnlyjzkefmaxkpfyhsjpjecxgdkpjpjojzihcxfpjskphscxgsjlkoihaakskggsjljpihjncxinjojkkpjncxiejljzjljpcxjkinjycxhsjnihjydwcxiajljtjkihiajyihjykpjpcxhsieinjoinjkiainjtiocxihjzinjydwcxjkihiecxiejlcxihinkpjkjnjliecxjyihjnjojljpcxinjtiainieiniekpjtjycxkpjycxjzhsidjljpihcxihjycxiejljzjljpihcxjnhsiojthscxhsjzinjskphsdmfmflcnfy
```

Dan examines the contents of his envelope.

```
envelope format $DAN_ENVELOPE

│ Seed
```

Dan generates a public/private key pair that will allow him to recover his seed from any single share if he retains his private key.

```
DAN_PRIVATE_KEY=`envelope generate prvkeys`
DAN_PUBLIC_KEY=`envelope generate pubkeys $DAN_PRIVATE_KEY`
```

Dan splits the envelope into a single group 2-of-3. The output of the `envelope` tool contains the list of share envelopes separated by spaces. He then assigns this to a shell array.

```
SHARE_ENVELOPES=(`envelope sskr split -g 2-of-3 --recipient $DAN_PUBLIC_KEY $DAN_ENVELOPE`)
```

Dan sends one envelope to each of Alice, Bob, and Carol.

```
SHARE_ENVELOPE_ALICE=${SHARE_ENVELOPES[1]}
SHARE_ENVELOPE_BOB=${SHARE_ENVELOPES[2]}
SHARE_ENVELOPE_CAROL=${SHARE_ENVELOPES[3]}
```

Dan ➡️ ☁️ ➡️ Alice, Bob, Carol

Bob examines the contents of his envelope, but can't recover the original seed: the SSKR share can only recover the seed with a threshold of other shares, and the SealedMessage can only be decrypted by Dan's private key.

```
envelope format $SHARE_ENVELOPE_BOB

│ ENCRYPTED [
│     'hasRecipient': SealedMessage
│     'sskrShare': SSKRShare
│ ]
```

By himself, Bob can't recover the seed.

```
envelope sskr join $SHARE_ENVELOPE_BOB

│ Error: invalid SSKR shares
```

At some future point, Dan retrieves two of the three envelopes so he can recover his seed.

```
envelope sskr join $SHARE_ENVELOPE_BOB $SHARE_ENVELOPE_CAROL | envelope extract ur

│ ur:seed/oxadgdhkwzdtfthptokigtvwnnjsqzcxknsktdaosezofptpbtlnlyjzkefmaxkpfyhsjpjecxgdkpjpjojzihcxfpjskphscxgsjlkoihaakskggsjljpihjncxinjojkkpjncxiejljzjljpcxjkinjycxhsjnihjydwcxiajljtjkihiajyihjykpjpcxhsieinjoinjkiainjtiocxihjzinjydwcxjkihiecxiejlcxihinkpjkjnjliecxjyihjnjojljpcxinjtiainieiniekpjtjycxkpjycxjzhsidjljpihcxihjycxiejljzjljpihcxjnhsiojthscxhsjzinjskphsdmluwmoxny
```

Dan can also recover his seed from a single share by decrypting it with his private key.

```
envelope decrypt --recipient $DAN_PRIVATE_KEY $SHARE_ENVELOPE_BOB | envelope extract wrapped | envelope extract ur

│ ur:seed/oxadgdhkwzdtfthptokigtvwnnjsqzcxknsktdaosezofptpbtlnlyjzkefmaxkpfyhsjpjecxgdkpjpjojzihcxfpjskphscxgsjlkoihaakskggsjljpihjncxinjojkkpjncxiejljzjljpcxjkinjycxhsjnihjydwcxiajljtjkihiajyihjykpjpcxhsieinjoinjkiainjtiocxihjzinjydwcxjkihiecxiejlcxihinkpjkjnjliecxjyihjnjojljpcxinjtiainieiniekpjtjycxkpjycxjzhsidjljpihcxihjycxiejljzjljpihcxjnhsiojthscxhsjzinjskphsdmluwmoxny
```
