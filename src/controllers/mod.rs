use derive_more::Display;
use diesel::AsChangeset;

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

pub trait GetResource<T> {
    fn get(&self, id: i32) -> Result<T, DataError>;
    fn get_all(&self) -> Result<Vec<T>, DataError>;
}

pub trait DeleteResource {
    fn delete(&self, id: i32) -> Result<usize, DataError>;
}

/// A controller that can create a resource using input type T and returns output type U.
pub trait CreateResource<T, U> {
    fn create(&self, input: &T) -> Result<U, DataError>;
}

/// Uses input type T and returns output type U.
pub trait UpdateResource<T: AsChangeset, U> {
    fn update(&self, id: i32, input: &T) -> Result<U, DataError>;
}