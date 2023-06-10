use common::error::{ConnectionError, StreamError};
use thiserror::Error;

/// Represents all the errors that can occur in the Server.
///
/// Variants:
/// * `SetupError`: An error occurred during the setup process.
/// * `ServerStreamError`: An error occurred during streaming.
/// * `ConnectionError`: An error occurred during connection setup or maintenance.
#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    SetupError(#[from] ServerSetupError),

    #[error(transparent)]
    ServerStreamError(#[from] StreamError),

    #[error("Server connection error: {0}")]
    ConnectionError(#[from] ConnectionError),
}

/// Represents the errors that can occur during server setup.
///
/// Variants:
/// * `EndpointCreationError`: An error occurred while creating the WebTransport client endpoint.
/// * `CertificateSetupError`: An error occurred while setting up the certificate.
#[derive(Error, Debug)]
pub enum ServerSetupError {
    #[error("failed to create WebTransport server endpoint")]
    EndpointCreationError,

    #[error("failed to load certificate. Check certificate path ({cert_path:?}) and key path ({key_path:?})")]
    CertificateSetupError { cert_path: String, key_path: String },
}

impl From<wtransport::error::ConnectionError> for ServerError {
    fn from(error: wtransport::error::ConnectionError) -> Self {
        ServerError::ConnectionError(ConnectionError::from(error))
    }
}
