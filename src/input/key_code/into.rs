use crate::input::KeyCode;

impl Into<u8> for KeyCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<char> for KeyCode {
    fn into(self) -> char {
        self as u8 as char
    }
}
