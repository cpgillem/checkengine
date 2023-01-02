use crate::models::member::Member;
use crate::models::register::{Register, NewRegister};
use crate::schema::register::dsl;
use crate::models::register::InputRegister;
use crate::DbPool;
use actix_web::{HttpRequest, error};
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};
use diesel::prelude::*;
use diesel::dsl::now;

#[get("")]
pub async fn get_registers(pool: web::Data<DbPool>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    // Extract the logged in user.
    let member = Member::from_header(&request, &pool)
        .map_err(|e| error::ErrorUnauthorized(e))?;

    // Find the user's registers.
    let registers = Register::belonging_to(&member)
        .load::<Register>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;
    
    Ok(HttpResponse::Ok().json(registers))
}

#[get("{id}")]
pub async fn get_register(pool: web::Data<DbPool>, path: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    // Extract logged in user.
    let member = Member::from_header(&request, &pool)
        .map_err(|e| error::ErrorUnauthorized(e))?;

    // Create connection.
    let mut connection = pool.get()
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Find the register, as long as it belongs to the user.
    let register = dsl::register
        .filter(dsl::id.eq(id))
        .filter(dsl::member_id.eq(member.id))
        .first::<Register>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(register))
}

#[post("")]
pub async fn add_register(pool: web::Data<DbPool>, input: web::Json<InputRegister>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    // Extract the logged in user.
    let member = Member::from_header(&request, &pool)
        .map_err(|e| error::ErrorUnauthorized(e))?;
    
    // Extract the input.
    let input_register = input.0;

    // Create a new register.
    let new_register = NewRegister::from_input(&input_register, &member);
    let inserted_register = diesel::insert_into(dsl::register)
        .values(&new_register)
        .get_result::<Register>(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(inserted_register))
}

#[delete("{id}")]
pub async fn delete_register(pool: web::Data<DbPool>, path: web::Path<i32>, request: actix_web::HttpRequest) ->  Result<HttpResponse, Error> {
    let id = path.into_inner();

    // Extract the logged in user.
    let member = Member::from_header(&request, &pool)
        .map_err(|e| error::ErrorUnauthorized(e))?;

    let mut connection = pool.get()
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Delete the register.
    diesel::delete(dsl::register.filter(dsl::member_id.eq(member.id)).find(id))
        .execute(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

#[patch("{id}")]
pub async fn update_register(pool: web::Data<DbPool>, path: web::Path<i32>, input: web::Json<InputRegister>,  request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    // Extract the logged in user.
    let member = Member::from_header(&request, &pool)
        .map_err(|e| error::ErrorUnauthorized(e))?;

    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let input_register = input.0;
    
    diesel::update(dsl::register.filter(dsl::member_id.eq(member.id)).find(id))
        .set((
            dsl::title.eq(input_register.title),
            dsl::modified_at.eq(now),
        ))
        .execute(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("updated"))
}