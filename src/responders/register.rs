use crate::controllers::register_controller::RegisterController;
use crate::responders::get_member;
use crate::models::register::InputRegister;
use actix_web::{HttpRequest, error};
use actix_web::{web, Error, HttpResponse, get, post, delete, patch};

#[get("")]
pub async fn get_registers(controller: web::Data<RegisterController>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    // Extract the logged in user.
    let member = get_member(&request, &controller.pool)?;

    // Find the user's registers.
    let registers = controller.get_all(&member)
        .map_err(|e| error::ErrorNotFound(e))?;
    
    Ok(HttpResponse::Ok().json(registers))
}

#[get("{id}")]
pub async fn get_register(controller: web::Data<RegisterController>, id: web::Path<i32>, request: HttpRequest) -> Result<HttpResponse, Error> {
    // Extract the logged in user.
    let member = get_member(&request, &controller.pool)?;

    // Retrieve the register.
    let register = controller.get(id.into_inner(), &member)
        .map_err(|e| error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(register))
}

#[post("")]
pub async fn add_register(controller: web::Data<RegisterController>, input: web::Json<InputRegister>, request: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    // Retrieve user.
    let member = get_member(&request, &controller.pool)?;

    // Create a new register.
    let inserted_register = controller.create(&input, &member)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(inserted_register))
}

#[delete("{id}")]
pub async fn delete_register(controller: web::Data<RegisterController>, id: web::Path<i32>, request: actix_web::HttpRequest) ->  Result<HttpResponse, Error> {
    // Extract the logged in user.
    let member = get_member(&request, &controller.pool)?;

    // Delete the register.
    controller.delete(id.into_inner(), &member)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("deleted"))
}

#[patch("{id}")]
pub async fn update_register(
    controller: web::Data<RegisterController>, 
    id: web::Path<i32>, 
    input: web::Json<InputRegister>,  
    request: actix_web::HttpRequest
) -> Result<HttpResponse, Error> {

    // Extract the logged in user.
    let member = get_member(&request, &controller.pool)?;
    
    let updated_register = controller.update(id.into_inner(), &input, &member)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(updated_register))
}