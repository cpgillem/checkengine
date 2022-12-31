use std::num::NonZeroU32;

use ring::pbkdf2::{PBKDF2_HMAC_SHA512, self};
use ring::{digest};
use ring::rand::{self, SecureRandom};

const SALT_LEN: usize = 16;
const HASH_LEN: usize = digest::SHA512_OUTPUT_LEN;
const ITERATIONS: u32 = 100_000;

pub type Hash = Vec<u8>;
pub type Salt = Vec<u8>;

#[derive(Debug)]
pub enum AuthError {
    CouldNotHash,
    CouldNotSalt,
    BadBase64String,
    Unspecified,
}

// Hashes and salts a raw password.
pub fn hash_password(password_raw: &str) -> Result<(String, String), AuthError> {
    let salt = match generate_salt() {
        Ok(s) => s,
        Err(_) => return Err(AuthError::CouldNotSalt),
    };

    let password_bytes = password_raw.as_bytes();
    let mut hash: Hash = vec![0u8; HASH_LEN];

    pbkdf2::derive(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), &salt, password_bytes, &mut hash);

    Ok((base64::encode(hash), base64::encode(salt)))
}

// Checks a raw password against a hashed password.
pub fn check_password(password_raw: &str, password_hash: &str, salt: &str) -> Result<bool, AuthError> {
    let password_bytes = password_raw.as_bytes();

    let decoded_password_hash = match base64::decode(password_hash) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::BadBase64String),
    };

    let decoded_salt = match base64::decode(salt) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::BadBase64String),
    };

    let result = pbkdf2::verify(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), &decoded_salt, &password_bytes, &decoded_password_hash);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn generate_salt() -> Result<Salt, AuthError> {
    let rng = rand::SystemRandom::new();
    let mut salt = vec![0u8; SALT_LEN];
    match rng.fill(&mut salt) {
        Ok(()) => Ok(salt),
        Err(_) => Err(AuthError::CouldNotSalt),
    }
}

#[cfg(test)]
mod tests {
    use super::{hash_password, check_password};

    #[test]
    fn test_check_password() {
        let (hash, salt) = hash_password("hunter2").unwrap();
        assert!(check_password("hunter2", &hash, &salt).unwrap());
        assert!(!check_password("hunter3", &hash, &salt).unwrap());
    }
}