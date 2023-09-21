use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    /// ARID: Apparently Random Identifier (ur:arid)
    Arid,
    /// CBOR data in hex
    Cbor,
    /// Binary byte string in hex
    Data,
    /// Date
    Date,
    /// Cryptographic digest (ur:digest)
    Digest,
    /// Envelope (ur:envelope)
    Envelope,
    /// Numeric value,
    Number,
    /// Known Value (number or string)
    Known,
    /// UTF-8 String
    String,
    /// Uniform Resource (UR)
    Ur,
    /// URI
    Uri,
    /// UUID
    Uuid,
    /// Wrapped Envelope (ur:envelope)
    Wrapped,
}
