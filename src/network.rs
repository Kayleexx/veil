use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::distributions::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::hash_and_reverse::Encryptor;

pub async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            // Read data from client
            let n = match socket.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket: {:?}", e);
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

            // Use StdRng which is Send-safe
            let mut rng = StdRng::from_entropy();
            let salt: String = (0..16)
                .map(|_| rng.sample(Alphanumeric) as char)
                .collect();

            // Choose encryptor dynamically
            let encryptor: Box<dyn Encryptor + Send> = match encryptor_type {
                "hash" => Box::new(crate::hash_and_reverse::HashEncryptor),
                "reverse" => Box::new(crate::hash_and_reverse::ReverseEncryptor),
                "aes" => {
                    let key = vec![0u8; 32]; // Demo key
                    let iv = vec![0u8; 16];  // Demo IV
                    Box::new(crate::aes::AesEncryptor::new(key, iv))
                }
                _ => {
                    eprintln!("Unknown encryptor type: {}", encryptor_type);
                    return;
                }
            };

            // Encrypt
            let encrypted = match encryptor.encrypt(input, &salt) {
                Ok(enc) => enc,
                Err(e) => {
                    eprintln!("Encryption failed: {}", e);
                    return;
                }
            };

            // Send encrypted response
            if let Err(e) = socket.write_all(encrypted.as_bytes()).await {
                eprintln!("Failed to write to socket: {:?}", e);
            }
        });
    }
}

pub async fn start_client(
    addr: &str,
    encryptor_type: &str,
    input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr).await?;

    // Send encryptor type + input
    let message = format!("{}|{}", encryptor_type, input);
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    let encrypted_response = String::from_utf8_lossy(&buffer[..n]);

    println!("Encrypted response from server: {}", encrypted_response);

    Ok(())
}
