use common::{
    error::{DatagramError, StreamError},
    message::Message,
    stream::{read_next_message, write_message},
};
use wtransport::Connection;

use crate::error::ServerError;

/// Handles a bidirectional stream.
///
/// This function will read messages from the stream and respond to them with a "Pong!" message.
///
/// # Arguments
///
/// * `connection` - A reference to the connection.
///
/// # Returns
///
/// An empty `Result` indicating success or an error.
pub async fn handle_bidirectional(connection: &Connection) -> Result<(), ServerError> {
    let (mut send_stream, mut recv_stream) = connection.accept_bi().await?;

    loop {
        println!("Reading next message from the stream...");

        let message = read_next_message(&mut recv_stream)
            .await
            .map_err(StreamError::from)?;

        println!("Received request data: {}", message.get_data());

        if let Message::Request(request) = message {
            let response = Message::new_response(&request.id, "Pong!".to_string());

            write_message(&mut send_stream, &response)
                .await
                .map_err(StreamError::from)?;
        }
    }
}

/// Handles a unidirectional stream.
///
/// This function will read messages from the stream and respond to them with a "Pong!" message.
/// Using 2 distinct streams for reading and writing.
///
/// # Arguments
///
/// * `connection` - A reference to the connection.
///
/// # Returns
///
/// An empty `Result` indicating success or an error.
pub async fn handle_unidirectional(connection: &Connection) -> Result<(), ServerError> {
    let mut recv_stream = connection.accept_uni().await?;
    let mut send_stream = connection.open_uni().await?;

    loop {
        println!("Reading next message from the stream...");

        let message = read_next_message(&mut recv_stream)
            .await
            .map_err(StreamError::from)?;

        println!("Received request data: {}", message.get_data());

        if let Message::Request(request) = message {
            let response = Message::new_response(&request.id, "Pong!".to_string());

            write_message(&mut send_stream, &response)
                .await
                .map_err(StreamError::from)?;
        }
    }
}

/// Handles a datagram.
///
/// This function will read datagram message and respond with a "Pong!" datagram message.
///
/// # Arguments
///
/// * `connection` - A reference to the connection.
///
/// # Returns
///
/// An empty `Result` indicating success or an error.
pub async fn handle_datagram(connection: &Connection) -> Result<(), DatagramError> {
    let datagram = connection.receive_datagram().await?;

    let message = Message::from_bytes(&datagram)?;

    println!("Received request data: {}", message.get_data());

    if let Message::Request(request) = message {
        let response = Message::new_response(&request.id, "Pong!".to_string());

        connection.send_datagram(response.as_bytes()?)?;
    }

    Ok(())
}
