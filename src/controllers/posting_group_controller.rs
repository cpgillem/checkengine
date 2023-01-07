use crate::{DbPool, models::posting_group::{InputPostingGroup, PostingGroup, NewPostingGroup, UpdatePostingGroup}, auth::JwtClaims, schema::posting_group};

use super::{get_connection, DataError, Resource};

use diesel::{prelude::*, dsl::now};

#[derive(Clone)]
pub struct PostingGroupController {
    pub pool: DbPool,
    pub jwt: JwtClaims,
}

impl Resource for PostingGroupController {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self {
        Self {
            pool: pool.clone(),
            jwt: jwt.clone(),
        }
    }
}

impl PostingGroupController {
    pub fn create(&self, input: &InputPostingGroup) -> Result<PostingGroup, DataError> {
        diesel::insert_into(posting_group::table)
            .values(&NewPostingGroup::from_input(&input, self.jwt.sub))
            .get_result::<PostingGroup>(&mut get_connection(&self.pool)?)
            .map_err(|_| DataError::NotInserted)
    }

    pub fn get_all(&self) -> Result<Vec<PostingGroup>, DataError> {
        posting_group::table
            .filter(posting_group::member_id.eq(self.jwt.sub))
            .load::<PostingGroup>(&mut get_connection(&self.pool)?)
            .map_err(|_| DataError::Unspecified)
    }

    pub fn get(&self, id: i32) -> Result<PostingGroup, DataError> {
        // Get the posting group.
        let posting_group = posting_group::table
            .filter(posting_group::id.eq(id))
            .first::<PostingGroup>(&mut get_connection(&self.pool)?)
            .map_err(|_| DataError::NotFound)?;

        // Return a different error if it is not owned.
        if posting_group.member_id != self.jwt.sub {
            return Err(DataError::NotOwned);
        }

        Ok(posting_group)
    }

    pub fn delete(&self, id: i32) -> Result<usize, DataError> {
        // Check existence and ownership.
        self.get(id)?;

        // Delete record.
        diesel::delete(
                posting_group::table.find(id)
            )
            .execute(&mut get_connection(&self.pool)?)
            .map_err(|_| DataError::NotDeleted)
    }

    pub fn update(&self, id: i32, input: &UpdatePostingGroup) -> Result<PostingGroup, DataError> {
        let mut connection = get_connection(&self.pool)?;

        self.get(id)?;

        // Update the record.
        let updated_posting_group = diesel::update(
                posting_group::table.find(id)
            )
            .set(input)
            .get_result::<PostingGroup>(&mut connection)
            .map_err(|_| DataError::NotUpdated)?;

        // Update metadata.
        diesel::update(posting_group::table.find(id))
            .set(posting_group::modified_at.eq(now))
            .execute(&mut connection)
            .map_err(|_| DataError::Unspecified)?;

        Ok(updated_posting_group)
    }
}