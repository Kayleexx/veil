use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::hash_and_reverse::CryptoService;

pub async fn start_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            let received = String::from_utf8_lossy(&buffer[..n]);
            println!("Received: {}", received);
        });
    }
}

pub async fn start_client(addr: &str, message: &str, crypto: &CryptoService) -> tokio::io::Result<()> {
    let mut stream = TcpStream::connect(addr).await?;
    let encrypted = crypto.encrypt(message, "salt").unwrap();
    stream.write_all(encrypted.as_bytes()).await?;
    println!("Sent encrypted message: {}", encrypted);
    Ok(())
}
