use crate::schema::*;
use crate::models::posting::Posting;
use chrono::prelude::*;
use diesel::{Insertable, Queryable, Identifiable};
use serde::{Deserialize, Serialize};

use crate::models::member::Member;

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

/// Represents a posting group with all its child postings.
#[derive(Debug, Serialize)]
pub struct FullPostingGroup<'a> {
    pub id: i32,
    pub posted_at: &'a NaiveDateTime,
    pub check_number: &'a Option<String>,
    pub summary: &'a String,
    pub created_at: &'a NaiveDateTime,
    pub modified_at: &'a NaiveDateTime,
    pub postings: &'a Vec<Posting>,
}

impl<'a> FullPostingGroup<'a> {
    pub fn new(posting_group: &'a PostingGroup, postings: &'a Vec<Posting>) -> Self {
        FullPostingGroup {
            id: posting_group.id,
            posted_at: &posting_group.posted_at,
            check_number: &posting_group.check_number,
            summary: &posting_group.summary,
            created_at: &posting_group.created_at,
            modified_at: &posting_group.modified_at,
            postings: &postings,
        }
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
    pub fn from_input(input: &InputPostingGroup, member: &Member) -> Self {
        Self {
            posted_at: input.posted_at.clone(),
            check_number: input.check_number.clone(),
            summary: input.summary.clone(),
            created_at: Utc::now().naive_utc(),
            modified_at: Utc::now().naive_utc(),
            member_id: member.id,
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
