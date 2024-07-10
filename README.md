# `envelope`

## A command line tool for manipulating the Gordian Envelope data type, written in pure Rust.

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally_

**NOTE:** Preview version. Not ready for production use.

<img src="images/envelope-rust-screen.jpg" width=960>

### Installation

To install from crates.io, run:

```bash
cargo install bc-envelope-cli
```

To install from source, clone this repo, change to its root directory and run:

```bash
cargo install --path .
```

Make sure your `~/.cargo/bin` directory is in your `PATH`.

### Usage

```bash
envelope --help
```

See the [docs](docs/README.md) directory for more information.

**NOTE:** The `envelope` tool does *not* have the same command line syntax as the Swift `envelope` tool.
