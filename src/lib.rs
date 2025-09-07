pub mod crypto;
pub mod network;
pub mod utils;


#[cfg(test)]

mod tests {
    #[test]
    fn modules_Load() {
        super::crypto::placeholder();
        super::network::placeholder();
        super::utils::placeholder();
    }
}