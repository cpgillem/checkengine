use crate::{
    auth::Signup,
    models::member::{Member, NewMember},
    schema::member,
    DbPool,
};

use super::{DataError, Controller};

use diesel::RunQueryDsl;
use diesel::prelude::*;

/// Stores state information and provides access to members (users).
#[derive(Clone)]
pub struct MemberController {
    pub pool: DbPool,
}

impl Controller for MemberController {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl MemberController {
    /// Creates a new member in the database from a signup.
    pub fn create(&self, signup: &Signup) -> Result<Member, DataError> {
        let mut connection = self.get_connection()?;
        let new_member = NewMember::from_signup(signup).map_err(|e| DataError::Auth(e))?;
        diesel::insert_into(member::table)
            .values(&new_member)
            .get_result::<Member>(&mut connection)
            .map_err(|_| DataError::NotInserted)
    }

    /// Retrieves one member by username.
    pub fn get(&self, username: &str) -> Result<Member, DataError> {
        let mut connection = self.get_connection()?;
        member::table
            .filter(member::columns::username.eq(&username))
            .get_result::<Member>(&mut connection)
            .map_err(|_| DataError::NotFound)
    }
}
