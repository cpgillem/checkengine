use crate::{models::member::Login};
use crate::{DbPool, auth};
use actix_web::{web, Error, HttpResponse, post};
use actix_web::error;
use crate::schema::member;
use diesel::prelude::*;
use crate::models::member::{Member, Signup, NewMember};
use crate::schema::member::dsl;

// Sends the login payload to the server and returns a JWT.
#[post("login")]
pub async fn authenticate(pool: web::Data<DbPool>, body: web::Json<Login>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get()
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Get user by username.
    let user = member::table
        .filter(member::columns::username.eq(&body.username))
        .get_result::<Member>(&mut connection)
        .map_err(|e| error::ErrorNotFound(e))?;

    // Check raw password. This will return an error if the password mismatches, otherwise continue with the function.
    auth::check_password(&body.password_raw, &user.password_hash, &user.salt)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Create a JWT and send it.
    let jwt = user.get_jwt()
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(jwt))
}

#[post("signup")]
pub async fn signup(pool: web::Data<DbPool>, body: web::Json<Signup>) -> Result<HttpResponse, Error> {
    // Create connection
    let mut connection = pool.get()
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Extract signup data.
    let signup = body.0;

    // Create a new member object.
    let new_member = NewMember::from_signup(&signup)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // Insert the new member.
    let inserted_member = diesel::insert_into(dsl::member)
        .values(&new_member)
        .get_result::<Member>(&mut connection)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(inserted_member))
}

