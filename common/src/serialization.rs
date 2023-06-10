use crate::{error::SerializationError, message::Message};

/// Serializes a Message into a Vec<u8>.
///
/// This function takes a reference to a Message struct as an argument, serializes it into a byte vector using
/// bincode (altough it is changeable), and returns the resulting byte vector. If serialization fails, it returns a SerializationError.
///
/// # Parameters
///
/// * `message` - A reference to the Message struct that needs to be serialized.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok` - Contains a Vec<u8> representing the serialized form of the Message.
/// * `Err` - Contains a `SerializationError` indicating that serialization has failed.
pub fn serialize_message(message: &Message) -> Result<Vec<u8>, SerializationError> {
    bincode::serialize(&message).map_err(|_| SerializationError::SerializationFailed {
        message: message.clone(),
    })
}

/// Deserializes a Vec<u8> into a Message.
///
/// This function takes a byte slice as an argument, attempts to deserialize it into a Message struct using
/// bincode (altough it is changeable), and returns the resulting Message. If deserialization fails, it returns a SerializationError.
///
/// # Parameters
///
/// * `bytes` - A slice of bytes that needs to be deserialized into a Message.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok` - Contains the deserialized Message.
/// * `Err` - Contains a `SerializationError` indicating that deserialization has failed.
pub fn deserialize_message(bytes: &[u8]) -> Result<Message, SerializationError> {
    bincode::deserialize(bytes).map_err(|_| SerializationError::DeserializationFailed {
        bytes: bytes.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Message;

    #[test]
    fn test_should_serialize_message() {
        let message = Message::new_request("Ping!".to_string());

        let serialized_message = serialize_message(&message).unwrap();

        assert_eq!(
            serialized_message,
            vec![
                0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 112, 120, 96, 80, 200, 192, 175, 162, 219,
                199, 236, 67, 228, 162, 39, 80, 11, 85, 93, 87, 250, 130, 196, 232, 191, 100, 195,
                97, 47, 201, 85, 57, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 80, 105, 110, 103, 33,
            ]
        );
    }

    #[test]
    fn test_should_deserialize_message() {
        let serialized_message = vec![
            0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 112, 120, 96, 80, 200, 192, 175, 162, 219, 199,
            236, 67, 228, 162, 39, 80, 11, 85, 93, 87, 250, 130, 196, 232, 191, 100, 195, 97, 47,
            201, 85, 57, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 80, 105, 110, 103, 33,
        ];

        let message = deserialize_message(&serialized_message).unwrap();

        assert_eq!(message, Message::new_request("Ping!".to_string()));
    }

    #[test]
    fn test_return_an_error_in_case_deserialization_fails() {
        let serialized_message = vec![
            1, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 112, 120, 96, 80, 200, 192, 175, 162, 219, 199,
            236, 67, 228, 162, 39, 80, 11, 85, 93, 87, 250, 130, 196, 232, 191, 100, 195, 97, 47,
            201, 85, 57, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 80, 105, 110, 103, 33,
        ];

        match deserialize_message(&serialized_message) {
            Ok(_) => panic!("Should return an error"),
            Err(error) => match error {
                SerializationError::DeserializationFailed { bytes } => {
                    assert_eq!(bytes, serialized_message)
                }
                _ => panic!("Should return a DeserializationFailed error"),
            },
        }
    }
}
