/// VRT Errors
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// Buffer is full
    #[error("Buffer is full")]
    BufferFull,
    /// Invalid TSI type.
    #[error("Invalid TSI Type: {0}")]
    Tsi(u8),
    /// Invalid TSF type.
    #[error("Invalid TSF Type: {0}")]
    Tsf(u8),
    /// Invalid packet type.
    #[error("Invalid Packet Type: {0}")]
    PktType(u8),
    /// Conversion error
    #[error("Conversion error: {0}")]
    TryFromIntError(#[from] core::num::TryFromIntError),
}
