# Priority Task: XID Document Public Distribution Workflow

## Problem Statement

There is no workflow in `bc-envelope-cli` to convert a XID document with private keys and provenance mark generator into a publicly distributable version while preserving signatures.

To create a publicly distributable XID document, users need to:
1. **Elide or omit private keys** - Replace private key assertions with ELIDED placeholders (preserving digest tree) or remove them entirely
2. **Elide the provenance mark generator** - The generator allows creating new provenance marks and should be elided (not omitted) to preserve the digest tree
3. **Preserve the signature** - If signed, the signature should remain valid after elision
4. **OR re-sign** - If modifications break the signature, the document needs re-signing

## Current Gaps

### 1. Commands Missing `--private` and `--generator` Output Options

These commands hard-code `PrivateOptions::Include` and don't support controlling output:

| Command                 | Missing Options            |
| ----------------------- | -------------------------- |
| `xid method add`        | `--private`, `--generator` |
| `xid method remove`     | `--private`, `--generator` |
| `xid delegate add`      | `--private`, `--generator` |
| `xid delegate update`   | `--private`, `--generator` |
| `xid delegate remove`   | `--private`, `--generator` |
| `xid service add`       | `--private`, `--generator` |
| `xid service update`    | `--private`, `--generator` |
| `xid service remove`    | `--private`, `--generator` |
| `xid attachment add`    | `--private`, `--generator` |
| `xid attachment remove` | `--private`, `--generator` |
| `xid provenance next`   | `--private`, `--generator` |

### 2. Key Commands Missing `--generator` Option

These commands have `--private` but not `--generator`:

| Command          | Missing Options |
| ---------------- | --------------- |
| `xid key add`    | `--generator`   |
| `xid key update` | `--generator`   |
| `xid key remove` | `--generator`   |

### 3. `--generator elide` Rejected at Runtime

The `GeneratorOptions::Elide` variant exists and is listed in help text, but is explicitly rejected at runtime in `xid new` with:
```
"Elide is not allowed for 'xid new'. Use 'omit' (the default)..."
```

This is correct for `xid new` (can't elide what doesn't exist), but `elide` should be available for commands that operate on existing documents.

### 4. No Standalone Export Command

No command exists to simply re-output an existing XID document with different private/generator options without modifying it.

---

## Implementation Plan

### Phase 1: Create Shared Output Options Infrastructure (DRY Refactor)

**Goal:** Create reusable argument structs that can be composed into commands needing output control.

#### 1.1 Create `OutputOptions` struct in `src/cmd/xid/mod.rs` or new file `output_options.rs`

```rust
/// Options controlling how sensitive data is output in XID documents.
#[derive(Debug, Args, Default)]
pub struct OutputOptions {
    #[command(flatten)]
    pub private_opts: PrivateOutputArgs,

    #[command(flatten)]
    pub generator_opts: GeneratorOutputArgs,
}

#[derive(Debug, Args, Default)]
pub struct PrivateOutputArgs {
    /// Whether to include, omit, elide, or encrypt private keys.
    #[arg(long = "private", default_value = "include")]
    pub private: PrivateOptions,
}

#[derive(Debug, Args, Default)]
pub struct GeneratorOutputArgs {
    /// Whether to include, omit, elide, or encrypt the provenance mark generator.
    #[arg(long = "generator", default_value = "include")]
    pub generator: GeneratorOptions,
}
```

#### 1.2 Update `xid_document_to_ur_string` signature

Update to accept `OutputOptions` instead of individual `PrivateOptions` and `Option<GeneratorOptions>`:

```rust
pub fn xid_document_to_ur_string(
    xid_document: &XIDDocument,
    output_opts: &OutputOptions,
    password_args: Option<&WritePasswordArgs>,
    shared_password: Option<String>,
    signing_options: XIDSigningOptions,
) -> Result<String>
```

#### 1.3 Create trait for commands with output options

```rust
pub trait HasOutputOptions {
    fn output_options(&self) -> &OutputOptions;
}
```

---

### Phase 2: Add Output Options to Key Commands

**Goal:** Add `--generator` option to existing key commands.

#### 2.1 Update `xid key add`
- Add `GeneratorOutputArgs` to `CommandArgs`
- Pass generator option to `xid_document_to_ur_string`

#### 2.2 Update `xid key update`
- Add `GeneratorOutputArgs` to `CommandArgs`
- Pass generator option to `xid_document_to_ur_string`

#### 2.3 Update `xid key remove`
- Add `GeneratorOutputArgs` to `CommandArgs`
- Pass generator option to `xid_document_to_ur_string`

---

### Phase 3: Add Output Options to Non-Key Commands

**Goal:** Add both `--private` and `--generator` options to all XID-modifying commands.

#### 3.1 Method commands
- `xid method add` - Add `OutputOptions`
- `xid method remove` - Add `OutputOptions`

#### 3.2 Delegate commands
- `xid delegate add` - Add `OutputOptions`
- `xid delegate update` - Add `OutputOptions`
- `xid delegate remove` - Add `OutputOptions`

#### 3.3 Service commands
- `xid service add` - Add `OutputOptions`
- `xid service update` - Add `OutputOptions`
- `xid service remove` - Add `OutputOptions`

#### 3.4 Attachment commands
- `xid attachment add` - Add `OutputOptions`
- `xid attachment remove` - Add `OutputOptions`

#### 3.5 Provenance commands
- `xid provenance next` - Add `OutputOptions`

---

### Phase 4: Add `xid export` Subcommand

**Goal:** Create a command to re-output an existing XID document with different output options without modifying it.

#### 4.1 Create `src/cmd/xid/export.rs`

```rust
/// Export a XID document with specified output options.
///
/// This command reads an existing XID document and outputs it with
/// the specified handling of private keys and provenance generator.
/// Use this to create publicly distributable versions of XID documents.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    output_opts: OutputOptions,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    signing_args: SigningArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}
```

#### 4.2 Register in `src/cmd/xid/mod.rs`

Add `Export(export::CommandArgs)` to `SubCommands` enum.

#### 4.3 Example usage in documentation

```bash
# Create publicly distributable version (elide secrets, preserve signature)
envelope xid export --private elide --generator elide $FULL_XID

# Create minimal version (omit secrets, re-sign required)
envelope xid export --private omit --generator omit --sign inception $FULL_XID

# Encrypt secrets for storage, re-sign
envelope xid export --private encrypt --generator encrypt \
    --encrypt-password "secret" --sign inception $FULL_XID
```

---

### Phase 5: Update Tests

#### 5.1 Add tests for new functionality in `tests/test_xid.rs`

- Test `--private elide` preserves signature
- Test `--generator elide` preserves signature
- Test `--private omit` requires re-signing
- Test `--generator omit` requires re-signing
- Test all combinations work correctly

#### 5.2 Add tests for `xid export` in `tests/test_xid_export.rs`

- Test export with elision preserves signature
- Test export with omission invalidates signature
- Test export with encryption
- Test round-trip: create → export public → verify signature

---

### Phase 6: Update Documentation

#### 6.1 Update `docs/XID.md`

- Add section on creating publicly distributable XID documents
- Document `xid export` command
- Add workflow examples for common use cases

#### 6.2 Add inline help text

Ensure all new options have clear `/// ` doc comments that appear in `--help`.

---

## Implementation Order

1. **Phase 1** - DRY infrastructure (blocks all other phases)
2. **Phase 2** - Key commands (small scope, validates Phase 1 design)
3. **Phase 3** - Non-key commands (apply pattern from Phase 2)
4. **Phase 4** - Export command (uses all infrastructure from prior phases)
5. **Phase 5** - Tests (validates all functionality)
6. **Phase 6** - Documentation (documents completed features)

## Success Criteria

- [ ] All XID-modifying commands support `--private` and `--generator` options
- [ ] `xid export` command exists and works correctly
- [ ] Elision preserves existing signatures
- [ ] Omission correctly requires re-signing
- [ ] No code duplication for output option handling
- [ ] Tests cover all new functionality
- [ ] Documentation is complete and accurate
