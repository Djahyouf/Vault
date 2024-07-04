use rand::Rng;

/// Generates a random salt of the given length
///
/// # Parameters
/// - `length` : The length of the salt in bytes
///
/// # Returns
/// A vector of bytes representing the salt
pub fn generate_salt(length: usize) -> Vec<u8> {
    let mut salt = vec![0u8; length];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}
