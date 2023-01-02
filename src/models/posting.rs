use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Posting {
    pub id: i32,
    pub posted_at: chrono::NaiveDateTime,

    /// Blank string if no check number provided.
    pub check_number: String,

    pub summary: String,

    /// Account where money comes from.
    pub from_register_id: i32,

    /// Account money goes into.
    pub to_register_id: i32,

    /// Amount in cents.
    pub amount: i64,

    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "posting" ]
pub struct NewPosting {
    pub posted_at: chrono::NaiveDateTime,
    pub check_number: String,
    pub summary: String,
    pub from_register_id: i32,
    pub to_register_id: i32,
    pub amount: i64,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl NewPosting {
    pub fn from_input(input: &InputPosting) -> Self {
        Self {
            posted_at: input.posted_at.clone(),
            check_number: input.check_number.clone(),
            summary: input.summary.clone(),
            from_register_id: input.from_register_id,
            to_register_id: input.to_register_id,
            amount: input.amount,
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "posting"]
pub struct InputPosting {
    pub posted_at: chrono::NaiveDateTime,
    pub check_number: String,
    pub summary: String,
    pub from_register_id: i32,
    pub to_register_id: i32,
    pub amount: i64,
}