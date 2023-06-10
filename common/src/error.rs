use thiserror::Error;

use crate::message::Message;

/// Enumerates potential errors that can occur during stream operations in the client.
///
/// `StreamError` includes various types of stream related
/// errors. Its variants include:
///
/// - `ReadError`: Errors occurred while reading from a stream.
/// - `WriteError`: Errors occurred while writing to a stream.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum StreamError {
    #[error(transparent)]
    ReadError(#[from] ReadStreamError),

    #[error(transparent)]
    WriteError(#[from] WriteStreamError),
}

/// Enumerates potential errors that can occur during the reading process from a stream.
///
/// `ReadStreamError` includes various types of errors that could be encountered while reading
/// data from a stream. The variants include:
///
/// - `ConnectionClosed`: This error indicates that the connection was closed before
///   enough bytes could be read.
/// - `StreamStopped`: This error indicates that the stream was stopped before enough bytes
///   could be read.
/// - `DataDeserializationFailed`: Errors occurred during deserialization of data from the stream.
/// - `DatagramError`: Errors specific to Datagram operations during its read from the stream.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ReadStreamError {
    #[error("connection closed before reading enough bytes")]
    ConnectionClosed,
    #[error("stream have stopped before reading enough bytes")]
    StreamStopped,
    #[error("failed to deserialize underlying data: {0}")]
    DataDeserializationFailed(#[from] SerializationError),
    #[error(transparent)]
    DatagramError(#[from] DatagramError),
}

/// Enumerates potential errors that can occur during the writing to a stream.
///
/// `WriteStreamError` includes various types of errors that could be encountered while writing
/// data to a stream. The variants include:
///
/// - `ConnectionClosed`: This error indicates that the connection was closed before
///   enough bytes could be written.
/// - `StreamStopped`: This error indicates that the stream was stopped before enough bytes
///   could be written.
/// - `DataSerializationFailed`: Errors occurred during serialization of data before writing to the stream.
/// - `DatagramError`: Errors specific to Datagram operations during writing to the stream.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum WriteStreamError {
    #[error("connection closed before writing enough bytes")]
    ConnectionClosed,
    #[error("stream have stopped before writing enough bytes")]
    StreamStopped,
    #[error("failed to serialize underlying data: {0}")]
    DataSerializationFailed(#[from] SerializationError),
    #[error(transparent)]
    DatagramError(#[from] DatagramError),
}

/// Enumerates potential errors that can occur during connection operations.
///
/// `ConnectionError` includes various types of errors that could be encountered while
/// establishing or maintaining a connection. The variants include:
///
/// - `ClosedByPeer`: This error indicates that the connection was closed by the peer. It carries
///   additional data in the form of the peer's code and reason.
/// - `ClosedLocally`: This error indicates that the connection was closed locally.
/// - `TimedOut`: This error indicates that the connection operation exceeded its allocated time.
/// - `HTTP3`: This error is specific to HTTP3 protocol errors. It includes a code and a reason string.
/// - `QuicError`: This error is specific to QUIC protocol errors.
/// - `MaxRetriesReached`: This error indicates that the maximum number of retry attempts has been reached.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ConnectionError {
    #[error("connection closed by peer with code {code:?}: {reason:?}")]
    ClosedByPeer { code: u64, reason: Vec<u8> },

    #[error("connection closed locally")]
    ClosedLocally,

    #[error("connection timed out")]
    TimedOut,

    #[error("HTTP3 error {code:?}: {reason:?}")]
    HTTP3 { code: u64, reason: String },

    #[error("QUIC protocol error")]
    QuicError,

    #[error("maximum retries ({retry_count:?}) reached")]
    MaxRetriesReached { retry_count: u16 },
}

/// Represents the potential errors that can occur while handling datagrams.
///
/// The `DatagramError` enum defines the following variants:
///
/// - `DeserializationFailed`: An error variant which indicates that a failure occurred while attempting to
///   deserialize the datagram.
/// - `ConnectionClosed`: An error variant which signifies that the connection was closed by the peer.
/// - `UnsupportedByPeer`: An error variant which signifies that datagrams are not supported by the peer.
/// - `QuicError`: An error variant which signifies that a QUIC protocol error occurred.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum DatagramError {
    #[error("datagram deserialization failed: {0}")]
    DeserializationFailed(#[from] SerializationError),

    #[error("connection closed by peer")]
    ConnectionClosed,

    #[error("datagrams are not supported by peer")]
    UnsupportedByPeer,

    #[error("QUIC protocol error")]
    QuicError,
}

/// An enumeration of potential errors that can occur during the serialization or deserialization of a `Message`.
///
/// The `SerializationError` enum includes two variants:
///
/// - `SerializationFailed`: This variant is used when the serialization of a `Message` fails. The failed `Message`
///   is included as part of the variant.
/// - `DeserializationFailed`: This variant is used when the deserialization of bytes into a `Message` fails. The
///   original bytes that failed to be deserialized are included as part of the variant.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum SerializationError {
    #[error("serialization failed")]
    SerializationFailed { message: Message },

    #[error("deserialization failed")]
    DeserializationFailed { bytes: Vec<u8> },
}

impl From<wtransport::error::StreamError> for ReadStreamError {
    fn from(error: wtransport::error::StreamError) -> Self {
        match error {
            wtransport::error::StreamError::ConnectionClosed => ReadStreamError::ConnectionClosed,
            wtransport::error::StreamError::Stopped => ReadStreamError::StreamStopped,
        }
    }
}

impl From<wtransport::error::StreamError> for WriteStreamError {
    fn from(error: wtransport::error::StreamError) -> Self {
        match error {
            wtransport::error::StreamError::ConnectionClosed => WriteStreamError::ConnectionClosed,
            wtransport::error::StreamError::Stopped => WriteStreamError::StreamStopped,
        }
    }
}

impl From<wtransport::error::ConnectionError> for ConnectionError {
    fn from(error: wtransport::error::ConnectionError) -> Self {
        match error {
            wtransport::error::ConnectionError::ConnectionClosed(e) => {
                ConnectionError::ClosedByPeer {
                    code: e.code().into_inner(),
                    reason: e.reason().to_vec(),
                }
            }
            wtransport::error::ConnectionError::LocallyClosed => ConnectionError::ClosedLocally,
            wtransport::error::ConnectionError::TimedOut => ConnectionError::TimedOut,
            wtransport::error::ConnectionError::H3(e) => ConnectionError::HTTP3 {
                code: e.code().to_code().into_inner(),
                reason: e.reason().to_string(),
            },
            wtransport::error::ConnectionError::QuicError => ConnectionError::QuicError,
        }
    }
}

impl From<wtransport::error::DatagramError> for DatagramError {
    fn from(error: wtransport::error::DatagramError) -> Self {
        match error {
            wtransport::error::DatagramError::ConnectionClosed => DatagramError::ConnectionClosed,
            wtransport::error::DatagramError::UnsupportedByPeer => DatagramError::UnsupportedByPeer,
            wtransport::error::DatagramError::Protocol => DatagramError::QuicError,
        }
    }
}
