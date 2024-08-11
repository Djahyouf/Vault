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

