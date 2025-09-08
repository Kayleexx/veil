use tokio::task;
use veil::network::{start_server, start_client};

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:7878";

    // Spawn the server in a background task
    task::spawn(async move {
        start_server(server_addr).await.unwrap();
    });

    // Give server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Send messages with different encryptor types
    start_client(server_addr, "hash", "Hello Hash!").await.unwrap();
    start_client(server_addr, "reverse", "Hello Reverse!").await.unwrap();

    let key = vec![0u8; 32];  // Example key
    let iv = vec![0u8; 16];   // Example IV
    start_client(server_addr, "aes", "Hello AES!").await.unwrap();
}
