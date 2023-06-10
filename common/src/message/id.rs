use crate::hash;

/// Generates a unique identifier for a given data set.
/// Should produce same id for the same data.
///
/// This function takes a byte slice as an argument, applies a hash function to it,
/// and returns the resulting hash as a unique identifier in the form of a byte vector.
///
/// # Parameters
///
/// * `data` - A slice of bytes that needs to be hashed to create a unique identifier.
///
/// # Returns
///
/// A vector of bytes that represent a unique identifier generated from the input data.
pub fn generate_id(data: &[u8]) -> Vec<u8> {
    hash::hash(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generates_same_id_for_same_input() {
        let data = "The answer is - 42".as_bytes();

        let id1 = super::generate_id(data);
        let id2 = super::generate_id(data);

        assert_eq!(id1, id2);
    }
}
