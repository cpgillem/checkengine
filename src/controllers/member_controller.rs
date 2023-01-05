use crate::{
    auth::Signup,
    models::member::{Member, NewMember},
    schema::member,
    DbPool,
};

use super::{get_connection, DataError};

use diesel::RunQueryDsl;
use diesel::prelude::*;

/// Stores state information and provides access to members (users).
#[derive(Clone)]
pub struct MemberController {
    pub pool: DbPool,
}

impl MemberController {
    /// Creates a new member in the database from a signup.
    pub fn create(&self, signup: &Signup) -> Result<Member, DataError> {
        let mut connection = get_connection(&self.pool)?;
        let new_member = NewMember::from_signup(signup).map_err(|e| DataError::Auth(e))?;
        diesel::insert_into(member::table)
            .values(&new_member)
            .get_result::<Member>(&mut connection)
            .map_err(|_| DataError::NotInserted)
    }

    /// Retrieves one member by username.
    pub fn get(&self, username: &str) -> Result<Member, DataError> {
        let mut connection = get_connection(&self.pool)?;
        member::table
            .filter(member::columns::username.eq(&username))
            .get_result::<Member>(&mut connection)
            .map_err(|_| DataError::NotFound)
    }
}
