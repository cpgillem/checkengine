use crate::{DbPool, models::{posting_group::{InputPostingGroup, PostingGroup, NewPostingGroup, UpdatePostingGroup}}, auth::JwtClaims, schema::posting_group};

use super::{DataError, ResourceController, GetResource, CreateResource, DeleteResource, UpdateResource, Controller};

use diesel::{prelude::*, dsl::now};

#[derive(Clone)]
pub struct PostingGroupController {
    pub pool: DbPool,
    pub jwt: JwtClaims,
}

impl Controller for PostingGroupController {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl ResourceController for PostingGroupController {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self {
        Self {
            pool: pool.clone(),
            jwt: jwt.clone(),
        }
    }

    fn get_member_id(&self) -> i32 {
        self.jwt.sub
    }
}

impl GetResource<PostingGroup> for PostingGroupController {
    fn get(&self, id: i32) -> Result<PostingGroup, DataError> {
        // Get the posting group.
        let posting_group = posting_group::table
            .filter(posting_group::id.eq(id))
            .first::<PostingGroup>(&mut self.get_connection()?)
            .map_err(|_| DataError::NotFound)?;

        // Return a different error if it is not owned.
        self.check_ownership(&posting_group)?;

        Ok(posting_group)
    }

    fn get_all(&self) -> Result<Vec<PostingGroup>, DataError> {
        posting_group::table
            .filter(posting_group::member_id.eq(self.get_member_id()))
            .load::<PostingGroup>(&mut self.get_connection()?)
            .map_err(|_| DataError::Unspecified)
    }
}

impl CreateResource<InputPostingGroup, PostingGroup> for PostingGroupController {
    fn create(&self, input: &InputPostingGroup) -> Result<PostingGroup, DataError> {
        diesel::insert_into(posting_group::table)
            .values(&NewPostingGroup::from_input(&input, self.get_member_id()))
            .get_result::<PostingGroup>(&mut self.get_connection()?)
            .map_err(|_| DataError::NotInserted)
    }
}

impl DeleteResource for PostingGroupController {
    fn delete(&self, id: i32) -> Result<usize, DataError> {
        // Check existence and ownership.
        self.get(id)?;

        // Delete record.
        diesel::delete(
                posting_group::table.find(id)
            )
            .execute(&mut self.get_connection()?)
            .map_err(|_| DataError::NotDeleted)
    }
}

impl UpdateResource<UpdatePostingGroup, PostingGroup> for PostingGroupController {
    fn update(&self, id: i32, input: &UpdatePostingGroup) -> Result<PostingGroup, DataError> {
        let mut connection = self.get_connection()?;

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