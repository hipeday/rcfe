use thiserror::Error;
use tonic::codegen::http::uri::InvalidUri;

#[derive(Error, Debug)]
pub enum Error {
    /// URI is invalid
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] InvalidUri),
    
    /// Tonic status error
    #[error("Tonic status error: {0}")]
    TonicStatus(#[from] tonic::Status),
}