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
}