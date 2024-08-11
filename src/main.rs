use std::str;

#[path = "data/data_block.rs"]
mod data_block;
#[path = "encryption/decryption.rs"]
mod decryption;
#[path = "encryption/encryption.rs"]
mod encryption;
#[path = "encryption/keys.rs"]
mod keys;
#[path = "encryption/nonce.rs"]
mod nonce;
#[path = "encryption/salt.rs"]
mod salt;
mod utils;

fn main() {
    let master_password = utils::ask_for_master_pswd();
    println!("You entered the following : {}", master_password);

    let plaintext = b"plaintext message";
    println!("Plaintext: {:?}", plaintext);
    println!("Plaintext: {}", str::from_utf8(plaintext).unwrap());

    let (ciphertext, salt, nonce) = encryption::encrypt(plaintext, &master_password);
    println!("Ciphertext: {:?}", ciphertext);
    println!("Salt: {:?}", salt);
    println!("Nonce: {:?}", nonce);

    let decypheredtext = decryption::decrypt(&ciphertext, &master_password, &salt, &nonce);
    println!("decypheredtext: {:?}", decypheredtext);
}
