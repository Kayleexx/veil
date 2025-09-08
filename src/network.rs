use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::hash_and_reverse::Encryptor;
use crate::VeilError;
use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;

pub async fn start_server(addr: &str) -> Result<(), VeilError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Failed to bind: {}", e)))?;

    println!("Server running on {}", addr);

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .map_err(|e| VeilError::Network(format!("Accept failed: {}", e)))?;

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            let n = match socket.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                    return;
                }
            };

            let received = String::from_utf8_lossy(&buffer[..n]);
            let parts: Vec<&str> = received.splitn(2, '|').collect();

            if parts.len() != 2 {
                eprintln!("Invalid input format");
                return;
            }

            let encryptor_type = parts[0];
            let input = parts[1];

            // Generate a random salt
            let mut rng = StdRng::from_entropy();
            let mut salt_bytes = [0u8; 16];
            rng.fill_bytes(&mut salt_bytes);
            let salt = hex::encode(salt_bytes);

            let encryptor: Box<dyn Encryptor + Send> = match encryptor_type {
                "hash" => Box::new(crate::hash_and_reverse::HashEncryptor),
                "reverse" => Box::new(crate::hash_and_reverse::ReverseEncryptor),
                "aes" => {
                    let key = vec![0u8; 32]; // dummy key
                    let iv = vec![0u8; 16];  // dummy IV
                    Box::new(crate::aes::AesEncryptor::new(key, iv))
                }
                _ => {
                    eprintln!("Unknown encryptor type: {}", encryptor_type);
                    return;
                }
            };

            let encrypted = match encryptor.encrypt(input, &salt) {
                Ok(enc) => enc,
                Err(e) => {
                    eprintln!("Encryption failed: {}", e);
                    return;
                }
            };

            // Send encrypted response back to client
            if let Err(e) = socket.write_all(encrypted.as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
            } else {
                println!(
                    "Processed request: type='{}', input='{}', output='{}'",
                    encryptor_type, input, encrypted
                );
            }
        });
    }
}

pub async fn start_client(addr: &str, message: &str) -> Result<(), VeilError> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Connect failed: {}", e)))?;

    stream.write_all(message.as_bytes())
        .await
        .map_err(|e| VeilError::Network(format!("Write failed: {}", e)))?;

    let mut buffer = vec![0u8; 4096];
    let n = stream.read(&mut buffer)
        .await
        .map_err(|e| VeilError::Network(format!("Read failed: {}", e)))?;

    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);
    println!("Message processed successfully.\n");

    Ok(())
}
