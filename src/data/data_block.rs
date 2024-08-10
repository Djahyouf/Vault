struct data_block {
    url: String,
    username: String,
    password: Vec<u8>,
    salt: Vec<u8>,
    nonce: Vec<u8>,
}
