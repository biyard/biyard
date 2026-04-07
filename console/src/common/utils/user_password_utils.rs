use crate::common::{Error, Result};

#[cfg(feature = "server")]
use crate::features::accounts::AccountError;

pub fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| Error::InternalServerError(format!("password hash failed: {e}")))
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
    bcrypt::verify(password, password_hash)
        .map_err(|e| Error::InternalServerError(format!("password verify failed: {e}")))
}

pub fn is_bcrypt_hash(password_hash: &str) -> bool {
    password_hash.starts_with("$2a$")
        || password_hash.starts_with("$2b$")
        || password_hash.starts_with("$2x$")
        || password_hash.starts_with("$2y$")
}

pub fn enforce_password_policy(
    password: &str,
    email: Option<&str>,
    name: Option<&str>,
) -> Result<()> {
    let violations =
        crate::features::accounts::utils::validate_password_rules(password, email, name);

    if violations.is_empty() {
        Ok(())
    } else {
        Err(AccountError::WeakPassword(violations.join(" ")).into())
    }
}
