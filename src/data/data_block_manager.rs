use crate::data_block::DataBlock;
use crate::utils;

pub fn data_block_2_storing_format(data_block: &DataBlock) -> String {
    // Convert the Vec<u8> fields to hexadecimal string representations
    let salt_hex = utils::vec_u8_to_base64(data_block.salt.clone());
    let nonce_hex = utils::vec_u8_to_base64(data_block.nonce.clone());
    let password_hex = utils::vec_u8_to_base64(data_block.password.clone());

    // Concatenate the fields into the required format
    format!(
        "{} | {} | {}",
        data_block.url,
        data_block.username,
        format!("{}{}{}", salt_hex, nonce_hex, password_hex)
    )
}
