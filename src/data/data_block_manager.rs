use crate::data_block::DataBlock;
use crate::utils;

pub fn data_block_2_storing_format(data_block: &DataBlock) -> String {
    let salt_hex = utils::vec_u8_to_base64(data_block.salt.clone());
    let nonce_hex = utils::vec_u8_to_base64(data_block.nonce.clone());
    let password_hex = utils::vec_u8_to_base64(data_block.password.clone());

    format!(
        "{} | {} | {}",
        data_block.url,
        data_block.username,
        format_args!("{}{}{}", salt_hex, nonce_hex, password_hex)
    )
}

pub fn storing_format_2_data_block(s: &String) -> DataBlock {
    let parts: Vec<&str> = s.splitn(3, " | ").collect();

    if parts.len() != 3 {
        panic!("Invalid string format!");
    }

    let url = parts[0].to_string();
    let username = parts[1].to_string();

    let concatenated_data = parts[2];
    let salt = utils::base64_to_vec_u8(&concatenated_data[..24]);
    let nonce = utils::base64_to_vec_u8(&concatenated_data[24..24 + 16]);
    let password = utils::base64_to_vec_u8(&concatenated_data[24 + 16..]);

    DataBlock {
        url,
        username,
        password: password.expect("REASON"),
        salt: salt.expect("REASON"),
        nonce: nonce.expect("REASON"),
    }
}
