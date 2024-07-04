use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

/// Derives a 256-bit key from the master password and salt using PBKDF2 with SHA-256
///
/// # Parameters
/// - `password`: The master password
/// - `salt`: The salt value
///
/// # Returns
/// A vector of bytes representing the derived key
pub fn gen_unique_key(pswd: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0u8, 32];
    pbkdf2_hmac::<Sha256>(pswd.as_bytes(), salt, 100_000, &mut key);
    key
}
