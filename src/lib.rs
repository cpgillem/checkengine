use diesel::{PgConnection, r2d2::ConnectionManager};


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub mod util;
pub mod responders;
pub mod models;
pub mod schema;
pub mod builders;
pub mod db_access;
pub mod auth;
