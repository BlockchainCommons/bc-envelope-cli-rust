# XID Provenance Sequence Example

This tutorial shows the exact `envelope` command sequence to create 20 signed `ur:xid/...` documents with encrypted private keys, encrypted provenance generators, and a sequence-number attachment for each revision (`0` through `19`). It then validates the chain using `provenance`.

## Prerequisites

- `envelope` must be installed and available on `$PATH`.
- `provenance` must be installed and available on `$PATH`.
- Run the commands from this directory: `bc-envelope-cli/examples/`.

## 1. Set Working Variables

```
export PASSWORD='xid-seq-pass'
export VENDOR='com.blockchaincommons.examples.xid-seq'
export CONFORMS_TO='https://developer.blockchaincommons.com/xid/provenance-seq/v1'

│ (no output)
```

## 2. Create the Inception XID Document (Sequence 0)

```
KEYPAIR=$(envelope generate keypairs)
PRVKEYS=$(echo "$KEYPAIR" | awk '{print $1}')
XID=$(envelope xid new "$PRVKEYS" \
  --private encrypt \
  --generator encrypt \
  --encrypt-password="$PASSWORD" \
  --sign inception)

PAYLOAD_0=$(envelope subject type number 0)
XID=$(envelope xid attachment add \
  --vendor "$VENDOR" \
  --conforms-to "$CONFORMS_TO" \
  --payload "$PAYLOAD_0" \
  --private encrypt \
  --generator encrypt \
  --password="$PASSWORD" \
  --encrypt-password="$PASSWORD" \
  --verify inception \
  --sign inception \
  "$XID")

printf '%s\n' "$XID" > xid-sequence.txt

│ (no output)
```

## 3. Advance Provenance 19 More Times (Sequence 1..19)

```
for SEQ in $(seq 1 19); do
  XID=$(envelope xid provenance next \
    --private encrypt \
    --generator encrypt \
    --password="$PASSWORD" \
    --encrypt-password="$PASSWORD" \
    --verify inception \
    --sign inception \
    "$XID")

  ATTACHMENTS=$(envelope xid attachment find --vendor "$VENDOR" "$XID")
  if [ -n "$ATTACHMENTS" ]; then
    while IFS= read -r ATTACHMENT; do
      [ -z "$ATTACHMENT" ] && continue
      XID=$(envelope xid attachment remove \
        "$ATTACHMENT" \
        --private encrypt \
        --generator encrypt \
        --password="$PASSWORD" \
        --encrypt-password="$PASSWORD" \
        --verify inception \
        --sign inception \
        "$XID")
    done <<< "$ATTACHMENTS"
  fi

  PAYLOAD=$(envelope subject type number "$SEQ")
  XID=$(envelope xid attachment add \
    --vendor "$VENDOR" \
    --conforms-to "$CONFORMS_TO" \
    --payload "$PAYLOAD" \
    --private encrypt \
    --generator encrypt \
    --password="$PASSWORD" \
    --encrypt-password="$PASSWORD" \
    --verify inception \
    --sign inception \
    "$XID")

  printf '%s\n' "$XID" >> xid-sequence.txt
done

│ (no output)
```

## 4. Confirm You Have 20 XID URs

```
wc -l xid-sequence.txt
head -n 3 xid-sequence.txt

│       20 xid-sequence.txt
│ ur:xid/...
│ ur:xid/...
│ ur:xid/...
```

## 5. Validate the Chain with `provenance`

`provenance validate` is silent on success.

```
xargs provenance validate < xid-sequence.txt
echo $?

│ 0
```

## 6. Optional: Validate the Extracted `ur:provenance/...` Marks

```
while IFS= read -r DOC; do
  envelope xid provenance get "$DOC"
done < xid-sequence.txt | xargs provenance validate
echo $?

│ 0
```

## 7. Optional: Verify Attachment Sequence Numbers

```
i=0
while IFS= read -r DOC; do
  ATTACHMENT=$(envelope xid attachment find --vendor "$VENDOR" "$DOC")
  PAYLOAD=$(envelope attachment payload "$ATTACHMENT")
  VALUE=$(envelope extract number "$PAYLOAD")
  if [ "$VALUE" != "$i" ]; then
    echo "mismatch at $i got $VALUE"
    exit 1
  fi
  i=$((i + 1))
done < xid-sequence.txt
echo "attachment-seq-ok"

│ attachment-seq-ok
```

## Helper Script in This Directory

If you want the same workflow wrapped in one command, this directory also includes `generate_xid_provenance_sequence.py`.

```
./generate_xid_provenance_sequence.py --count 20 > xid-sequence-script.txt
xargs provenance validate < xid-sequence-script.txt
echo $?

│ 0
```
