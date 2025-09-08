use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;
use rand::SeedableRng;
use crate::hash_and_reverse::Encryptor;
use crate::VeilError;

pub async fn start_server(addr: &str) -> Result<(), VeilError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Failed to bind: {}", e)))?;

    println!("Server running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept()
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

            let result: Result<String, VeilError> = match encryptor_type {
                "hash" => {
                    let enc: Box<dyn Encryptor + Send> = Box::new(crate::hash_and_reverse::HashEncryptor);
                    enc.encrypt(input, "").map_err(|e| VeilError::Encryption(e.to_string()))
                }
                "reverse" => {
                    let enc: Box<dyn Encryptor + Send> = Box::new(crate::hash_and_reverse::ReverseEncryptor);
                    enc.encrypt(input, "").map_err(|e| VeilError::Encryption(e.to_string()))
                }
                "aes" => {
                    let key = vec![0u8; 32];
                    let iv = vec![0u8; 16];
                    let enc: Box<dyn Encryptor + Send> = Box::new(crate::aes::AesEncryptor::new(key, iv));
                    enc.encrypt(input, "").map_err(|e| VeilError::Encryption(e.to_string()))
                }
                "split" => {
                    let args: Vec<&str> = input.split('|').collect();
                    if args.len() != 3 {
                        Err(VeilError::Encryption("split command requires format: secret|n|k".into()))
                    } else {
                        let secret = args[0].as_bytes();
                        let n: usize = args[1].parse().unwrap_or(0);
                        let k: usize = args[2].parse().unwrap_or(0);
                        crate::secret_sharing::split_secret(secret, n, k)
                            .map(|shares| shares.into_iter().map(|s| hex::encode(s)).collect::<Vec<_>>().join(","))
                    }
                }
                "combine" => {
                    let shares: Vec<Vec<u8>> = input.split(',').map(|s| hex::decode(s).unwrap()).collect();
                    crate::secret_sharing::combine_secret_shares(&shares)
                        .map(|s| String::from_utf8_lossy(&s).to_string())
                }
                _ => Err(VeilError::Encryption(format!("Unknown encryptor type: {}", encryptor_type))),
            };

            match result {
                Ok(output) => {
                    if let Err(e) = socket.write_all(output.as_bytes()).await {
                        eprintln!("Failed to write to socket: {}", e);
                        return;
                    }
                    println!(
                        "Processed request: type='{}', input='{}', output='{}'",
                        encryptor_type, input, output
                    );
                }
                Err(e) => {
                    let msg = format!("Error: {:?}", e);
                    if let Err(err) = socket.write_all(msg.as_bytes()).await {
                        eprintln!("Failed to send error to client: {}", err);
                    }
                    eprintln!("Failed to process request: type='{}', input='{}', error='{:?}'", encryptor_type, input, e);
                }
            }
        });
    }
}

pub async fn start_client(addr: &str, full_input: &str) -> Result<(), VeilError> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Connect failed: {}", e)))?;

    stream.write_all(full_input.as_bytes())
        .await
        .map_err(|e| VeilError::Network(format!("Write failed: {}", e)))?;

    let mut buffer = vec![0u8; 4096];
    let n = stream.read(&mut buffer)
        .await
        .map_err(|e| VeilError::Network(format!("Read failed: {}", e)))?;

    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);

    Ok(())
}
