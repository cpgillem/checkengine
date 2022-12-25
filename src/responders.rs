use actix_web::Responder;
use serde::{Serialize, Deserialize};
use crate::db_access;
use super::DbPool;
use actix_web::{web, Error, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRegister {
    pub title: String,
}

pub async fn get_registers(db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match db_access::get_all_registers(db) {
        Ok(registers) => Ok(HttpResponse::Ok().json(registers)),
        _ => Ok(HttpResponse::InternalServerError().body("fail")),
    }
}

pub async fn get_register() -> impl Responder {
    format!("one responder by ID")
}

pub async fn add_register() -> impl Responder {
    format!("add register")
}

pub async fn delete_register() ->  impl Responder {
    format!("delete register")
}