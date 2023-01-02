use crate::schema::*;
use chrono::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PostingGroup {
    pub id: i32,
    pub posted_at: NaiveDateTime,
    pub check_number: String,
    pub summary: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "posting_group"]
pub struct NewPostingGroup {
    pub posted_at: NaiveDateTime,
    pub check_number: String,
    pub summary: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl NewPostingGroup {
    pub fn from_input(input: &InputPostingGroup) -> Self {
        Self {
            posted_at: input.posted_at.clone(),
            check_number: input.check_number.clone(),
            summary: input.summary.clone(),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name="posting_group"]
pub struct InputPostingGroup {
    pub posted_at: NaiveDateTime,
    pub check_number: String,
    pub summary: String,
}
