use std::num::NonZeroU32;

use ring::pbkdf2::{PBKDF2_HMAC_SHA512, self};
use ring::{digest};
use ring::rand::{self, SecureRandom, generate};

const SALT_LEN: usize = 16;
const HASH_LEN: usize = digest::SHA512_OUTPUT_LEN;
const ITERATIONS: u32 = 100_000;

pub type Hash = [u8; HASH_LEN];
pub type Salt = [u8; SALT_LEN];

// Hashes and salts a raw password.
pub fn hash_password(password_raw: &str) -> Result<(Hash, Salt), ring::error::Unspecified> {
    let salt = generate_salt()?;
    let password_bytes = password_raw.as_bytes();
    let mut hash: Hash = [0u8; HASH_LEN];
    pbkdf2::derive(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), &salt, password_bytes, &mut hash);
    Ok((hash, salt))
}

// Checks a raw password against a hashed password.
pub fn check_password(password_raw: &str, password_hash: &Hash, salt: &Salt) -> Result<bool, ring::error::Unspecified> {
    let password_bytes = password_raw.as_bytes();
    let result = pbkdf2::verify(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), salt, &password_bytes, password_hash);
    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn generate_salt() -> Result<[u8; SALT_LEN], ring::error::Unspecified> {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; SALT_LEN];
    rng.fill(&mut salt)?;
    Ok(salt)
}

#[cfg(test)]
mod tests {
    use super::{generate_salt, hash_password, check_password};

    #[test]
    fn test_generate_salt() {
        generate_salt().unwrap();
    }

    #[test]
    fn test_hash_password() {
        let raw = "hunter2";
        hash_password(&raw).unwrap();
    }

    #[test]
    fn test_check_password() {
        let raw = "hunter2";
        let (hash, salt) = hash_password(&raw).unwrap();
        let check = check_password("hunter2", &hash, &salt).unwrap();
        assert_eq!(check, true);
        let check_wrong = check_password("hunter3", &hash, &salt).unwrap();
        assert_eq!(check_wrong, false);
    }
}