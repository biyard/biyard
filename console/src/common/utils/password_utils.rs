use sha3::{Digest, Sha3_256};

/// Deterministically hash a secret for lookup use cases such as API keys.
/// This is intentionally not used for storing user passwords.
pub fn hash_secret_for_lookup(secret: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(secret.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Verify a deterministically hashed lookup secret.
pub fn verify_secret_for_lookup(secret: &str, hash: &str) -> bool {
    hash_secret_for_lookup(secret) == hash
}
