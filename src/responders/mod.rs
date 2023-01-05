use actix_web::{Error, error, HttpRequest};

use crate::{DbPool, DbConnection, models::member::Member};

// Resource routes for registers (accounts).
pub mod register;

/// Provides responders for treating users like a resource, mostly
/// for admin activity.
pub mod member;

/// Provides the common routes for login/signup/reset.
pub mod auth;

pub mod posting_group;

// Wraps the function for creating a DB connection in a result with an actix error.
pub fn get_connection(pool: &DbPool) -> Result<DbConnection, Error> {
    Ok(pool.get().map_err(|e| error::ErrorInternalServerError(e))?)
}

// Wraps the function for retrieving a member from the header in a result with an actix error.
pub fn get_member(request: &HttpRequest, pool: &DbPool) -> Result<Member, Error> {
    Ok(Member::from_header(request, pool).map_err(|e| error::ErrorUnauthorized(e))?)
}