use crate::{
    models::{register::{Register, NewRegister, InputRegister, UpdateRegister}},
    schema::register,
    DbPool, auth::JwtClaims,
};

use super::{DataError, ResourceController, GetResource, CreateResource, DeleteResource, UpdateResource, Controller};

use diesel::{RunQueryDsl, dsl::now};
use diesel::prelude::*;

/// Stores state information and provides access to registers.
#[derive(Clone)]
pub struct RegisterController {
    pub pool: DbPool,
    pub jwt: JwtClaims,
}

impl Controller for RegisterController {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl ResourceController for RegisterController {
    fn new(pool: &DbPool, jwt: &JwtClaims) -> Self {
        Self {
            pool: pool.clone(),
            jwt: jwt.clone(),
        }
    }

    fn get_member_id(&self) -> i32 {
        self.jwt.sub
    }
}

impl GetResource<Register> for RegisterController {
    fn get(&self, id: i32) -> Result<Register, DataError> {
        let mut connection = self.get_connection()?;

        // Get the register.
        let register = register::table
            .filter(register::id.eq(id))
            .first::<Register>(&mut connection)
            .map_err(|_| DataError::NotFound)?;
        
        // It's a different problem if it isn't owned.
        if register.member_id != self.get_member_id() {
            return Err(DataError::NotOwned);
        }

        Ok(register)
    }

    /// Retrieves all registers.
    fn get_all(&self) -> Result<Vec<Register>, DataError> {
        let mut connection = self.get_connection()?;
        Ok(
            register::table
                .filter(register::member_id.eq(self.get_member_id()))
                .load::<Register>(&mut connection)
                .map_err(|_| DataError::Unspecified)?
        )
    }
}

impl CreateResource<InputRegister, Register> for RegisterController {
    /// Creates a new register in the database from input.
    fn create(&self, input: &InputRegister) -> Result<Register, DataError> {
        let mut connection = self.get_connection()?;
        let new_register = NewRegister::from_input(&input, self.get_member_id());
        diesel::insert_into(register::table)
            .values(&new_register)
            .get_result::<Register>(&mut connection)
            .map_err(|_| DataError::NotInserted)
    }
}

impl DeleteResource for RegisterController {
    /// Deletes a register, if allowed. Only actually return a 404 even if it is found but not authorized.
    fn delete(&self, id: i32) -> Result<usize, DataError> {
        let mut connection = self.get_connection()?;

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
        let mut connection = self.get_connection()?;

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