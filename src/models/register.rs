use crate::schema::*;
use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Register {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "register"]
pub struct NewRegister<'a> {
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl NewRegister<'_> {
    pub fn from_input(input: &InputRegister) -> NewRegister {
        NewRegister {
            title: &input.title,
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRegister {
    pub title: String,
}
