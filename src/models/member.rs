use crate::schema::*;
use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Member {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "member"]
pub struct NewMember<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl NewMember<'_> {
    pub fn from_input(input: &InputMember) -> NewMember {

        NewMember {
            username: &input.username,
            password_hash: &input.password_raw,
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMember {
    pub username: String,
    pub password_raw: String,
}