use crate::auth::{Login, Signup};
use crate::controllers::member_controller::MemberController;
use crate::auth;
use actix_web::{web, Error, HttpResponse, post};
use actix_web::error;

// Sends the login payload to the server and returns a JWT.
#[post("login")]
pub async fn authenticate(controller: web::Data<MemberController>, body: web::Json<Login>) -> Result<HttpResponse, Error> {
    // Get user by username.
    let user = controller.get(&body.username)
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
pub async fn signup(controller: web::Data<MemberController>, body: web::Json<Signup>) -> Result<HttpResponse, Error> {
    // Extract signup data.
    let signup = body.0;

    // Insert the new member.
    let inserted_member = controller.create(&signup).map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(inserted_member))
}

