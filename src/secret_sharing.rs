use p256::{Scalar, elliptic_curve::PrimeField};
use rand_core::OsRng;
use vsss_rs_std::{shamir, Share};
use crate::VeilError;

const SCALAR_SIZE: usize = 32;

/// Split a secret into `n` shares with threshold `k`.
pub fn split_secret(secret: &[u8], n: usize, k: usize) -> Result<Vec<Vec<u8>>, VeilError> {
    if secret.len() > SCALAR_SIZE {
        return Err(VeilError::Encryption("Secret too long; max 32 bytes".to_string()));
    }

    let mut secret_bytes = [0u8; SCALAR_SIZE];
    secret_bytes[SCALAR_SIZE - secret.len()..].copy_from_slice(secret);
    let secret_scalar_opt: Option<Scalar> = Scalar::from_repr(secret_bytes.into()).into();
    let secret_scalar = secret_scalar_opt
        .ok_or_else(|| VeilError::Encryption("Invalid scalar representation".into()))?;

    let mut rng = OsRng;
    let shares = shamir::split_secret(k, n, secret_scalar, &mut rng)
        .map_err(|e| VeilError::Encryption(e.to_string()))?;

    let mut byte_shares = Vec::with_capacity(shares.len());
    for share in shares {
        let mut bytes = vec![0u8; 1 + SCALAR_SIZE];
        bytes[0] = share.identifier();
        bytes[1..].copy_from_slice(share.value());
        byte_shares.push(bytes);
    }

    Ok(byte_shares)
}

/// Combine byte shares to reconstruct the secret.
pub fn combine_secret_shares(shares: &[Vec<u8>]) -> Result<Vec<u8>, VeilError> {
    if shares.len() < 2 {
        return Err(VeilError::Encryption("Need at least 2 shares".to_string()));
    }

    let mut scalar_shares = Vec::with_capacity(shares.len());
    for share_bytes in shares {
        if share_bytes.len() != 1 + SCALAR_SIZE {
            return Err(VeilError::Encryption(format!(
                "Invalid share size: expected {} bytes",
                1 + SCALAR_SIZE
            )));
        }

        let index = share_bytes[0];
        let mut val_bytes = [0u8; SCALAR_SIZE];
        val_bytes.copy_from_slice(&share_bytes[1..]);

        let scalar_opt: Option<Scalar> = Scalar::from_repr(val_bytes.into()).into();
        let scalar = scalar_opt
            .ok_or_else(|| VeilError::Encryption("Invalid scalar in share".into()))?;

        let share = Share::from_field_element(index, scalar)
            .map_err(|e| VeilError::Encryption(e.to_string()))?;
        scalar_shares.push(share);
    }

    let secret_scalar = shamir::combine_shares::<Scalar>(&scalar_shares)
        .map_err(|e| VeilError::Encryption(e.to_string()))?;

    let mut secret = secret_scalar.to_repr().to_vec();
    // strip leading zero padding we added when splitting
    while secret.starts_with(&[0]) && secret.len() > 1 {
        secret.remove(0);
    }

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_combine() {
        let secret = b"hello veil";
        let n = 5;
        let k = 3;

        let shares = split_secret(secret, n, k).expect("Split should work");
        assert_eq!(shares.len(), n);

        let reconstructed = combine_secret_shares(&shares[0..k]).expect("Combine should work");
        assert_eq!(reconstructed, secret);
    }
}
