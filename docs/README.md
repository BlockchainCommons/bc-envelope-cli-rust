# `envelope` Docs

The following docs exemplify the basic functionality of the `envelope` tool. The built-in `--help` is a good source of information. You may also find the unit tests in the [tests](../tests/) directory useful.

* [Overview of Commands](Overview.md) — Adding subjects, assertions, signatures, and salt.
* [Basic Examples](BasicExamples.md) — Demonstrating standard methodologies for entry, encryption, and signing.
* [SSKR Example](SSKRExample.md) — Using Shamir's Secret Sharing to lock envelopes.
* [Complex Metadata Example](MetadataExample.md) — Creating envelopes with layered, structured data.
* [DID Document Example](DIDExample.md) — Creating and wrapping identifiers.
* [Verifiable Credential Example](VCResidentExample.md) — Building complex credentials around an identifier.
* [Verifiable Credential with Detailed Elision Example](VCElisionExample.md) — Eliding some of a credential.
* [Attachments](Attachments.md) — Standardized third-party attachments for envelopes.
* [Signing Envelopes](Signing.md) — Signing and verifying signatures on envelopes.
* [XID Documents](XID.md) — Working with XIDs and XID documents.
* [XID Edges](XID-Edges.md) — Working with verifiable claims (edges) on XID documents.
* [Pattern Matching](PatternMatching.md) - Using patterns to match on parts of envelopes
    * [Envelope Pattern Expression Syntax](envelope_patex.md) - The syntax for envelope patterns used in pattern matching.

For more examples of `envelope-cli` usage, see the [envelope-cli videos](https://github.com/BlockchainCommons/envelope-cli-swift#videos) and their transcripts.
