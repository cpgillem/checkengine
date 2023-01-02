use crate::models::register::{Register, NewRegister};
use crate::schema::register::dsl;
use crate::models::register::InputRegister;
use crate::DbPool;
use actix_web::error;
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};
use diesel::prelude::*;
use diesel::dsl::now;

#[get("")]
pub async fn get_registers(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;
    let registers = dsl::register.load::<Register>(&mut connection).map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(registers))
}

#[get("{id}")]
pub async fn get_register(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;
    let register = dsl::register.find(id).first::<Register>(&mut connection).map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(register))
}

#[post("")]
pub async fn add_register(pool: web::Data<DbPool>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;
    let input_register = input.0;
    let new_register = NewRegister::from_input(&input_register);
    let inserted_register = diesel::insert_into(dsl::register).values(&new_register).get_result::<Register>(&mut connection).map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(inserted_register))
}

#[delete("{id}")]
pub async fn delete_register(pool: web::Data<DbPool>, path: web::Path<i32>) ->  Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;
    diesel::delete(dsl::register.find(id)).execute(&mut connection).map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().body("deleted"))
}

#[patch("{id}")]
pub async fn update_register(pool: web::Data<DbPool>, path: web::Path<i32>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;
    let id = path.into_inner();
    let input_register = input.0;
    diesel::update(dsl::register.find(id))
        .set((
            dsl::title.eq(input_register.title),
            dsl::modified_at.eq(now),
        ))
        .execute(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().body("updated"))
}