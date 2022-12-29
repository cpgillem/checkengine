use crate::{db_access, models::{register::InputRegister, member::InputMember}};
use checkengine::DbPool;
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};

// Sends the login payload to the server and returns a JWT.
#[post("")]
pub async fn authenticate(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

// Creates a user in the database.
#[post("")]
pub async fn create_member(pool: web::Data<DbPool>, input: web::Json<InputMember>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[get("{id}")]
pub async fn get_member(pool: web::Data<DbPool>, input: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[delete("{id}")]
pub async fn delete_member(pool: web::Data<DbPool>, path: web::Path<i32>) ->  Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[patch("{id}")]
pub async fn update_member(pool: web::Data<DbPool>, path: web::Path<i32>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}