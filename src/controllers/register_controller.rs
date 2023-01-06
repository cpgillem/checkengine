use crate::{
    models::{register::{Register, NewRegister, InputRegister, UpdateRegister}},
    schema::register,
    DbPool, auth::JwtClaims,
};

use super::{get_connection, DataError};

use diesel::{RunQueryDsl, dsl::now};
use diesel::prelude::*;

/// Stores state information and provides access to registers.
#[derive(Clone)]
pub struct RegisterController {
    pub pool: DbPool,
}

impl RegisterController {
    /// Creates a new register in the database from input.
    pub fn create(&self, input: &InputRegister, jwt: &JwtClaims) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;
        let new_register = NewRegister::from_input(&input, jwt.sub);
        diesel::insert_into(register::table)
            .values(&new_register)
            .get_result::<Register>(&mut connection)
            .map_err(|_| DataError::NotInserted)
    }

    /// Retrieves all registers.
    pub fn get_all(&self, jwt: &JwtClaims) -> Result<Vec<Register>, DataError> {
        let mut connection = get_connection(&self.pool)?;
        Ok(
            register::table
                .filter(register::member_id.eq(jwt.sub))
                .load::<Register>(&mut connection)
                .map_err(|_| DataError::Unspecified)?
        )
    }

    /// Retrieves one register.
    pub fn get(&self, id: i32, jwt: &JwtClaims) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Get the register.
        let register = register::table
            .filter(register::id.eq(id))
            .first::<Register>(&mut connection)
            .map_err(|_| DataError::NotFound)?;
        
        // It's a different problem if it isn't owned.
        if register.member_id != jwt.sub {
            return Err(DataError::NotOwned);
        }

        Ok(register)
    }

    /// Deletes a register, if allowed. Only actually return a 404 even if it is found but not authorized.
    pub fn delete(&self, id: i32, jwt: &JwtClaims) -> Result<usize, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Retrieve register, if owned, or in existence.
        self.get(id, jwt)?;
        
        // Delete the record.
        diesel::delete(
                register::table.find(id)
            )
            .execute(&mut connection)
            .map_err(|_| DataError::NotDeleted)
    }

    /// Updates a register if allowed.
    pub fn update(&self, id: i32, input: &UpdateRegister, jwt: &JwtClaims) -> Result<Register, DataError> {
        let mut connection = get_connection(&self.pool)?;

        // Retrieve the register, if owned, or in existence.
        self.get(id, jwt)?;

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
