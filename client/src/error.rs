use common::error::{ConnectionError, StreamError};
use thiserror::Error;

/// Represents all the errors that can occur in the Client.
///
/// Variants:
/// * `SetupError`: An error occurred during the setup process.
/// * `ClientStreamError`: An error occurred during streaming.
/// * `ConnectionError`: An error occurred during connection setup or maintenance.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ClientError {
    #[error(transparent)]
    SetupError(#[from] ClientSetupError),

    #[error(transparent)]
    ClientStreamError(#[from] StreamError),

    #[error("Client connection error: {0}")]
    ConnectionError(#[from] ConnectionError),
}

/// Represents the errors that can occur during client setup.
///
/// Variants:
/// * `EndpointCreationError`: An error occurred while creating the WebTransport client endpoint.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ClientSetupError {
    /// Error occurred while creating the WebTransport client endpoint.
    #[error("failed to create WebTransport client endpoint")]
    EndpointCreationError,
}

impl From<wtransport::error::ConnectionError> for ClientError {
    fn from(error: wtransport::error::ConnectionError) -> Self {
        ClientError::ConnectionError(ConnectionError::from(error))
    }
}
