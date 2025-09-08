use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::hash_and_reverse::Encryptor;
use crate::VeilError;
use crate::mpc;
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

            let command_type = parts[0];
            let input = parts[1];

            let response_bytes: Vec<u8> = match command_type {
                "split" => {
                    let args: Vec<&str> = input.split('|').collect();
                    if args.len() != 3 {
                        "Invalid split command".to_string().into_bytes()
                    } else {
                        let secret = args[0];
                        let threshold: usize = args[1].parse().unwrap_or(0);
                        let total: usize = args[2].parse().unwrap_or(0);
                        match mpc::split_command(secret, threshold, total) {
                            Ok(shares) => shares.join(",").into_bytes(),
                            Err(e) => format!("Error: {}", e).into_bytes(),
                        }
                    }
                }
                "combine" => {
                    let hex_shares: Vec<&str> = input.split(',').collect();
                    match mpc::combine_command(hex_shares) {
                        Ok(secret) => secret.into_bytes(),
                        Err(e) => format!("Error: {}", e).into_bytes(),
                    }
                }
                "mpc" => {
                    let parties: Vec<Vec<Vec<u8>>> = input
                        .split('|')
                        .map(|p| {
                            p.split(',')
                                .map(|s| hex::decode(s).unwrap_or_default())
                                .collect()
                        })
                        .collect();

                    match mpc::aggregate_secrets(parties) {
                        Ok(agg) => hex::encode(agg).into_bytes(),
                        Err(e) => format!("Encryption error: {}", e).into_bytes(),
                    }
                }
                "aes" | "reverse" | "hash" => {
                    // Generate random salt
                    let mut rng = StdRng::from_entropy();
                    let mut salt_bytes = [0u8; 16];
                    rng.fill_bytes(&mut salt_bytes);
                    let salt = hex::encode(salt_bytes);

                    let encryptor: Box<dyn Encryptor + Send> = match command_type {
                        "hash" => Box::new(crate::hash_and_reverse::HashEncryptor),
                        "reverse" => Box::new(crate::hash_and_reverse::ReverseEncryptor),
                        "aes" => {
                            let key = vec![0u8; 32];
                            let iv = vec![0u8; 16];
                            Box::new(crate::aes::AesEncryptor::new(key, iv))
                        }
                        _ => {
                            eprintln!("Unknown encryptor type: {}", command_type);
                            return;
                        }
                    };

                    match encryptor.encrypt(input, &salt) {
                        Ok(enc) => enc.into_bytes(),
                        Err(e) => format!("Encryption error: {}", e).into_bytes(),
                    }
                }
                _ => format!("Unknown command: {}", command_type).into_bytes(),
            };

            if let Err(e) = socket.write_all(&response_bytes).await {
                eprintln!("Failed to write to socket: {}", e);
            } else {
                println!(
                    "Processed request: type='{}', input='{}', output='{}'",
                    command_type,
                    input,
                    String::from_utf8_lossy(&response_bytes)
                );
            }
        });
    }
}

pub async fn start_client(addr: &str, command: &str) -> Result<String, VeilError> {
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .map_err(|e| VeilError::Network(format!("Failed to connect: {}", e)))?;

    stream.write_all(command.as_bytes())
        .await
        .map_err(|e| VeilError::Network(format!("Failed to write: {}", e)))?;

    let mut buffer = vec![0u8; 4096];
    let n = stream.read(&mut buffer)
        .await
        .map_err(|e| VeilError::Network(format!("Failed to read: {}", e)))?;

    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}