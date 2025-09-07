use veil::hash_and_reverse::{HashEncryptor, ReverseEncryptor, CryptoService};
use veil::aes::AesEncryptor;

fn main() {
    // SHA256
    let hash_service = CryptoService::new(Box::new(HashEncryptor));
    let hashed = hash_service.encrypt("hello", "salt").unwrap();
    println!("SHA256 hashed: {}", hashed);

    // Reverse
    let rev_service = CryptoService::new(Box::new(ReverseEncryptor));
    let reversed = rev_service.encrypt("hello", "salt").unwrap();
    println!("Reversed: {}", reversed);

    // AES
    let aes = AesEncryptor::new(vec![0u8; 32], vec![0u8; 16]);
    let aes_service = CryptoService::new(Box::new(aes));
    let encrypted = aes_service.encrypt("hello", "salt").unwrap();
    println!("AES-CBC encrypted: {}", encrypted);
}
