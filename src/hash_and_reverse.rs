use sha2::{Sha256, Digest};

pub trait Encryptor: Send + Sync {
    fn encrypt(&self, input: &str, salt: &str) -> Result<String, String>;
}

pub struct HashEncryptor;

impl Encryptor for HashEncryptor {
    fn encrypt(&self, input: &str, salt: &str) -> Result<String, String> {
        let combined = format!("{}{}", input, salt);
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();
        let encrypted = format!("{:x}", result);
        Ok(encrypted)
    }
}

pub struct ReverseEncryptor;

impl Encryptor for ReverseEncryptor {
    fn encrypt(&self, input: &str, salt: &str) -> Result<String, String> {
        let combined = format!("{}{}", input, salt);
        let encrypted = combined.chars().rev().collect::<String>();
        Ok(encrypted)
    }
}

pub fn apply_encryption(encryptor: &dyn Encryptor, input: &str, salt: &str) -> Result<String, String> {
    encryptor.encrypt(input, salt)
}


pub fn apply_encryption_generic<E: Encryptor>(
    encryptor: &E,
    input: &str,
    salt: &str
) -> Result<String, String> {
    encryptor.encrypt(input, salt)
}


pub struct CryptoService {
    encryptor: Box<dyn Encryptor>,
}

impl CryptoService {
    pub fn new(encryptor: Box<dyn Encryptor>) -> Self {
        Self{ encryptor }
    }

    pub fn encrypt(&self, input: &str, salt: &str) -> Result<String, String> {
        self.encryptor.encrypt(input, salt)
    }
}

pub struct BorrowedCryptoService<'a> {
    encryptor: &'a dyn Encryptor, 
}

impl<'a> BorrowedCryptoService<'a> {
    pub fn new(encryptor: &'a dyn Encryptor) -> Self {
        Self { encryptor }
    }
    pub fn encrypt(&self, input: &str, salt: &str) -> Result<String, String> {
        self.encryptor.encrypt(input, salt)
    }
}
