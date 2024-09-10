use crate::constants;
use crate::keys;
use crate::salt;
use crate::utils;
use std::fs::metadata;
use std::fs::File;
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

pub fn create_master_key(master: &str) -> String {
    let salt = salt::generate_salt(16);
    let key = keys::gen_unique_key(master, &salt);

    let salt_hex = utils::vec_u8_to_base64(salt);
    let key_hex = utils::vec_u8_to_base64(key);

    format!("{}{}", salt_hex, key_hex)
}

fn write_master_key(key: String) {
    // TODO
}
