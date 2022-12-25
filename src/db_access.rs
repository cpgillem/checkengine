use actix_web::web;
use diesel::prelude::*;

use crate::DbPool;
use crate::models;
use crate::schema::register::dsl::*;

pub fn get_all_registers(pool: web::Data<DbPool>) -> Result<Vec<models::Register>, diesel::result::Error> {
    let mut connection = pool.get().unwrap();
    let items = register.load::<models::Register>(&mut connection)?;
    Ok(items)
} 