use crate::schema::*;
use chrono::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Register {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
    pub parent_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "register"]
pub struct NewRegister<'a> {
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
    pub parent_id: Option<i32>,
}

impl NewRegister<'_> {
    pub fn from_input(input: &InputRegister) -> NewRegister {
        NewRegister {
            title: &input.title,
            parent_id: input.parent_id,
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name="register"]
pub struct InputRegister {
    pub title: String,
    pub parent_id: Option<i32>,
}
