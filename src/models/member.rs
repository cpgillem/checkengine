use crate::{auth::{self, AuthError, JwtClaims}, DbConnection, DbPool};
use chrono::prelude::*;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::schema::member;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Member {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub salt: String,
}

impl Member {
    pub fn new(id: i32, username: &str, password_hash: &str, salt: &str) -> Self {
        Self {
            id,
            username: String::from(username),
            password_hash: String::from(password_hash),
            salt: String::from(salt),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }

    /// Queries the database for a member with the ID in the JWT sub claim.
    pub fn from_jwt_claims(claims: &JwtClaims, pool: &DbPool) -> Result<Self, AuthError> {
        let mut connection = pool.get().map_err(|_| AuthError::Unspecified)?;
        Ok(member::table
            .find(claims.sub)
            .first::<Member>(&mut connection)
            .map_err(|_| AuthError::UserNotFound)?)
    }

    /// Extracts the actual member from the HTTP header.
    pub fn from_header(request: &actix_web::HttpRequest, pool: &DbPool) -> Result<Self, AuthError> {
        let claims = auth::validate_jwt_from_header(request)?;
        Ok(Member::from_jwt_claims(&claims, &pool)?)
    }

    fn get_jwt_claims(&self) -> JwtClaims {
        let expiration = auth::get_jwt_exp_timestamp();
        JwtClaims { sub: self.id, exp: expiration as usize }
    }

    pub fn get_jwt(&self) -> Result<String, AuthError> {
        Ok(
            encode(&Header::default(), &self.get_jwt_claims(), &EncodingKey::from_secret(&auth::get_jwt_secret()))
                .map_err(|_| AuthError::JwtError)?
        )
    }
}

#[derive(Insertable, Debug)]
#[table_name = "member"]
pub struct NewMember {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl NewMember{
    pub fn from_signup(signup: &Signup) -> Result<NewMember, AuthError> {
        let (hash, salt) = auth::hash_password(&signup.password_raw)?;

        Ok(NewMember {
            username: signup.username.clone(),
            password_hash: hash.clone(),
            salt: salt.clone(),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMember {
    pub username: String,
    pub password_raw: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Login {
    pub username: String,
    pub password_raw: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Signup {
    pub username: String, 
    pub password_raw: String,
}

#[cfg(test)]
mod tests {
    use crate::{auth, models::member::Signup};

    use super::{NewMember, Member};

    #[test]
    fn test_from_input() {
        let input_member = Signup {
            username: String::from("user"),
            password_raw: String::from("hunter2"),
        };

        let new_member = NewMember::from_signup(&input_member).expect("could not convert to NewMember");

        assert!(auth::check_password("hunter2", &new_member.password_hash, &new_member.salt).is_ok());
    }

    #[test]
    fn test_get_jwt() {
        std::env::set_var("JWT_EXPIRATION", "3600");
        std::env::set_var("JWT_SECRET", "blahblahblah");
        let member = Member::new(0, "user", "hash", "salt");
        member.get_jwt().unwrap();
    }
}