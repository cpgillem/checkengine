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

pub trait Controller {
    fn get_pool(&self) -> &DbPool;

    fn get_connection(&self) -> Result<DbConnection, DataError> {
        Ok(self.get_pool().get().map_err(|_| DataError::Unspecified)?)
    }
}

/// Defines a resource controller with CRUD operations on a connection pool and with authorization.
pub trait ResourceController : Controller {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self;
    fn get_member_id(&self) -> i32;
}

pub trait GetResource<T> : ResourceController {
    fn get(&self, id: i32) -> Result<T, DataError>;
}

pub trait GetAllResource<T> : ResourceController {
    fn get_all(&self) -> Result<Vec<T>, DataError>;
}

pub trait GetChildren<Parent, Child> : ResourceController {
    fn get_children(&self, parent: &Parent) -> Result<Vec<Child>, DataError>;
}

pub trait GetParent<Parent, Child> : ResourceController {
    fn get_parent(&self, child: &Child) -> Result<Parent, DataError>;
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