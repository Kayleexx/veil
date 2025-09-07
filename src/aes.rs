use aes::Aes256;
use cbc::Encryptor as CbcEncryptor;
use aes::cipher::{KeyIvInit, BlockEncryptMut, generic_array::GenericArray};
use hex;

use crate::hash_and_reverse::Encryptor; 

pub struct AesEncryptor {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl AesEncryptor {
    pub fn new(key: Vec<u8>, iv: Vec<u8>) -> Self {
        assert_eq!(key.len(), 32, "Key must be 32 bytes for AES-256");
        assert_eq!(iv.len(), 16, "IV must be 16 bytes for AES-256");
        Self { key, iv }
    }
}

impl Encryptor for AesEncryptor {
    fn encrypt(&self, input: &str, salt: &str) -> Result<String, String> {
        let combined = format!("{}{}", input, salt);
        let mut buffer = combined.as_bytes().to_vec();

        // Manual PKCS7 padding
        let block_size = 16;
        let pad_len = block_size - (buffer.len() % block_size);
        buffer.extend(vec![pad_len as u8; pad_len]);

        // Initialize AES-CBC encryptor
        let mut cipher = CbcEncryptor::<Aes256>::new_from_slices(&self.key, &self.iv)
            .map_err(|e| format!("Cipher creation error: {:?}", e))?;

        // Encrypt in-place
        let mut ciphertext = vec![0u8; buffer.len()];
        for (i, chunk) in buffer.chunks(block_size).enumerate() {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block_mut(&mut block);
            ciphertext[i*block_size..(i+1)*block_size].copy_from_slice(&block);
        }

        Ok(hex::encode(ciphertext))
    }
}
