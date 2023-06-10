use wtransport::{RecvStream, SendStream};

use crate::{
    error::{ReadStreamError, WriteStreamError},
    message::Message,
};

/// Read an exact number of bytes from a stream.
///
/// This function reads bytes from the stream into the buffer provided until the buffer is full.
/// If the stream ends before that, it returns an error.
///
/// Will wait until the buffer is full.
///
/// # Parameters
///
/// * `stream` - A mutable reference to the stream from which bytes are to be read.
/// * `buf` - A mutable slice of bytes that the read bytes will be put into.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok` - If the entire buffer is filled.
/// * `Err` - If an error occurs during reading from the stream.
pub async fn read_exact(stream: &mut RecvStream, buf: &mut [u8]) -> Result<(), ReadStreamError> {
    let mut read = 0;
    while read < buf.len() {
        match stream.read(&mut buf[read..]).await {
            Ok(Some(n)) => read += n,
            Ok(None) => return Err(ReadStreamError::StreamStopped),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

/// Reads the next message from a stream.
///
/// This function reads bytes from the stream and attempts to deserialize them into a `Message`.
///
/// # Parameters
///
/// * `stream` - A mutable reference to the stream from which the message is to be read.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok` - Contains the `Message` read from the stream.
/// * `Err` - If an error occurs during reading from the stream or deserializing the message.
pub async fn read_next_message(stream: &mut RecvStream) -> Result<Message, ReadStreamError> {
    let mut bytes_to_read_buffer: [u8; 8] = [0; 8];
    read_exact(stream, &mut bytes_to_read_buffer).await?;

    let bytes_to_read = u64::from_be_bytes(bytes_to_read_buffer);

    let mut msg_bytes = vec![0; bytes_to_read as usize];
    read_exact(stream, &mut msg_bytes).await?;

    let message = Message::from_bytes(&msg_bytes)?;

    Ok(message)
}

/// Writes a message to a stream.
///
/// This function serializes the provided `Message` into bytes and then writes them to the stream.
///
/// # Parameters
///
/// * `stream` - A mutable reference to the stream into which the message is to be written.
/// * `message` - A reference to the `Message` that is to be written to the stream.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok` - If the entire message is written.
/// * `Err` - If an error occurs during writing to the stream or serializing the message.
pub async fn write_message(
    stream: &mut SendStream,
    message: &Message,
) -> Result<(), WriteStreamError> {
    let msg_bytes = message.as_bytes()?;

    stream.write(&msg_bytes.len().to_be_bytes()).await?;
    stream.write_all(&msg_bytes).await?;

    Ok(())
}
