use crate::{models::member::Login};
use crate::{DbPool, auth};
use actix_web::{web, Error, HttpResponse, post};
use actix_web::error;
use crate::schema::member;
use diesel::prelude::*;
use crate::models::member::Member;

// Sends the login payload to the server and returns a JWT.
#[post("authenticate")]
pub async fn authenticate(pool: web::Data<DbPool>, body: web::Json<Login>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    // Get user by username.
    let user = member::table.filter(member::columns::username.eq(&body.username)).get_result::<Member>(&mut connection).map_err(|e| error::ErrorNotFound(e))?;

    // Check raw password. This will return an error if the password mismatches, otherwise continue with the function.
    auth::check_password(&body.password_raw, &user.password_hash, &user.salt).map_err(|e| error::ErrorInternalServerError(e))?;

    // Create a JWT and send it.
    let jwt = user.get_jwt().map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().body(jwt))
}