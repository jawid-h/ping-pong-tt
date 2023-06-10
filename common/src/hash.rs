/// Computes the SHA256 hash of the given data. Altough implementation of the hash is interchangeable.
///
/// This function takes a slice of bytes as input, hashes it using the hashing algorithm,
/// and returns the hash as a `Vec<u8>`.
///
/// # Parameters
///
/// * `data`: A byte slice (`&[u8]`) representing the data to be hashed.
///
/// # Returns
///
/// A vector of bytes (`Vec<u8>`) representing the hash of the input data.
pub fn hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_should_produce_same_hash_for_the_same_data() {
        let data = b"original data";

        let hash1 = super::hash(data);
        let hash2 = super::hash(data);

        assert_eq!(hash1, hash2);
    }
}
