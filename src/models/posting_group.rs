use crate::schema::*;
use chrono::prelude::*;
use diesel::{Insertable, Queryable, Identifiable, AsChangeset};
use serde::{Deserialize, Serialize};

use super::Resource;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name="posting_group"]
pub struct PostingGroup {
    pub id: i32,
    pub posted_at: NaiveDateTime,
    pub check_number: Option<String>,
    pub summary: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub member_id: i32,
}

impl Resource for PostingGroup {
    fn get_member_id(&self) -> i32 {
        self.member_id
    }
}

#[derive(Insertable, Debug)]
#[table_name = "posting_group"]
pub struct NewPostingGroup {
    pub posted_at: NaiveDateTime,
    pub check_number: Option<String>,
    pub summary: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub member_id: i32,
}

impl NewPostingGroup {
    pub fn from_input(input: &InputPostingGroup, member_id: i32) -> Self {
        Self {
            posted_at: input.posted_at.clone(),
            check_number: input.check_number.clone(),
            summary: input.summary.clone(),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
            member_id
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPostingGroup {
    pub posted_at: NaiveDateTime,
    pub check_number: Option<String>,
    pub summary: String,
    // pub postings: Vec<InputPosting>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "posting_group"]
pub struct UpdatePostingGroup {
    pub posted_at: Option<NaiveDateTime>,
    pub check_number: Option<String>,
    pub summary: Option<String>,
}