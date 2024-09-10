use std::str;

mod constants;
#[path = "data/DataBlock.rs"]
mod data_block;
#[path = "data/data_block_manager.rs"]
mod data_block_manager;
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
#[path = "master/verification.rs"]
mod verification;

fn main() {
    println!("Welcome to Vault");

    let mut master_password: String = String::new();
    if verification::master_exist() {
        master_password = utils::ask_for_master_pswd();
        // Test if master is correct here.
    } else {
        println!(
            "
                    ********************************************************
                    *                                                      *
                    *          VAULT DETECTED NO MASTER PASSWORD           *
                    *                                                      *
                    *     You are about to create a new master password.   *
                    *                                                      *
                    *     WARNING: There is NO way to retrieve it!         *
                    *                                                      *
                    *     Make sure your master password is strong, as     *
                    *     it will be used to encrypt all other passwords.  *
                    *                                                      *
                    *     DO NOT store your master password anywhere—      *
                    *     engrave it into your brain!                      *
                    *                                                      *
                    ********************************************************
            "
        );

        master_password = utils::ask_for_master_pswd();
        println!("You entered the following : {}", master_password);
        // Store password here.
        verification::create_master_key(&master_password);
    }

    /*

    let plaintext = b"plaintext message";
    println!("Plaintext: {}", str::from_utf8(plaintext).unwrap());

    let (ciphertext, salt, nonce) = encryption::encrypt(plaintext, &master_password);
    let encoded_ciphertext = utils::vec_u8_to_base64(ciphertext.clone());
    println!("Ciphertext (Base64): {}", encoded_ciphertext);
    println!("Ciphertext: \n{:?}", ciphertext);
    println!("Salt: {:?}", salt);
    println!("Nonce: {:?}", nonce);

    let decypheredtext = decryption::decrypt(&ciphertext, &master_password, &salt, &nonce);
    println!("decypheredtext: {:?}", decypheredtext);
    println!(
        "decypheredtext: {}",
        str::from_utf8(&decypheredtext).unwrap()
    );

    let block = data_block::DataBlock::new(
        "netflix".to_string(),
        "user".to_string(),
        ciphertext,
        salt.clone(),
        nonce.clone(),
    );

    println!("data_block :\n {:?}", block);

    println!("salt base64 : {}", utils::vec_u8_to_base64(salt.clone()));
    println!(
        "salt base64 len : {}",
        utils::vec_u8_to_base64(salt.clone()).len()
    );
    println!("nonce base64 : {}", utils::vec_u8_to_base64(nonce.clone()));
    println!(
        "nonce base64 len : {}",
        utils::vec_u8_to_base64(nonce.clone()).len()
    );

    let written_form: String = data_block_manager::data_block_2_storing_format(&block);
    println!("written form : {}", written_form);

    println!();

    let block_from_string = data_block_manager::storing_format_2_data_block(&written_form);
    println!(
        "data_block created from string format :\n {:?}",
        block_from_string
    );

    */
}
