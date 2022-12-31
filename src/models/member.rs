use std::error::Error;

use crate::{schema::*, auth::{self, Hash, Salt, AuthError}};
use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use base64;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Member {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
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

#[cfg(test)]
mod tests {
    use crate::auth;

    use super::{InputMember, NewMember};

    #[test]
    fn test_from_input() {
        let input_member = InputMember {
            username: String::from("user"),
            password_raw: String::from("hunter2"),
        };

        let new_member = NewMember::from_input(&input_member).expect("could not convert to NewMember");

        assert!(auth::check_password("hunter2", &new_member.password_hash, &new_member.salt).unwrap());
    }
}