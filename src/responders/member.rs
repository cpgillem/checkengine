use crate::{models::{register::{InputRegister}, member::{InputMember, Member}}, schema::member};
use crate::DbPool;
use actix_web::{web, Error, HttpResponse, get, post, delete, patch, error};
use diesel::{QueryDsl, RunQueryDsl};

// Creates a user in the database.
#[post("")]
pub async fn create_member(_pool: web::Data<DbPool>, _input: web::Json<InputMember>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[get("{id}")]
pub async fn get_member_by_id(pool: web::Data<DbPool>, input: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = input.into_inner();

    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let member = member::table.find(id).first::<Member>(&mut connection).map_err(|e| error::ErrorNotFound(e))?;
    Ok(HttpResponse::Ok().json(member))
}

#[delete("{id}")]
pub async fn delete_member(_pool: web::Data<DbPool>, _path: web::Path<i32>) ->  Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[patch("{id}")]
pub async fn update_member(_pool: web::Data<DbPool>, _path: web::Path<i32>, _input: web::Json<InputRegister>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}