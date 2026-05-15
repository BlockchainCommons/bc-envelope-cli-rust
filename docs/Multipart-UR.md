# Multipart UR Examples

`envelope` emits UR strings. Multipart QR output is handled by the companion
`mur` tool, which splits a UR into fountain-coded QR frames or an animated QR
sequence.

## Create a UR with `envelope`

Any envelope UR can be passed to `mur`. This example creates a small salted
envelope:

```sh
ENVELOPE=$(envelope subject type string "paper backup example" | envelope salt)
```

For a larger backup, use the command that creates the document you want to put
on paper, then pass that UR to `mur`.

## Numbered QR Frames

Use `mur frames` to write one PNG per multipart frame:

```sh
mur frames --max-fragment-len 80 --output backup-frames "$ENVELOPE"
```

`--max-fragment-len` controls the maximum fragment length used for fountain
coding. Smaller fragments create more QR frames, but each frame is easier for a
scanner to read. Larger fragments create fewer frames, but each QR code is
denser.

Useful frame options include:

```sh
mur frames \
    --max-fragment-len 80 \
    --size 768 \
    --correction high \
    --cycles 4 \
    --output backup-frames \
    "$ENVELOPE"
```

The output directory will contain numbered PNG files suitable for printing one
frame per page.

## Animated QR

Use `mur animate` to produce an animated QR sequence:

```sh
mur animate --max-fragment-len 80 --cycles 4 --output backup.gif "$ENVELOPE"
```

The default animation format is GIF. `--fps` controls playback speed, and
`--cycles` controls how many complete passes through the frame set are emitted.

## Read from Standard Input

Both `mur frames` and `mur animate` accept `-` to read the UR from standard
input:

```sh
envelope subject type string "paper backup example" |
    envelope salt |
    mur frames --max-fragment-len 80 --output backup-frames -
```

This keeps the generated UR out of the shell history.
