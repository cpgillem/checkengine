use crate::{
    models::{register::{Register, NewRegister, InputRegister, UpdateRegister}},
    schema::register,
    DbPool, auth::JwtClaims,
};

use super::{get_connection, DataError, Resource, GetResource, CreateResource, DeleteResource, UpdateResource};

use diesel::{RunQueryDsl, dsl::now};
use diesel::prelude::*;

/// Stores state information and provides access to registers.
#[derive(Clone)]
pub struct RegisterController {
    pub pool: DbPool,
    pub jwt: JwtClaims,
}

impl Resource for RegisterController {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self {
        Self {
            pool: pool.clone(),
            jwt: jwt.clone(),
        }
    }
}

impl GetResource<Register> for RegisterController {
    fn get(&self, id: i32) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Get the register.
        let register = register::table
            .filter(register::id.eq(id))
            .first::<Register>(&mut connection)
            .map_err(|_| DataError::NotFound)?;
        
        // It's a different problem if it isn't owned.
        if register.member_id != self.jwt.sub {
            return Err(DataError::NotOwned);
        }

        Ok(register)
    }

    /// Retrieves all registers.
    fn get_all(&self) -> Result<Vec<Register>, DataError> {
        let mut connection = get_connection(&self.pool)?;
        Ok(
            register::table
                .filter(register::member_id.eq(self.jwt.sub))
                .load::<Register>(&mut connection)
                .map_err(|_| DataError::Unspecified)?
        )
    }
}

impl CreateResource<InputRegister, Register> for RegisterController {
    /// Creates a new register in the database from input.
    fn create(&self, input: &InputRegister) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;
        let new_register = NewRegister::from_input(&input, self.jwt.sub);
        diesel::insert_into(register::table)
            .values(&new_register)
            .get_result::<Register>(&mut connection)
            .map_err(|_| DataError::NotInserted)
    }
}

impl DeleteResource for RegisterController {
    /// Deletes a register, if allowed. Only actually return a 404 even if it is found but not authorized.
    fn delete(&self, id: i32) -> Result<usize, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Retrieve register, if owned, or in existence.
        self.get(id)?;
        
        // Delete the record.
        diesel::delete(
                register::table.find(id)
            )
            .execute(&mut connection)
            .map_err(|_| DataError::NotDeleted)
    }
}

impl UpdateResource<UpdateRegister, Register> for RegisterController {
    /// Updates a register if allowed.
    fn update(&self, id: i32, input: &UpdateRegister) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Retrieve the register, if owned, or in existence.
        self.get(id)?;

        // Make the update.
        let updated_register = diesel::update(
                register::table.find(id)
            )
            .set(input)
            .get_result::<Register>(&mut connection)
            .map_err(|_| DataError::NotUpdated)?;

        // Update metadata.
        diesel::update(register::table.find(id))
            .set(register::modified_at.eq(now))
            .execute(&mut connection)
            .map_err(|_| DataError::Unspecified)?;

        Ok(updated_register)
    }
}