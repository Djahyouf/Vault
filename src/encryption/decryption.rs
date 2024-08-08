use crate::keys;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};

/// Decrypts ciphertext using AES-GCM with the derived key and the provided nonce
///
/// # Parameters
/// - `ciphertext`: The encrypted data to decrypt
/// - `master`: The master password
/// - `salt`: The salt used during encryption
/// - `nonce`: The nonce used during encryption
///
/// # Returns
/// The decrypted plaintext as a `Vec<u8>`
pub fn decrypt(ciphertext: &[u8], master: &str, salt: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = keys::gen_unique_key(master, salt);

    let sliced_key: Key<Aes256Gcm> = *Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(&sliced_key);

    let decrypted_text = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("Error: Decryption has failed!");

    decrypted_text
}
