# `envelope`

## A command line tool for manipulating the Gordian Envelope data type, written in pure Rust.

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally & Christopher Allen_

**NOTE:** Preview version. Not ready for production use.

<img src="images/envelope-rust-screen.jpg" width=960>

## Installation

To install from crates.io, run:

```bash
cargo install bc-envelope-cli
```

To install from source, clone this repo, change to its root directory and run:

```bash
cargo install --path .
```

Make sure your `~/.cargo/bin` directory is in your `PATH`.

## Usage

```bash
envelope --help
```

See the [docs](docs/README.md) directory for more information.

**NOTE:** The `envelope` tool does *not* have the same command line syntax as the Swift `envelope` tool.

## Version History

### 0.12.0: February 4, 2025

- Maintenance release.

### 0.11.0: December 24, 2024

- Added support for [XID Document Services](docs/XID.md).
- Endpoints in XID keys are now deprecated in favor of services.

### 0.10.0: December 15 2024

- Started adding support for [XID Documents](docs/XID.md).
- Anywhere in `envelope` that accepts a `ur:envelope` can also accept any other UR types containing envelopes, including XID documents.

### 0.9.3: December 1, 2024

- Aligned to dependencies.

### 0.9.2: October 16, 2024

- Schnorr signatures are now BIP-340 compliant.

### 0.9.1: July 12, 2024

- Fixed bug that would cause certain valid combinations of SSKR shares to be rejected.
