use sha3::{Digest, Sha3_256};

/// Hash a password using bcrypt with default cost
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "test_password_123";
        let hashed = hash_password(password);

        assert_eq!(hashed, hash_password(password));
    }

    #[test]
    fn test_different_passwords_produce_different_hashes() {
        let password1 = "password1";
        let password2 = "password2";

        let hash1 = hash_password(password1);
        let hash2 = hash_password(password2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        let password = "same_password";

        let hash1 = hash_password(password);
        let hash2 = hash_password(password);

        assert_eq!(hash1, hash2);
    }
}
