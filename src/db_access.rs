use diesel::prelude::*;

use crate::DbConnection;
use crate::models::register::*;
use crate::schema::register as repo;

pub fn get_all_registers(connection: &mut DbConnection) -> Result<Vec<Register>, diesel::result::Error> {
    repo::table.load::<Register>(connection)
}

pub fn get_register(connection: &mut DbConnection, id: i32) -> Result<Register, diesel::result::Error> {
    repo::table.find(id).first(connection)
}

pub fn create_register(connection: &mut DbConnection, input_register: InputRegister) -> Result<Register, diesel::result::Error> {
    let new_register = NewRegister::from_input(&input_register);
    
    diesel::insert_into(repo::table)
        .values(&new_register)
        .get_result(connection)
}

pub fn delete_register(connection: &mut DbConnection, id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(repo::table.find(id)).execute(connection)
}