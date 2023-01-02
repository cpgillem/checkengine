use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::member::Member;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[belongs_to(Member)]
#[table_name = "register"]
pub struct Register {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
    pub parent_id: Option<i32>,
    pub member_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "register"]
pub struct NewRegister<'a> {
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
    pub parent_id: Option<i32>,
    pub member_id: i32,
}

impl NewRegister<'_> {
    pub fn from_input(input: &InputRegister) -> NewRegister {
        NewRegister {
            title: &input.title,
            parent_id: input.parent_id,
            member_id: input.member_id,
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
    pub member_id: i32,
}
