use crate::{db_access, models::register::InputRegister};
use crate::DbPool;
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};

#[get("")]
pub async fn get_registers(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut db = pool.get().unwrap();
    match db_access::get_all_registers(&mut db) {
        Ok(registers) => Ok(HttpResponse::Ok().json(registers)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

#[get("{id}")]
pub async fn get_register(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut db = pool.get().unwrap();
    match db_access::get_register(&mut db, id) {
        Ok(register) => Ok(HttpResponse::Ok().json(register)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

#[post("")]
pub async fn add_register(pool: web::Data<DbPool>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    let mut db = pool.get().unwrap();
    let input_register = input.0;
    match db_access::create_register(&mut db, input_register) {
        Ok(register) => Ok(HttpResponse::Ok().json(register)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

#[delete("{id}")]
pub async fn delete_register(pool: web::Data<DbPool>, path: web::Path<i32>) ->  Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut db = pool.get().unwrap();
    match db_access::delete_register(&mut db, id) {
        Ok(_) => Ok(HttpResponse::Ok().body("deleted")),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

#[patch("{id}")]
pub async fn update_register(pool: web::Data<DbPool>, path: web::Path<i32>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    let mut db = pool.get().unwrap();
    let id = path.into_inner();
    let input_register = input.0;
    match db_access::update_register(&mut db, id, input_register) {
        Ok(_) => Ok(HttpResponse::Ok().body("success")),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}