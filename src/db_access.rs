use diesel::prelude::*;
use chrono::prelude::*;

use crate::DbConnection;
use crate::models::register::*;
use crate::schema::register as repo;

pub fn get_all_registers(connection: &mut DbConnection) -> Result<Vec<Register>, diesel::result::Error> {
    let items = repo::table.load::<Register>(connection)?;
    Ok(items)
} 

pub fn create_register(connection: &mut DbConnection, input_register: InputRegister) -> Result<Register, diesel::result::Error> {
    let new_register = NewRegister {
        title: &input_register.title,
        created_at: Utc::now().naive_utc(),
        modified_at: Utc::now().naive_utc(),
    };
    
    diesel::insert_into(repo::table)
        .values(&new_register)
        .get_result(connection)
}