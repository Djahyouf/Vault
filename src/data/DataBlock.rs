#[derive(Debug)]
pub struct DataBlock {
    pub url: String,
    pub username: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl DataBlock {
    pub fn new(
        url: String,
        username: String,
        password: Vec<u8>,
        salt: Vec<u8>,
        nonce: Vec<u8>,
    ) -> Self {
        DataBlock {
            url,
            username,
            password,
            salt,
            nonce,
        }
    }
}
