use base64::{decode, encode};
use std::error::Error;
use std::io::{self, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn ask_for_master_pswd() -> String {
    print!("Enter master password : ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut password = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('\n') => break,
            termion::event::Key::Char(c) => {
                password.push(c);
                write!(stdout, "*").unwrap();
                stdout.flush().unwrap();
            }
            termion::event::Key::Backspace => {
                if !password.is_empty() {
                    password.pop();
                    write!(stdout, "\u{8} \u{8}").unwrap();
                    stdout.flush().unwrap();
                }
            }
            _ => {}
        }
    }

    write!(stdout, "\n\r").unwrap();
    stdout.flush().unwrap();

    password
}

/// Converts a Vec<u8> to a Base64 encoded String.
///
/// # Parameters
/// - `data`: The Vec<u8> to convert
///
/// # Returns
/// A Base64 encoded String representing the data
pub fn vec_u8_to_base64(data: Vec<u8>) -> String {
    encode(data)
}

/// Converts a Base64 encoded String back to a Vec<u8>.
///
/// # Parameters
/// - `encoded`: The Base64 encoded String
///
/// # Returns
/// A Result containing the Vec<u8> or an error if decoding fails
pub fn base64_to_vec_u8(encoded: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = decode(encoded)?;
    Ok(decoded)
}

