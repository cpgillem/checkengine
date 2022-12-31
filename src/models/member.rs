use crate::{schema::*, auth::{self, AuthError}};
use chrono::{prelude::*, Duration};
use diesel::{Insertable, Queryable};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Member {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl Member {
    pub fn new(id: i32, username: &str, password_hash: &str) -> Member {
        Member {
            id,
            username: String::from(username),
            password_hash: String::from(password_hash),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }

    fn get_jwt_claims(&self) -> JwtClaims {
        let expiration = auth::get_jwt_exp_timestamp();
        JwtClaims { sub: self.username.clone(), exp: expiration as usize }
    }

    pub fn get_jwt(&self) -> Result<String, AuthError> {
        match encode(&Header::default(), &self.get_jwt_claims(), &EncodingKey::from_secret(&auth::get_jwt_secret())) {
            Ok(v) => Ok(v),
            Err(_) => Err(AuthError::JwtError),
        }
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
    pub fn from_input(input: &InputMember) -> Result<NewMember, AuthError> {
        let (hash, salt) = auth::hash_password(&input.password_raw)?;

        Ok(NewMember {
            username: input.username.clone(),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::auth;

    use super::{InputMember, NewMember, Member};

    #[test]
    fn test_from_input() {
        let input_member = InputMember {
            username: String::from("user"),
            password_raw: String::from("hunter2"),
        };

        let new_member = NewMember::from_input(&input_member).expect("could not convert to NewMember");

        assert!(auth::check_password("hunter2", &new_member.password_hash, &new_member.salt).unwrap());
    }

    #[test]
    fn test_get_jwt() {
        std::env::set_var("JWT_EXPIRATION", "3600");
        std::env::set_var("JWT_SECRET", "blahblahblah");
        let member = Member::new(0, "user", "hash");
        let jwt = member.get_jwt().unwrap();
    }
}