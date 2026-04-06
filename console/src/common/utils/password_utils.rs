use sha3::{Digest, Sha3_256};

/// Hash a password/key using SHA3-256 (deterministic)
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Verify a password by comparing SHA3-256 hashes
pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
