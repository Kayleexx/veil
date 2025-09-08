pub mod hash_and_reverse; 
pub mod aes;              
pub mod network;
pub use hash_and_reverse::*;
pub use aes::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::{start_server, start_client}; // <-- add this
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_server_client_hash() {
        let addr = "127.0.0.1:7879"; 
        tokio::spawn(async move { start_server(addr).await.unwrap() });
        sleep(Duration::from_millis(100)).await;
        let result = tokio::spawn(async move { start_client(addr, "hash", "testmessage").await.unwrap() }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_server_client_reverse() {
        let addr = "127.0.0.1:7880";
        tokio::spawn(async move { start_server(addr).await.unwrap() });
        sleep(Duration::from_millis(100)).await;
        let result = tokio::spawn(async move { start_client(addr, "reverse", "abc123").await.unwrap() }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_server_client_aes() {
        let addr = "127.0.0.1:7881";
        tokio::spawn(async move { start_server(addr).await.unwrap() });
        sleep(Duration::from_millis(100)).await;
        let result = tokio::spawn(async move { start_client(addr, "aes", "securetext").await.unwrap() }).await;
        assert!(result.is_ok());
    }
}
