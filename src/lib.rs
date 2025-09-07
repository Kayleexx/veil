
pub mod hash_and_reverse; 
pub mod aes;              

pub use hash_and_reverse::*;
pub use aes::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes::AesEncryptor;
    use crate::hash_and_reverse::Encryptor;

    #[test]
    fn test_aes_encryptor() {
        let key = vec![0u8; 32]; // 32-byte AES-256 key
        let iv = vec![0u8; 16];  // 16-byte IV

        let aes = AesEncryptor::new(key, iv);

        let input = "hello";
        let salt = "world";

        let encrypted = aes.encrypt(input, salt).unwrap();

        assert!(encrypted.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(!encrypted.is_empty());
    }
}
