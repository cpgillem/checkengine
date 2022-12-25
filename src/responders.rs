use actix_web::Responder;
use crate::{db_access, models::register::InputRegister};
use super::DbPool;
use actix_web::{web, Error, HttpResponse};

pub async fn get_registers(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut db = pool.get().unwrap();
    match db_access::get_all_registers(&mut db) {
        Ok(registers) => Ok(HttpResponse::Ok().json(registers)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

pub async fn get_register() -> impl Responder {
    format!("one responder by ID")
}

pub async fn add_register(pool: web::Data<DbPool>, input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    let mut db = pool.get().unwrap();
    let input_register = input.0;
    match db_access::create_register(&mut db, input_register) {
        Ok(register) => Ok(HttpResponse::Ok().json(register)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

pub async fn delete_register() ->  impl Responder {
    format!("delete register")
}