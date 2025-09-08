use crate::VeilError;
use crate::secret_sharing::{split_secret, combine_secret_shares};

pub fn split_command(secret: &str, threshold: usize, total: usize) -> Result<Vec<String>, VeilError> {
    let shares = split_secret(secret.as_bytes(), threshold, total)?;
    Ok(shares.iter().map(|s| hex::encode(s)).collect())
}

pub fn combine_command(hex_shares: Vec<&str>) -> Result<String, VeilError> {
    let shares: Result<Vec<Vec<u8>>, _> = hex_shares
        .iter()
        .map(|h| hex::decode(h).map_err(|e| VeilError::MPC(e.to_string())))
        .collect();
    let secret = combine_secret_shares(&shares?)?;
    Ok(String::from_utf8(secret)?)
}

/// Aggregate secrets from multiple parties (for MPC)
pub fn aggregate_secrets(shares: Vec<Vec<Vec<u8>>>) -> Result<Vec<u8>, VeilError> {
    if shares.is_empty() {
        return Err(VeilError::MPC("No secrets provided".into()));
    }

    let mut aggregated = Vec::new();
    for party_shares in shares {
        let secret = combine_secret_shares(&party_shares)?;
        aggregated.push(secret);
    }

    let mut result = aggregated[0].clone();
    for secret in aggregated.iter().skip(1) {
        for (i, byte) in secret.iter().enumerate() {
            if i < result.len() {
                result[i] ^= byte;
            } else {
                result.push(*byte);
            }
        }
    }

    Ok(result)
}
