use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

/// Core error type for rcfe-core crate
/// # Variants
/// * `InvalidUri` - Indicates that the provided URI is invalid
/// * `TonicStatus` - Wraps errors from tonic status
/// * `IllegalArgument` - Indicates that an illegal argument was provided
#[derive(Error, Debug)]
pub enum Error {
    /// URI is invalid
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] InvalidUri),

    /// Tonic status error
    #[error("Tonic status error: {0}")]
    TonicStatus(#[from] tonic::Status),

    /// Illegal argument error
    #[error("Illegal argument: {0}")]
    IllegalArgument(String),

    /// Bytes sequence parsing error
    #[error("Byte sequence parsing error")]
    ByteSequenceParseError,

    /// InvalidTxnSequence
    /// Indicates that the transaction sequence is invalid
    /// # Arguments
    /// * `String` - Description of the error
    #[error("Invalid transaction sequence: {0}")]
    InvalidTxnSequence(String),

    /// Lease KeepAlive error
    /// Indicates an error occurred during lease keep-alive
    /// # Arguments
    /// * `String` - Description of the error
    #[error("Lease keep-alive error: {0}")]
    KeepAliveError(String),

    /// Watch error
    /// Indicates an error occurred during watch operation
    /// # Arguments
    /// * `String` - Description of the error
    #[error("Watch error: {0}")]
    WatchError(String),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}
