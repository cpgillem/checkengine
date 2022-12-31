use crate::{models::member::Login, db_access};
use crate::{DbPool, auth};
use actix_web::{web, Error, HttpResponse, post};

// Sends the login payload to the server and returns a JWT.
#[post("authenticate")]
pub async fn authenticate(pool: web::Data<DbPool>, body: web::Json<Login>) -> Result<HttpResponse, Error> {
    let mut db = match pool.get() {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("could not connect to database")),
    };

    // Get user by username.
    let user = match db_access::get_member(&mut db, &body.username) {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("could not find user")),
    };

    // Check raw password.
    let check_password_result = match auth::check_password(&body.password_raw, &user.password_hash, &user.salt) {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("could not check password")),
    };

    // If the password mismatched, return message.
    if !check_password_result {
        return Ok(HttpResponse::Ok().body("password mismatch"));
    }

    // Create a JWT and send it.
    match user.get_jwt() {
        Ok(v) => Ok(HttpResponse::Ok().body(v)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("could not create JWT")),
    }
}