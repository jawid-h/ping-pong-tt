use std::time::Duration;

use common::{
    error::{DatagramError, ReadStreamError, StreamError, WriteStreamError},
    message::Message,
    stream::{read_next_message, write_message},
};
use tokio::time::sleep;
use wtransport::Connection;

use crate::error::ClientError;

/// Send messages bidirectionally over a connection.
///
/// # Arguments
///
/// * `connection` - The connection over which the message is sent.
/// * `message` - The message to be sent.
/// * `count_option` - Optional argument to limit the number of times the message is sent. If `None`, the message is sent indefinitely.
///
/// # Returns
///
/// This function returns `Ok(())` if all messages were sent successfully, or an `Err(ClientError)` if an error occurs.
///
/// This function sends the message and waits for a response. This cycle is repeated until the sent message count has reached the optional `count_option` limit.
pub async fn send_bidirectional(
    connection: &Connection,
    message: &Message,
    count_option: Option<u32>,
    inbox: &mut Vec<Message>,
) -> Result<(), ClientError> {
    let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

    let mut sent_count = 0;
    loop {
        write_message(&mut send_stream, message)
            .await
            .map_err(StreamError::from)?;

        let response = read_next_message(&mut recv_stream)
            .await
            .map_err(StreamError::from)?;

        println!("Received response data: {}", response.get_data());

        inbox.push(response);

        sent_count += 1;

        if let Some(count) = count_option {
            if sent_count >= count {
                break;
            }
        }
    }

    Ok(())
}

/// Sends messages unidirectionally over a connection.
///
/// # Arguments
///
/// * `connection` - The connection over which the message is sent.
/// * `message` - The message to be sent.
/// * `count_option` - Optional argument to limit the number of times the message is sent. If `None`, the message is sent indefinitely.
///
/// # Returns
///
/// This function returns `Ok(())` if all messages were sent successfully, or an `Err(ClientError)` if an error occurs.
///
/// This function sends the message and waits for a response. This cycle is repeated until the sent message count has reached the optional `count_option` limit.
pub async fn send_unidirectional(
    connection: &Connection,
    message: &Message,
    count_option: Option<u32>,
    inbox: &mut Vec<Message>,
) -> Result<(), ClientError> {
    let mut send_stream = connection.open_uni().await?;
    let mut recv_stream = connection.accept_uni().await?;

    let mut sent_count = 0;
    loop {
        write_message(&mut send_stream, message)
            .await
            .map_err(StreamError::from)?;

        let response = read_next_message(&mut recv_stream)
            .await
            .map_err(StreamError::from)?;

        println!("Received response data: {}", response.get_data());

        inbox.push(response);

        sent_count += 1;

        if let Some(count) = count_option {
            if sent_count >= count {
                break;
            }
        }
    }

    Ok(())
}

/// Sends messages over a connection using datagrams.
///
/// # Arguments
///
/// * `connection` - The connection over which the message is sent.
/// * `message` - The message to be sent.
/// * `count_option` - Optional argument to limit the number of times the message is sent. If `None`, the message is sent indefinitely.
///
/// # Returns
///
/// This function returns `Ok(())` if all messages were sent successfully, or an `Err(ClientError)` if an error occurs.
///
/// This function sends the message and waits for a response. This cycle is repeated until the sent message count has reached the optional `count_option` limit.

pub async fn send_datagram(
    connection: &Connection,
    message: &Message,
    count_option: Option<u32>,
    inbox: &mut Vec<Message>,
) -> Result<(), ClientError> {
    let mut sent_count = 0;
    loop {
        let datagram = message
            .as_bytes()
            .map_err(|e| StreamError::WriteError(WriteStreamError::from(e)))?;

        connection
            .send_datagram(&datagram)
            .map_err(|e| StreamError::WriteError(WriteStreamError::from(DatagramError::from(e))))?;

        sent_count += 1;

        // Loop as we need to make sure all datagrams are received by the server
        // and we got all the responses back.
        loop {
            if let Ok(response) = connection.receive_datagram().await {
                let message = Message::from_bytes(&response)
                    .map_err(|e| StreamError::ReadError(ReadStreamError::from(e)))?;

                println!("Received response data: {}", message.get_data());

                inbox.push(message);

                break;
            }

            // TODO: move this to a configuration variable as magic numbers are evil
            sleep(Duration::from_millis(100)).await;
        }

        if let Some(count) = count_option {
            if sent_count >= count {
                break;
            }
        }
    }

    Ok(())
}
