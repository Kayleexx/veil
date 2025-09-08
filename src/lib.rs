pub mod aes;
pub mod network;
pub mod utils;
pub mod hash_and_reverse;
pub mod secret_sharing;

use std::fmt;

#[derive(Debug)]
pub enum VeilError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Encryption(String),
    InvalidInput(String),
    Network(String),  
}

impl fmt::Display for VeilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VeilError::Io(e) => write!(f, "IO error: {}", e),
            VeilError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            VeilError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            VeilError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            VeilError::Network(msg) => write!(f, "Network error: {}", msg), // ✅
        }
    }
}

impl std::error::Error for VeilError {}

impl From<std::io::Error> for VeilError {
    fn from(e: std::io::Error) -> Self { VeilError::Io(e) }
}

impl From<std::string::FromUtf8Error> for VeilError {
    fn from(e: std::string::FromUtf8Error) -> Self { VeilError::Utf8(e) }
}


