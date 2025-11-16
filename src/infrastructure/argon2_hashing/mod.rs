use anyhow::Result;
use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let bytes_password = password.as_bytes();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(bytes_password, &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e.to_string()))?;

    Ok(password_hash.to_string())
}

pub fn verify(password: String, hashed_password: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hashed_password)
        .map_err(|e| anyhow::anyhow!("Failed to parse hash: {}", e.to_string()))?;

    let bytes_password = password.as_bytes();

    let argon2 = Argon2::default();
    let result = argon2
        .verify_password(bytes_password, &parsed_hash).is_ok();

    Ok(result)
}
