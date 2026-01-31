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
* [XID Documents](XID.md) — Overview of XID support.
    * [XID Basics](XID-Basics.md) — Creating and identifying XID documents.
    * [XID Export](XID-Export.md) — Exporting with controlled private key handling.
    * [XID Keys](XID-Keys.md) — Working with XID document keys.
    * [XID Methods](XID-Methods.md) — Resolution methods.
    * [XID Delegates](XID-Delegates.md) — Working with delegates.
    * [XID Services](XID-Services.md) — Working with services.
    * [XID Provenance](XID-Provenance.md) — Provenance mark chains.
    * [XID Signing](XID-Signing.md) — Signing and verifying.
    * [XID Attachments](XID-Attachments.md) — Third-party metadata.
    * [XID Edges](XID-Edges.md) — Verifiable claims (edges).
* [Pattern Matching](PatternMatching.md) - Using patterns to match on parts of envelopes
    * [Envelope Pattern Expression Syntax](envelope_patex.md) - The syntax for envelope patterns used in pattern matching.

For more examples of `envelope-cli` usage, see the [envelope-cli videos](https://github.com/BlockchainCommons/envelope-cli-swift#videos) and their transcripts.
