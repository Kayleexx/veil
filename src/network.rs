use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;
use rand::SeedableRng;
use crate::hash_and_reverse::Encryptor;
use crate::VeilError;

pub async fn start_server(addr: &str) -> Result<(), VeilError> {
    let listener = TcpListener::bind(addr).await.map_err(|e| VeilError::Network(format!("Failed to bind: {}", e)))?;

    loop {
        let (mut socket, _) = listener.accept().await.map_err(|e| VeilError::Network(format!("Accept failed: {}", e)))?;

        tokio::spawn(async move {
    let mut buffer = vec![0u8; 1024];

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

    // Create a Send-safe RNG here, inside async block
    let mut rng = rand::rngs::StdRng::from_entropy();
    let salt: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                .chars()
                .nth(idx)
                .unwrap()
        })
        .collect();

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

            if let Err(e) = socket.write_all(encrypted.as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
            }
        });
    }
}

pub async fn start_client(addr: &str, encryptor_type: &str, input: &str) -> Result<(), VeilError> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Connect failed: {}", e)))?;

    let message = format!("{}|{}", encryptor_type, input);
    stream.write_all(message.as_bytes())
        .await
        .map_err(|e| VeilError::Network(format!("Write failed: {}", e)))?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer)
        .await
        .map_err(|e| VeilError::Network(format!("Read failed: {}", e)))?;

    let encrypted_response = String::from_utf8_lossy(&buffer[..n]);
    println!("Encrypted response from server: {}", encrypted_response);

    Ok(())
}
