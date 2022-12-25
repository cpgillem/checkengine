use std::env;

use dotenvy::dotenv;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub fn create_connection() -> super::DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
    .build(manager)
    .expect("Could not create DB connection pool.")
}