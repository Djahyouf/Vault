mod keys;
mod nonce;
mod salt;

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};

/// Encrypts plaintext using AES-GCM with the derived key and a random nonce
///
/// # Parameters
/// - `plaintext`: The data to encrypt
/// - `master`: The master password
///
/// # Returns
/// A tuple containing the ciphered text, salt used, and nonce used
pub fn encrypt(plaintext: &[u8], master: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let salt = salt::generate_salt(16);
    println!("salt produced ! : {:?}", salt);
    let key = keys::gen_unique_key(master, &salt);
    println!("derived key produced ! : {:?}", key);

    let sliced_key: Key<Aes256Gcm> = *Key::<Aes256Gcm>::from_slice(&key);
    println!("sliced key produced !");
    let cipher = Aes256Gcm::new(&sliced_key);
    println!("cipher produced !");

    let nonce = nonce::generate_nonce();
    println!("nonce produced !");

    let ciphered_text = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("Error: Encryption has failed !");
    println!("ciphered text produced !");

    (ciphered_text, salt, nonce)
}
