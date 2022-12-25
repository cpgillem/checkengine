use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Register {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name="register"]
pub struct NewRegister<'a> {
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}