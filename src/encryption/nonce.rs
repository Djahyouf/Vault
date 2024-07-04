use rand::Rng;

/// Generates a random nonce of 12 bytes
///
/// # Returns
/// A vector of bytes representing the nonce
pub fn generate_nonce() -> Vec<u8> {
    let mut salt = vec![0u8; 12];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}
