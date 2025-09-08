use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::distributions::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::hash_and_reverse::Encryptor;
use crate::VeilError;


pub async fn start_server(addr: &str) -> Result<(), VeilError> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            let n = match socket.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => return eprintln!("Failed to read: {}", e),
            };

            let received = String::from_utf8_lossy(&buffer[..n]);
            let parts: Vec<&str> = received.splitn(2, '|').collect();
            if parts.len() != 2 {
                return eprintln!("Invalid input format");
            }

            let encryptor_type = parts[0];
            let input = parts[1];

            let salt: String = (0..16).map(|_| rand::thread_rng().sample(rand::distributions::Alphanumeric) as char).collect();

            let encryptor: Box<dyn Encryptor + Send> = match encryptor_type {
                "hash" => Box::new(crate::hash_and_reverse::HashEncryptor),
                "reverse" => Box::new(crate::hash_and_reverse::ReverseEncryptor),
                "aes" => {
                    let key = vec![0u8; 32];
                    let iv = vec![0u8; 16];
                    Box::new(crate::aes::AesEncryptor::new(key, iv))
                }
                _ => return eprintln!("Unknown encryptor type: {}", encryptor_type),
            };

            let encrypted = match encryptor.encrypt(input, &salt) {
                Ok(enc) => enc,
                Err(e) => return eprintln!("Encryption failed: {}", e),
            };

            if let Err(e) = socket.write_all(encrypted.as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
            }
        });
    }
}


pub async fn start_client(addr: &str, encryptor_type: &str, input: &str) -> Result<(), VeilError> {
    let mut stream = TcpStream::connect(addr).await?;

    let message = format!("{}|{}", encryptor_type, input);
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    let encrypted_response = String::from_utf8_lossy(&buffer[..n]);

    println!("Encrypted response from server: {}", encrypted_response);

    Ok(())
}
