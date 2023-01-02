use std::num::NonZeroU32;

use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use ring::pbkdf2::{PBKDF2_HMAC_SHA512, self};
use ring::{digest};
use ring::rand::{self, SecureRandom};

use actix_web::{error, HttpResponse};
use derive_more::{Error, Display};
use serde::{Deserialize, Serialize};

use crate::DbPool;
use crate::models::member::Member;

const SALT_LEN: usize = 16;
const HASH_LEN: usize = digest::SHA512_OUTPUT_LEN;
const ITERATIONS: u32 = 100_000;
const HEADER_PREFIX: &str = "Bearer ";

pub type Hash = Vec<u8>;
pub type Salt = Vec<u8>;

#[derive(Debug, Error, Display)]
pub enum AuthError {
    PasswordMismatch,
    UserNotFound,
    CouldNotHash,
    CouldNotSalt,
    BadBase64String,
    JwtError,
    InvalidJwt,
    BadRequest,
    Unspecified,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtClaims {
    pub sub: i32,
    pub exp: usize,
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).insert_header(ContentType::html()).body("fail")
    }
}

// Hashes and salts a raw password.
pub fn hash_password(password_raw: &str) -> Result<(String, String), AuthError> {
    let salt = generate_salt().map_err(|_| AuthError::CouldNotSalt)?;

    let password_bytes = password_raw.as_bytes();
    let mut hash: Hash = vec![0u8; HASH_LEN];

    pbkdf2::derive(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), &salt, password_bytes, &mut hash);

    Ok((base64::encode(hash), base64::encode(salt)))
}

// Checks a raw password against a hashed password.
pub fn check_password(password_raw: &str, password_hash: &str, salt: &str) -> Result<(), AuthError> {
    let password_bytes = password_raw.as_bytes();

    let decoded_password_hash = base64::decode(password_hash).map_err(|_| AuthError::BadBase64String)?;

    let decoded_salt = base64::decode(salt).map_err(|_| AuthError::BadBase64String)?;

    pbkdf2::verify(PBKDF2_HMAC_SHA512, NonZeroU32::new(ITERATIONS).unwrap(), &decoded_salt, &password_bytes, &decoded_password_hash).map_err(|e| AuthError::PasswordMismatch)
}

pub fn get_jwt_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}

pub fn get_jwt_exp_timestamp() -> i64 {
    // This type of operation should only panic if .env is wrong, and that's fine
    let seconds = std::env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION not set")
        .parse::<i64>()
        .expect("invalid JWT_EXPIRATION"); 
    Utc::now()
        .checked_add_signed(Duration::seconds(seconds))
        .expect("invalid JWT_EXPIRATION")
        .timestamp()
}

/// Validates a JWT or returns an error. Returns a claims object with the user ID if successful.
pub fn validate_jwt(jwt: &str) -> Result<JwtClaims, AuthError> {
    // Get the secret from the environment.
    let secret = get_jwt_secret();
    let decoding_key = &DecodingKey::from_secret(&secret);

    // Validate JWT or return an error.
    let message = decode::<JwtClaims>(jwt, decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|_| AuthError::InvalidJwt)?;

    Ok(message.claims)
}

/// Extracts the JWT claims from a header.
pub fn validate_jwt_from_header(request: &actix_web::HttpRequest) -> Result<JwtClaims, AuthError> {
    let jwt_str = extract_jwt(&request)?;
    let claims = validate_jwt(&jwt_str)?;
    Ok(claims)
}

fn extract_jwt(request: &actix_web::HttpRequest) -> Result<String, AuthError> {
    // Extract the header.
    let auth_header = match request.headers().get("Authorization") {
        Some(v) => v,
        None => return Err(AuthError::BadRequest),
    };
    let header_str = auth_header.to_str().map_err(|_| AuthError::BadRequest)?;

    extract_jwt_from_header_str(header_str)
}

fn extract_jwt_from_header_str(header_str: &str) -> Result<String, AuthError> {
    // Make sure the header is formatted correctly.
    if ! str::starts_with(&header_str, HEADER_PREFIX) {
        return Err(AuthError::BadRequest);
    }

    // Extract the encoded token.
    Ok(String::from(&header_str[HEADER_PREFIX.len()..]))
}

fn generate_salt() -> Result<Salt, AuthError> {
    let rng = rand::SystemRandom::new();
    let mut salt = vec![0u8; SALT_LEN];
    rng.fill(&mut salt).map_err(|_| AuthError::CouldNotSalt)?;
    Ok(salt)
}

#[cfg(test)]
mod tests {
    use super::{hash_password, check_password, extract_jwt_from_header_str};

    #[test]
    fn test_check_password() {
        let (hash, salt) = hash_password("hunter2").unwrap();
        assert!(check_password("hunter2", &hash, &salt).is_ok());
        assert!(check_password("hunter3", &hash, &salt).is_err());
    }

    #[test]
    fn test_extract_jwt_from_header_str() {
        let header_str = String::from("Bearer kdsjfldsjkflsjdlajsdf");
        let token = extract_jwt_from_header_str(&header_str);
        assert_eq!(String::from("kdsjfldsjkflsjdlajsdf"), token.unwrap());

        let header_str = String::from("blah");
        let result = extract_jwt_from_header_str(&header_str);
        assert!(result.is_err());
    }
}