use crate::constants;
use crate::keys;
use crate::salt;
use crate::utils;
use std::fs::{metadata, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn is_storage_empty() -> bool {
    let path = Path::new(constants::STORAGE_FILE_PATH);

    if !path.exists() {
        println!("File does not exist.");

        println!("Creating file ...");
        let file = File::create(path);
        if file.is_err() {
            println!("Error while creating file.")
        } else {
            println!("File created successfully !");
        }

        return true;
    }

    match metadata(path) {
        Ok(metadata) => metadata.len() == 0,
        Err(e) => {
            eprintln!("Error reading file metadata: {}", e);
            false
        }
    }
}

pub fn master_exist() -> bool {
    !is_storage_empty()
}

pub fn verify_master(master: &str) -> bool {
    let master_key: String = read_master_key().unwrap();

    let salt_base64 = &master_key[..constants::BASE64_SALT_LEN];
    let derived_key_base64 = &master_key[constants::BASE64_SALT_LEN..];

    let original_salt = utils::base64_to_vec_u8(salt_base64).unwrap();
    let original_derived_key = utils::base64_to_vec_u8(derived_key_base64).unwrap();

    let new_derived_key = keys::gen_unique_key(master, &original_salt);

    original_derived_key == new_derived_key
}

pub fn create_master_key(master: &str) {
    let salt = salt::generate_salt(16);
    let key = keys::gen_unique_key(master, &salt);

    let salt_hex = utils::vec_u8_to_base64(salt);
    let key_hex = utils::vec_u8_to_base64(key);

    let _ = write_master_key(format!("{}{}", salt_hex, key_hex));
}

fn write_master_key(key: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(constants::STORAGE_FILE_PATH)?;

    file.write_all(key.as_bytes())?;
    Ok(())
}

fn read_master_key() -> io::Result<String> {
    let file = File::open(constants::STORAGE_FILE_PATH)?;
    let mut reader = BufReader::new(file);

    // Read the first line (which contains the master key)
    let mut master_key = String::new();
    reader.read_line(&mut master_key)?;

    // Remove any trailing newline or carriage return
    Ok(master_key.trim().to_string())
}
