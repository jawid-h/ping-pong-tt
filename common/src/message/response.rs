use serde::{Deserialize, Serialize};

use super::{id::generate_id, MessageType};

/// Struct representing a Response Message.
///
/// This struct encapsulates the data for a response message in the application.
/// Each response message has an identifier `id`, the request's ID `request_id`, a type `message_type`, and the message `data`.
///
/// # Fields
///
/// * `id` - A vector of bytes that uniquely identifies this message.
/// * `request_id` - The ID of the request this response is for.
/// * `message_type` - Enum specifying the type of the message.
/// * `data` - The content of the response message.
///
/// The `id` is automatically generated based on the message content when a new `ResponseMessage` is created.
///
/// # Serialization
///
/// This struct can be serialized and deserialized with Serde.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseMessage {
    pub id: Vec<u8>,
    pub request_id: Vec<u8>,
    pub message_type: MessageType,
    pub data: String,
}

impl ResponseMessage {
    /// Constructs a new `ResponseMessage`.
    ///
    /// This function takes the request ID and a string as the message content, assigns a `MessageType::Response` to the `message_type`,
    /// generates an ID based on the message content, and returns a new instance of `ResponseMessage`.
    ///
    /// # Parameters
    ///
    /// * `request_id` - The ID of the request this response is for.
    /// * `data` - The content of the message.
    ///
    /// # Returns
    ///
    /// An instance of `ResponseMessage`.
    pub fn new(request_id: &[u8], data: String) -> Self {
        Self {
            id: generate_id(data.as_bytes()),
            request_id: request_id.to_vec(),
            message_type: MessageType::Response,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_have_correct_type_and_id() {
        let request_id = vec![1, 2, 3, 4];
        let text = "Ping!".to_string();

        let message = ResponseMessage::new(&request_id, text.clone());

        assert_eq!(message.request_id, request_id);
        assert_eq!(message.message_type, MessageType::Response);
        assert_eq!(message.data, text);
        assert_eq!(message.id, generate_id(text.as_bytes()));
    }
}
