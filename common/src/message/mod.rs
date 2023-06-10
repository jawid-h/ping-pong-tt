use serde::{Deserialize, Serialize};

use crate::{
    error::SerializationError,
    serialization::{deserialize_message, serialize_message},
};

pub mod id;
pub mod request;
pub mod response;

/// An enumeration of the possible types of messages that can be sent or received in the system.
///
/// The `Message` enum includes two variants:
///
/// - `Request`: This variant wraps a `RequestMessage`, which represents a request from the client.
/// - `Response`: This variant wraps a `ResponseMessage`, which represents a response from the server.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Message {
    Request(request::RequestMessage),
    Response(response::ResponseMessage),
}

/// A representation of the different types of messages that can be part of a `Message`.
///
/// The `MessageType` enum includes two variants:
///
/// - `Request`: Represents a request message.
/// - `Response`: Represents a response message.
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum MessageType {
    Request = 0,
    Response = 1,
}

impl Message {
    /// Gest the data of the underlying message type.
    ///
    /// # Returns
    ///
    /// An underlying message type data as `String`.
    pub fn get_data(&self) -> String {
        match self {
            Self::Request(request) => request.data.clone(),
            Self::Response(response) => response.data.clone(),
        }
    }

    /// Gest the message as it's byte representation.
    ///
    /// # Returns
    ///
    /// Message as it's byte representaton in for of `Vec<u8>`.
    pub fn as_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        serialize_message(self)
    }

    /// Constructs a new `Message` from it's byte representation.
    ///
    /// # Parameters
    ///
    /// * `bytes` - The byte representation of the message.
    ///
    /// # Returns
    ///
    /// An instance of `Message`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        deserialize_message(bytes)
    }

    /// Constructs a new `RequestMessage`.
    ///
    /// This function takes a string as the message content, assigns a `MessageType::Request` to the `message_type`,
    ///
    /// # Parameters
    ///
    /// * `data` - The content of the message.
    ///
    /// # Returns
    ///
    /// An instance of `RequestMessage`.
    pub fn new_request(data: String) -> Self {
        Self::Request(request::RequestMessage::new(data))
    }

    /// Constructs a new `ResponseMessage`.
    ///
    /// This function takes the request ID and a string as the message content, assigns a `MessageType::Response` to the `message_type`,
    ///
    /// # Parameters
    ///
    /// * `request_id` - The ID of the request this response is for.
    ///
    /// # Returns
    ///
    /// An instance of `ResponseMessage`.
    pub fn new_response(request_id: &[u8], data: String) -> Self {
        Self::Response(response::ResponseMessage::new(request_id, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::message::id::generate_id;

    mod new_response {
        use super::*;

        #[test]
        fn test_should_have_correct_type_and_id() {
            let request_id = vec![1, 2, 3, 4];
            let text = "Ping!".to_string();

            let message = Message::new_response(&request_id, text.clone());

            match message {
                Message::Response(response) => {
                    assert_eq!(response.request_id, request_id);
                    assert_eq!(response.message_type, MessageType::Response);
                    assert_eq!(response.data, text);
                    assert_eq!(response.id, generate_id(text.as_bytes()));
                }
                _ => panic!("Message should be a response"),
            }
        }
    }

    mod new_request {
        use super::*;

        #[test]
        fn test_should_have_correct_type_and_id() {
            let text = "Ping!".to_string();

            let message = Message::new_request(text.clone());

            match message {
                Message::Request(request) => {
                    assert_eq!(request.message_type, MessageType::Request);
                    assert_eq!(request.data, text);
                    assert_eq!(request.id, generate_id(text.as_bytes()));
                }
                _ => panic!("Message should be a request"),
            }
        }
    }

    mod as_bytes {
        use super::*;

        #[test]
        fn test_should_return_correct_bytes() {
            let text = "Ping!".to_string();
            let message = Message::new_request(text);

            let bytes = message.as_bytes().unwrap();

            assert_eq!(
                bytes,
                vec![
                    0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 112, 120, 96, 80, 200, 192, 175, 162, 219,
                    199, 236, 67, 228, 162, 39, 80, 11, 85, 93, 87, 250, 130, 196, 232, 191, 100,
                    195, 97, 47, 201, 85, 57, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 80, 105, 110,
                    103, 33
                ]
            );
        }
    }

    mod from_bytes {
        use super::*;

        #[test]
        fn test_should_return_correct_message() {
            let bytes = vec![
                0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 112, 120, 96, 80, 200, 192, 175, 162, 219,
                199, 236, 67, 228, 162, 39, 80, 11, 85, 93, 87, 250, 130, 196, 232, 191, 100, 195,
                97, 47, 201, 85, 57, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 80, 105, 110, 103, 33,
            ];
            let text = "Ping!".to_string();

            let message = Message::from_bytes(&bytes).unwrap();

            assert_eq!(message, Message::new_request(text));
        }

        #[test]
        fn test_should_return_error_for_invalid_bytes() {
            let bytes = vec![0, 0, 0, 0, 32, 0, 0, 0, 0, 0];

            let message = Message::from_bytes(&bytes);

            assert!(message.is_err());
        }
    }
}
