use derive_more::Display;

use crate::{auth::{self, JwtClaims}, DbConnection, DbPool};

pub mod member_controller;
pub mod register_controller;
pub mod posting_group_controller;
pub mod posting_controller;

/// Returned from controllers, in order to separate from using diesel's errors.
#[derive(Debug, Display)]
pub enum DataError {
    NotFound,
    NotInserted,
    NotDeleted,
    NotUpdated,
    Auth(auth::AuthError),
    NotOwned,
    Unspecified,
}

/// Wraps the function for creating a DB connection in a result with a data error.
pub fn get_connection(pool: &DbPool) -> Result<DbConnection, DataError> {
    Ok(pool.get().map_err(|_| DataError::Unspecified)?)
}

/// Defines a resource controller with CRUD operations on a connection pool and with authorization.
pub trait Resource {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self;
}