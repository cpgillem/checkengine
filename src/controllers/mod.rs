use derive_more::Display;

use crate::{auth, DbConnection, DbPool};

pub mod member_controller;
pub mod register_controller;
pub mod posting_group_controller;

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

// Wraps the function for creating a DB connection in a result with a data error.
pub fn get_connection(pool: &DbPool) -> Result<DbConnection, DataError> {
    Ok(pool.get().map_err(|_| DataError::Unspecified)?)
}