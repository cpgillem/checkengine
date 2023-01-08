use diesel::prelude::*;

use crate::{models::{posting::{Posting, InputPosting, UpdatePosting, NewPosting}, posting_group::PostingGroup}, DbPool, auth::JwtClaims, schema::{posting, posting_group}};

use super::{DataError, ResourceController, GetResource, Controller, UpdateResource, GetParent, DeleteResource, CreateResource};

#[derive(Clone)]
pub struct PostingController {
    pub pool: DbPool,
    pub jwt: JwtClaims,
}

impl GetParent<PostingGroup, Posting> for PostingController {
    fn get_parent(&self, posting: &Posting) -> Result<PostingGroup, DataError> {
        // Get parent group.
        posting_group::table
            .find(&posting.posting_group_id)
            .get_result::<PostingGroup>(&mut self.get_connection()?)
            .map_err(|_| DataError::NotFound)
    }   
}

impl Controller for PostingController {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl ResourceController for PostingController {
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

impl GetResource<Posting> for PostingController {
    
    fn get(&self, id: i32) -> Result<Posting, DataError> {
        // Get posting.
        let posting = posting::table
            .find(id)
            .get_result::<Posting>(&mut self.get_connection()?)
            .map_err(|_| DataError::NotFound)?;

        let posting_group = self.get_parent(&posting)?;

        // Check ownership.
        if posting_group.member_id != self.jwt.sub {
            return Err(DataError::NotOwned);
        }

        Ok(posting)
    }
}

impl UpdateResource<UpdatePosting, Posting> for PostingController {
    fn update(&self, id: i32, input: &UpdatePosting) -> Result<Posting, DataError> {
        let mut connection = self.get_connection()?;

        // Check ownership.
        self.get(id)?;

        // Update record.
        let updated_posting = diesel::update(
                posting::table.find(id)
            )
            .set(input)
            .get_result::<Posting>(&mut connection)
            .map_err(|_| DataError::NotUpdated)?;

        Ok(updated_posting)
    }
}

impl DeleteResource for PostingController {
    fn delete(&self, id: i32) -> Result<usize, DataError> {
        // Check ownership.
        self.get(id)?;

        // Delete.
        diesel::delete(
                posting::table.find(id)
            )
            .execute(&mut self.get_connection()?)
            .map_err(|_| DataError::NotDeleted)
    }
}

impl CreateResource<InputPosting, Posting> for PostingController {
    fn create(&self, input: &InputPosting) -> Result<Posting, DataError> {
        diesel::insert_into(posting::table)
            .values(&NewPosting::from_input(&input))
            .get_result::<Posting>(&mut self.get_connection()?)
            .map_err(|_| DataError::NotInserted)
    }
}