pub enum Mode { ENCRYPT, DECRYPT }

impl Mode {
    pub fn from(s: &str) -> Option<Mode> {
        match s.trim() {
            "encrypt" | "enc" | "e" => Some(Mode::ENCRYPT),
            "decrypt" | "dec" | "d" => Some(Mode::DECRYPT),
            _ => None
        }
    }
}

pub mod argparser;
pub mod base64;
pub mod encrypt;
pub mod decrypt;
pub mod random;
