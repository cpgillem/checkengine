use diesel::prelude::*;
use diesel::dsl::now;

use crate::DbConnection;
use crate::models::member::Member;
use crate::models::register::*;
use crate::schema::member;
use crate::schema::register::{modified_at, title, self};

pub fn get_all_registers(connection: &mut DbConnection) -> Result<Vec<Register>, diesel::result::Error> {
    register::table.load::<Register>(connection)
}

pub fn get_register(connection: &mut DbConnection, id: i32) -> Result<Register, diesel::result::Error> {
    register::table.find(id).first(connection)
}

pub fn get_member(connection: &mut DbConnection, username: &str) -> Result<Member, diesel::result::Error> {
    member::table.filter(member::columns::username.eq(username)).get_result::<Member>(connection)
}

pub fn create_register(connection: &mut DbConnection, input_register: InputRegister) -> Result<Register, diesel::result::Error> {
    let new_register = NewRegister::from_input(&input_register);
    
    diesel::insert_into(register::table)
        .values(&new_register)
        .get_result(connection)
}

pub fn delete_register(connection: &mut DbConnection, id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(register::table.find(id)).execute(connection)
}

pub fn update_register(connection: &mut DbConnection, id: i32, input_register: InputRegister) -> Result<usize, diesel::result::Error> {
    diesel::update(register::table.find(id))
        .set((
            title.eq(input_register.title),
            modified_at.eq(now),
        ))
        .execute(connection)
}