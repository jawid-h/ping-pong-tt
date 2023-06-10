use serde::{Deserialize, Serialize};

use super::{id::generate_id, MessageType};

/// Struct representing a Request Message.
///
/// This struct encapsulates the data for a request message in the application.
/// Each request message has an identifier `id`, a type `message_type`, and the message `data`.
///
/// # Fields
///
/// * `id` - A vector of bytes that uniquely identifies this message.
/// * `message_type` - Enum specifying the type of the message.
/// * `data` - The content of the request message.
///
/// The `id` is automatically generated based on the message content when a new `RequestMessage` is created.
///
/// # Serialization
///
/// This struct can be serialized and deserialized with Serde.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestMessage {
    pub id: Vec<u8>,
    pub message_type: MessageType,
    pub data: String,
}

impl RequestMessage {
    /// Constructs a new `RequestMessage`.
    ///
    /// This function takes a string as the message content, assigns a `MessageType::Request` to the `message_type`,
    /// generates an ID based on the message content, and returns a new instance of `RequestMessage`.
    ///
    /// # Parameters
    ///
    /// * `data` - The content of the message.
    ///
    /// # Returns
    ///
    /// An instance of `RequestMessage`.
    pub fn new(data: String) -> Self {
        Self {
            id: generate_id(data.as_bytes()),
            message_type: MessageType::Request,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_have_correct_type_and_id() {
        let text = "Ping!".to_string();

        let message = RequestMessage::new(text.clone());

        assert_eq!(message.data, text);
        assert_eq!(message.message_type, MessageType::Request);
        assert_eq!(message.id, generate_id(text.as_bytes()));
    }
}
